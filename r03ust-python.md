
# r03ust Python Example

## Real Software Should Remember Why It Worked

`r03ust` is a run resurrection standard.

It does not replace Python, `venv`, `pip`, `pytest`, `uv`, Poetry, Docker, Nix, or CI.

It watches them.

It records what happened when the project worked, what machine it worked on, what environment existed, what dependencies were present, what command succeeded, and what changed later.

The point is not magic reproducibility.

The point is removing mystery.

If a Python project worked once, the project should remember why.

---

## The r03ust Method

```text
1. REMOVE the mystery.
2. RECREATE the conditions.
3. REVIVE the project as if you had never left.
```

A Python project often dies from drift.

The virtual environment disappears. The Python version changes. `requirements.txt` gets edited. A system package vanishes. A path changes. The developer forgets the one command that actually worked.

Then the project becomes a haunted folder.

`r03ust` turns that haunted folder back into a machine.

---

## Example Python Project Shape

```text
my-python-app/
  README.md
  pyproject.toml
  requirements.txt
  src/
  tests/

  .r03ust/
    config.toml
    ledger.jsonl
    success/
      last-run.json
      last-test.json
      last-install.json
    logs/
      stdout/
      stderr/
    env/
      python.json
      pip-freeze.txt
      path.txt
      env.safe.json
    machine/
      system.json
    artifacts/
      hashes.json
    revive/
      revive.sh
      notes.txt
```

The visible Python project stays normal.

The `.r03ust/` directory becomes the project memory.

---

## Basic Workflow

```bash
cd my-python-app

r03ust init

python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

r03ust run python main.py
r03ust run pytest
r03ust run python -m build
```

Python creates the environment.

`pip` installs the dependencies.

`pytest` proves the project works.

`r03ust` remembers the conditions.

---

## What r03ust Records

Every successful run should leave a receipt.

For a Python command, `r03ust` should record at least:

```text
project name
project path
timestamp
command
exit code
duration
working directory
git commit
git branch
git dirty state
hostname
username
OS
kernel
CPU
architecture
RAM
Python version
pip version
virtual environment path
requirements.txt hash
pyproject.toml hash
pip freeze output
PATH hash
safe environment variables
stdout log path
stderr log path
artifact paths
artifact hashes
```

Optional records:

```text
uv version
Poetry version
Conda environment
system package hints
Docker image hash
Nix flake hash
database version
open ports
service status
benchmark result
manual notes
```

---

## Example Run Receipt

After this command:

```bash
r03ust run pytest
```

`r03ust` may write a ledger entry like this:

```json
{
  "project_name": "my-python-app",
  "project_path": "/home/big-bro/dev/my-python-app",
  "timestamp": "2026-06-01T18:20:00-05:00",
  "command": "pytest",
  "exit_code": 0,
  "success": true,
  "duration_ms": 1842,
  "git_commit": "abc123",
  "git_branch": "main",
  "git_dirty": false,
  "hostname": "a-bot",
  "os": "Garuda Linux",
  "kernel": "6.x",
  "architecture": "x86_64",
  "cpu": "Intel i7-1255U",
  "python_version": "3.12.3",
  "pip_version": "24.0",
  "venv_path": ".venv",
  "requirements_hash": "sha256:91ac...",
  "pyproject_hash": "sha256:44ff...",
  "pip_freeze_hash": "sha256:8ab1...",
  "path_hash": "sha256:e700...",
  "stdout": ".r03ust/logs/stdout/2026-06-01T18-20-00-pytest.log",
  "stderr": ".r03ust/logs/stderr/2026-06-01T18-20-00-pytest.log"
}
```

This is not just a log.

It is a resurrection receipt.

---

## Six Months Later

A developer returns to the project.

The project no longer works.

Instead of guessing, they run:

```bash
r03ust doctor
```

Example output:

```text
LAST SUCCESSFUL TEST
  command: pytest
  machine: a-bot
  python: 3.12.3
  pip: 24.0
  git commit: abc123
  requirements hash: sha256:91ac...

CURRENT STATE
  python: 3.13.1              CHANGED
  pip: 25.0                   CHANGED
  virtualenv: .venv           MISSING
  requirements hash:          CHANGED
  git commit:                 DIFFERENT
  kernel:                     CHANGED

LIKELY REVIVE PATH
  python -m venv .venv
  source .venv/bin/activate
  pip install -r requirements.txt
  r03ust run pytest
```

The developer now knows what changed.

The project has confessed.

---

## Core Commands

```text
r03ust init
```

Creates the `.r03ust/` directory.

```text
r03ust run <command>
```

Runs a command and records the full run receipt.

Examples:

```bash
r03ust run python main.py
r03ust run pytest
r03ust run python -m build
```

```text
r03ust last
```

Shows the last successful run.

```text
r03ust status
```

Shows whether the current project resembles the last successful state.

```text
r03ust diff
```

Compares current conditions against the last successful run.

```text
r03ust doctor
```

Explains what is missing, changed, suspicious, or likely broken.

```text
r03ust revive
```

Prints the fastest known path back to a working state.

```text
r03ust ledger
```

Shows the run history.

```text
r03ust seal
```

Marks a run as important, stable, or release-worthy.

---

## Background Mode

The simplest version of `r03ust` does not need a daemon.

The foundation is:

```bash
r03ust run <command>
```

That is enough to build the standard.

Later, background mode can exist:

```bash
r03ust watch
```

Or as a user service:

```bash
systemctl --user enable --now r03ust-watch@my-python-app
```

Background mode can watch for file changes, dependency changes, lockfile changes, and repeated command patterns.

But the daemon is optional.

The ledger is mandatory.

---

## What r03ust Is Not

`r03ust` is not a replacement for Python packaging.

It is not a replacement for virtual environments.

It is not a replacement for Docker.

It is not a replacement for Nix.

It is not a replacement for CI.

It is the truth layer above them.

Python manages Python.

Docker freezes containers.

Nix describes environments.

CI runs checks.

`r03ust` records what actually worked.

---

## The Mental Model

```text
Python creates the environment.
pip installs the dependencies.
pytest proves the project works.
r03ust remembers the conditions.
```

That is the standard.

Not manage everything.

Witness everything.

---

## The Promise

A forgotten Python project should not feel like a corpse.

It should feel like a machine under a tarp.

Pull the cover off.

Read the ledger.

Recreate the conditions.

Revive the run.

```text
REMOVE the mystery.
RECREATE the conditions.
REVIVE the project.
```

That is `r03ust`.

