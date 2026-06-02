#[derive(Debug, Clone)]
pub struct RunReceipt {
    pub schema_version: u32,
    pub timestamp: String,
    pub cwd: String,
    pub command: String,
    pub argv: Vec<String>,
    pub exit_code: i32,
    pub success: bool,
    pub duration_ms: u128,
    pub stdout_log: String,
    pub stderr_log: String,
    pub os: String,
    pub arch: String,
    pub hostname: Option<String>,
    pub git_branch: Option<String>,
    pub git_commit: Option<String>,
    pub git_dirty: Option<bool>,
}

impl RunReceipt {
    pub fn to_json_line(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"timestamp\":{},\"cwd\":{},\"command\":{},\"argv\":[{}],\"exit_code\":{},\"success\":{},\"duration_ms\":{},\"stdout_log\":{},\"stderr_log\":{},\"os\":{},\"arch\":{},\"hostname\":{},\"git_branch\":{},\"git_commit\":{},\"git_dirty\":{}}}",
            self.schema_version,
            json_string(&self.timestamp),
            json_string(&self.cwd),
            json_string(&self.command),
            self.argv
                .iter()
                .map(|arg| json_string(arg))
                .collect::<Vec<_>>()
                .join(","),
            self.exit_code,
            self.success,
            self.duration_ms,
            json_string(&self.stdout_log),
            json_string(&self.stderr_log),
            json_string(&self.os),
            json_string(&self.arch),
            json_option_string(&self.hostname),
            json_option_string(&self.git_branch),
            json_option_string(&self.git_commit),
            json_option_bool(self.git_dirty),
        )
    }

    pub fn from_json_line(line: &str) -> std::result::Result<Self, String> {
        let trimmed = line.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err("line is not a JSON object".to_string());
        }

        Ok(Self {
            schema_version: number_field(trimmed, "schema_version")?
                .try_into()
                .map_err(|_| "schema_version is out of range".to_string())?,
            timestamp: string_field(trimmed, "timestamp")?,
            cwd: string_field(trimmed, "cwd")?,
            command: string_field(trimmed, "command")?,
            argv: array_string_field(trimmed, "argv")?,
            exit_code: number_field(trimmed, "exit_code")?
                .try_into()
                .map_err(|_| "exit_code is out of range".to_string())?,
            success: bool_field(trimmed, "success")?,
            duration_ms: number_field(trimmed, "duration_ms")?,
            stdout_log: string_field(trimmed, "stdout_log")?,
            stderr_log: string_field(trimmed, "stderr_log")?,
            os: string_field(trimmed, "os")?,
            arch: string_field(trimmed, "arch")?,
            hostname: option_string_field(trimmed, "hostname")?,
            git_branch: option_string_field(trimmed, "git_branch")?,
            git_commit: option_string_field(trimmed, "git_commit")?,
            git_dirty: option_bool_field(trimmed, "git_dirty")?,
        })
    }

    pub fn compact_line(&self) -> String {
        format!(
            "{} | success={} | exit={} | {} | stdout={} | stderr={}",
            self.timestamp,
            self.success,
            self.exit_code,
            self.command,
            self.stdout_log,
            self.stderr_log
        )
    }
}

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                output.push_str(&format!("\\u{:04x}", character as u32))
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn json_option_string(value: &Option<String>) -> String {
    value
        .as_ref()
        .map_or_else(|| "null".to_string(), |value| json_string(value))
}

fn json_option_bool(value: Option<bool>) -> String {
    value.map_or_else(|| "null".to_string(), |value| value.to_string())
}

fn field_value<'a>(line: &'a str, field: &str) -> std::result::Result<&'a str, String> {
    let marker = format!("\"{field}\":");
    let start = line
        .find(&marker)
        .ok_or_else(|| format!("missing field `{field}`"))?
        + marker.len();
    Ok(&line[start..])
}

fn string_field(line: &str, field: &str) -> std::result::Result<String, String> {
    let value = field_value(line, field)?.trim_start();
    parse_string(value).map(|(value, _)| value)
}

fn option_string_field(line: &str, field: &str) -> std::result::Result<Option<String>, String> {
    let value = field_value(line, field)?.trim_start();
    if value.starts_with("null") {
        Ok(None)
    } else {
        parse_string(value).map(|(value, _)| Some(value))
    }
}

fn number_field(line: &str, field: &str) -> std::result::Result<u128, String> {
    let value = field_value(line, field)?.trim_start();
    let number: String = value
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    if number.is_empty() {
        return Err(format!("field `{field}` is not a number"));
    }
    number
        .parse()
        .map_err(|_| format!("field `{field}` is not a valid number"))
}

fn bool_field(line: &str, field: &str) -> std::result::Result<bool, String> {
    let value = field_value(line, field)?.trim_start();
    if value.starts_with("true") {
        Ok(true)
    } else if value.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field `{field}` is not a boolean"))
    }
}

fn option_bool_field(line: &str, field: &str) -> std::result::Result<Option<bool>, String> {
    let value = field_value(line, field)?.trim_start();
    if value.starts_with("null") {
        Ok(None)
    } else {
        bool_field(line, field).map(Some)
    }
}

fn array_string_field(line: &str, field: &str) -> std::result::Result<Vec<String>, String> {
    let value = field_value(line, field)?.trim_start();
    if !value.starts_with('[') {
        return Err(format!("field `{field}` is not an array"));
    }
    let mut rest = &value[1..];
    let mut items = Vec::new();
    loop {
        rest = rest.trim_start();
        if rest.starts_with(']') {
            return Ok(items);
        }
        let (item, next) = parse_string(rest)?;
        items.push(item);
        rest = next.trim_start();
        if rest.starts_with(',') {
            rest = &rest[1..];
        } else if rest.starts_with(']') {
            return Ok(items);
        } else {
            return Err(format!("field `{field}` has malformed array contents"));
        }
    }
}

fn parse_string(input: &str) -> std::result::Result<(String, &str), String> {
    let mut chars = input.char_indices();
    if chars.next().map(|(_, c)| c) != Some('"') {
        return Err("expected JSON string".to_string());
    }

    let mut output = String::new();
    let mut escaped = false;
    for (index, character) in chars {
        if escaped {
            match character {
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                other => output.push(other),
            }
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' {
            return Ok((output, &input[index + 1..]));
        } else {
            output.push(character);
        }
    }

    Err("unterminated JSON string".to_string())
}
