# r03ust
(robust)

r03ust: THE RUN RESURRECTION STANDARD

REAL SOFTWARE SHOULD REMEMBER WHY IT WORKED.

r03ust is a project standard and command-line system for recording, explaining, and reviving successful software runs. Its purpose is simple: remove mystery from working software.

Most projects fail slowly because nobody remembers the exact conditions under which they last worked. The command ran once. The build passed once. The demo launched once. Then the machine changed, the dependencies shifted, the environment drifted, and the project became a corpse with no autopsy.

r03ust fixes that.

It creates a run ledger: a durable, inspectable record of what ran, where it ran, how it ran, what machine it ran on, what dependencies existed, what compiler was used, what environment variables mattered, what artifacts were produced, and what conditions surrounded success.

The standard is built around the r03ust Method:

1. REMOVE THE MYSTERY

Every successful run must leave evidence.

The project should record the command, timestamp, working directory, git commit, compiler version, dependency state, system info, CPU, kernel, OS, memory, target platform, important environment variables, output logs, exit code, duration, artifact hashes, and any warnings or unusual conditions.

A working project should never say:

“It worked before, but I don’t know why.”

It should say:

“It worked here, under these conditions, with this toolchain, from this commit, producing these artifacts.”

2. RECREATE THE CONDITIONS

The ledger is not just history. It is a reconstruction map.

r03ust should make it easy to compare the current machine against the last known successful run. It should identify changed dependencies, changed compilers, changed environment variables, missing tools, missing files, altered paths, different CPU architecture, different OS/kernel, changed Cargo.lock, changed config, and changed artifacts.

The goal is not perfect time travel.

The goal is to get close enough to truth that the developer can stop guessing.

3. REVIVE THE PROJECT

A project should be returnable.

After weeks, months, or years away, the developer should be able to enter the folder and run:

r03ust status
r03ust last
r03ust diff
r03ust doctor
r03ust revive

And quickly understand:

what last worked,
why it worked,
what changed,
what is missing,
what command to try first,
and what must be restored.

r03ust is not only reproducibility.

It is revivability.

THE STANDARD

A r03ust-compliant project contains a .r03ust directory.

Example shape:

.r03ust/
ledger.jsonl
success/
last-build.json
last-test.json
last-run.json
logs/
stdout/
stderr/
machine/
hostname.txt
cpu.txt
os.txt
kernel.txt
memory.txt
env/
safe-env.json
path-hash.txt
artifacts/
hashes.json
diffs/
config.toml

The ledger should be append-only by default. Every meaningful run adds a record. Successful runs are promoted into the success directory. Failed runs may be recorded separately, but the heart of the standard is the successful run receipt.

A run receipt should answer:

What command ran?
Where did it run?
When did it run?
Who ran it?
What machine ran it?
What compiler/toolchain ran it?
What dependencies existed?
What environment mattered?
What files or configs mattered?
What artifacts came out?
Did it succeed?
How long did it take?
What changed since last success?

CORE COMMANDS

r03ust init

Creates the .r03ust structure.

r03ust run <command>

Runs a command through r03ust and records the result.

Example:

r03ust run cargo build
r03ust run cargo test
r03ust run ./zig-out/bin/zag doctor

r03ust last

Shows the last known successful run.

r03ust status

Shows whether the current project resembles the last successful state.

r03ust diff

Compares the current machine, environment, dependency state, and project state against the last successful run.

r03ust doctor

Explains what is missing, changed, suspicious, or likely broken.

r03ust revive

Prints the fastest known path back to a working state.

r03ust ledger

Shows the run history.

r03ust seal

Marks a run as important, stable, or release-worthy.

r03ust explain

Turns the ledger into a human-readable restoration note.

WHAT r03ust RECORDS

Minimum required fields:

project_name
project_path
timestamp
command
exit_code
duration_ms
git_commit
git_branch
dirty_state
hostname
username
os_name
os_version
kernel
architecture
cpu_model
ram_total
rustc_version
cargo_version
target_triple
Cargo.lock hash
Cargo.toml hash
PATH hash
important environment variables
artifact paths
artifact hashes
stdout log path
stderr log path

Optional fields:

GPU
disk space
installed system packages
container image
Nix flake hash
Docker image hash
Zig version
Node version
Python version
database version
open ports
service status
benchmark result
cold-cache/warm-cache marker
notes

THE CULTURE

r03ust belongs to the same family as real software survives abuse, outline-legible engineering, repo shape, mystery sludge, and unearned complexity.

It exists because working software should not depend on memory, vibes, luck, or one developer’s fragile ritual.

The project should confess.

If it worked once, it should confess why.

If it broke later, it should confess what changed.

If it must be revived, it should leave a trail.

SLOGANS

REAL SOFTWARE LEAVES RECEIPTS.

IF IT WORKED ONCE, IT SHOULD CONFESS WHY.

NO MYSTERY BUILDS.

NO GHOST SUCCESS.

REVIVABLE RUNS OVER VAGUE REPRODUCIBILITY.

THE MACHINE REMEMBERS.

REMOVE. RECREATE. REVIVE.

WHY RUST

Rust is a strong language for r03ust because the tool itself must be fast, portable, dependable, and easy to ship as a single binary.

The standard is about trust, so the tool should feel trustworthy.

Rust gives r03ust:

fast startup,
safe filesystem handling,
strong parsing,
cross-platform potential,
single-binary distribution,
excellent CLI ergonomics,
good JSON/TOML support,
strong hashing libraries,
and natural integration with Cargo projects.

But r03ust should not only serve Rust.

Rust builds the tool.

The standard serves every project.

A C project can use it.
A Zig project can use it.
A Python project can use it.
A Node project can use it.
A shell-script project can use it.

r03ust is not a Rust-only build tool.

It is a run resurrection standard written in Rust.

THE PROMISE

A forgotten project should not feel like an abandoned ruin.

It should feel like a machine under a tarp.

Pull the cover off.
Read the ledger.
Recreate the conditions.
Revive the run.

That is r03ust.

REMOVE the mystery.
RECREATE the conditions.
REVIVE the project.
