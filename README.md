# r03ust

r03ust is a run receipt tool for remembering what actually happened when a command ran.

The long-term standard is about run resurrection: remove mystery, recreate conditions, and revive a project later. This repository is intentionally **not** the full standard yet. The current implementation is a smallest honest v0 with one job:

> create a truthful run receipt.

Git remembers what changed. r03ust remembers what worked.

## WORKING IN V0

The v0 CLI implements only the receipt path:

```bash
r03ust init
r03ust run -- <command...>
r03ust last
r03ust ledger
r03ust doctor
```

### `r03ust init`

Creates the minimal storage shape:

```text
.r03ust/
  ledger.jsonl
  logs/
```

The command is idempotent. Running it again does not erase existing ledger data.

### `r03ust run -- <command...>`

Runs the real command after `--`, captures stdout and stderr into log files, and appends one JSON receipt to `.r03ust/ledger.jsonl`.

A failed wrapped command is still recorded honestly. The receipt records `success: false` and the real exit code, and the `r03ust run` process exits with that same code.

Each v0 receipt includes:

```text
schema_version
timestamp
cwd
command
argv
exit_code
success
duration_ms
stdout_log
stderr_log
os
arch
hostname
git_branch
git_commit
git_dirty
```

If git data is unavailable, the git fields are recorded as `null` instead of crashing.

### `r03ust last`

Reads the newest valid receipt and prints the command, result, duration, logs, and git state. If the ledger is empty, it says so clearly. If a ledger line is malformed, it reports the bad line number without panicking.

### `r03ust ledger`

Prints a compact receipt history and handles missing storage, empty ledgers, malformed lines, and mixed success/failure receipts.

### `r03ust doctor`

Checks whether the current directory can use r03ust storage. It reports:

```text
current working directory
.r03ust exists
ledger.jsonl exists
logs directory exists
ledger readable
logs writable
git available
inside git repo or not
OS
architecture
```

Doctor returns nonzero only when r03ust storage is missing or unusable.

## NOT YET IMPLEMENTED

v0 does **not** implement:

- revive
- diff
- seal
- explain
- status
- adapters
- dependency drift detection
- artifact comparison
- success promotion
- environment capture beyond basic machine/git data
- full project resurrection
- templates, examples, background watchers, or generated revive scripts

No output in v0 claims those features work.

## PLANNED LATER

Later versions may implement the broader r03ust standard:

- known-good success receipts
- project and dependency snapshots
- toolchain and environment comparison
- artifact hashing and comparison
- adapters for Python, Rust, Zig, Node, and other ecosystems
- drift diagnosis
- revive plans
- human-readable explain reports
- release sealing

Those features will be added only when they can record real behavior instead of placeholder state.

## Build and test

```bash
cargo build
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

## Example commands

```bash
cargo run -- init
cargo run -- run -- echo hello
cargo run -- last
cargo run -- ledger
cargo run -- doctor
cargo run -- run -- sh -c "echo bad >&2; exit 7"
cargo run -- last
```
