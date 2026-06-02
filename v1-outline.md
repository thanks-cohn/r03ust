```
R03UST/

  README.md                    (project overview, philosophy, quick start, and first confession)
  LICENSE                      (usage rights and legal permissions)
  CARGO.TOML                   (rust dependencies, binaries, and workspace definition)
  RUST-TOOLCHAIN.TOML          (pinned rust toolchain for repeatable builds)
  .GITIGNORE                   (generated files excluded from version control)

  DOCS/

    R03BUST-STANDARD.md        (the official run resurrection standard)
    R03BUST-METHOD.md          (remove, recreate, revive)
    R03BUST-PYTHON.md          (python environment integration)
    R03BUST-RUST.md            (cargo project integration)
    R03BUST-ZIG.md             (zig project integration)
    LEDGER-FORMAT.md           (append-only run history specification)
    RECEIPT-FORMAT.md          (single successful run record schema)
    REVIVE-FLOW.md             (how projects return to working state)
    DRIFT-DETECTION.md         (finding what changed between successes)
    REPO-SHAPE.md              (outline-legible engineering principles)
    GLOSSARY.md                (mystery sludge, cardboard code, repo shape, etc.)

  EXAMPLES/

    PYTHON-BASIC/

      README.md                (walkthrough of a python project using r03bust)
      REQUIREMENTS.TXT         (example dependency set)
      PYPROJECT.TOML           (example project metadata)
      SRC/
      TESTS/
      .R03BUST/

    RUST-BASIC/

      README.md                (walkthrough of a cargo project using r03bust)
      CARGO.TOML
      CARGO.LOCK
      SRC/
      TESTS/
      .R03BUST/

    ZIG-BASIC/

      README.md                (walkthrough of a zig project using r03bust)
      BUILD.ZIG
      BUILD.ZIG.ZON
      SRC/
      TESTS/
      .R03BUST/

    POLYGLOT-BASIC/

      README.md                (mixed rust, zig, and python example)
      RUST/
      PYTHON/
      ZIG/

  SRC/

    MAIN.RS                    (cli entrypoint and startup flow)
    CLI.RS                     (command parsing and routing)
    CONFIG.RS                  (configuration loading and defaults)
    ERROR.RS                   (shared errors and exit codes)

    COMMANDS/

      INIT.RS                  (create project ledger structure)
      RUN.RS                   (execute command and capture receipt)
      LAST.RS                  (show latest successful run)
      LEDGER.RS                (browse and query run history)
      STATUS.RS                (summarize current project health)
      DIFF.RS                  (compare current state against last success)
      DOCTOR.RS                (diagnose drift and missing requirements)
      REVIVE.RS                (generate restoration strategy)
      SEAL.RS                  (mark trusted milestones)
      EXPLAIN.RS               (generate human-readable reports)
      EXPORT.RS                (export ledger data)
      SEARCH.RS                (search historical runs)
      TIMELINE.RS              (view project evolution through time)

    CORE/

      PROJECT.RS               (project discovery and identity)
      RUN_RECEIPT.RS           (single recorded run structure)
      LEDGER_WRITER.RS         (append new records to ledger)
      LEDGER_READER.RS         (load and query historical records)
      SUCCESS_STORE.RS         (manage known-good runs)
      SNAPSHOT.RS              (capture project and machine state)
      ARTIFACT.RS              (track generated outputs)
      REVIVE_PLAN.RS           (generate restoration plans)
      SESSION.RS               (group related runs into sessions)
      RUN_ID.RS                (stable run identifiers)
      HISTORY.RS               (long-term project memory)

    CAPTURE/

      MACHINE.RS               (hostname, architecture, machine identity)
      OS.RS                    (distribution, kernel, platform details)
      CPU.RS                   (processor information)
      MEMORY.RS                (ram information)
      DISK.RS                  (filesystem and storage state)
      ENV.RS                   (safe environment variables)
      GIT.RS                   (commit, branch, dirty state)
      PATH.RS                  (path layout and hashing)
      PROCESS.RS               (exit code, duration, logs)
      DEPENDENCIES.RS          (installed dependency state)
      ARTIFACTS.RS             (generated outputs and hashes)
      SERVICES.RS              (running services and daemons)
      NETWORK.RS               (network environment details)
      PACKAGES.RS              (system package inventory)
      TOOLCHAINS.RS            (compiler and runtime versions)

    ADAPTERS/

      PYTHON.RS                (venv, pip, pyproject, pytest support)
      RUST.RS                  (cargo, rustc, cargo.lock support)
      ZIG.RS                   (build.zig and zig cache support)
      NODE.RS                  (node and package manager support)
      SHELL.RS                 (shell project support)
      GO.RS                    (go module support)
      CMAKE.RS                 (cmake project support)
      MAKE.RS                  (makefile project support)

    COMPARE/

      DRIFT.RS                 (overall drift engine)
      MACHINE_DIFF.RS          (machine comparison)
      ENV_DIFF.RS              (environment comparison)
      DEPENDENCY_DIFF.RS       (dependency comparison)
      ARTIFACT_DIFF.RS         (artifact comparison)
      TOOLCHAIN_DIFF.RS        (compiler comparison)
      PACKAGE_DIFF.RS          (system package comparison)
      CONFIG_DIFF.RS           (configuration comparison)

    OUTPUT/

      HUMAN.RS                 (terminal-friendly output)
      JSON.RS                  (machine-readable output)
      TOML.RS                  (configuration-style output)
      MARKDOWN.RS              (documentation reports)
      REPORT.RS                (long-form diagnostics)
      HTML.RS                  (shareable project reports)

    STORAGE/

      LEDGER_DB.RS             (ledger persistence layer)
      INDEX.RS                 (fast run lookup indexes)
      CACHE.RS                 (temporary cached data)
      ARCHIVE.RS               (old run archival support)

    UTIL/

      HASH.RS                  (hashing helpers)
      TIME.RS                  (timestamps and durations)
      FS.RS                    (filesystem helpers)
      COMMAND.RS               (process launching helpers)
      SANITIZE.RS              (secret redaction)
      FORMAT.RS                (shared formatting helpers)

  TEMPLATES/

    R03BUST/

      CONFIG.TOML             (default project configuration)
      REVIVE.SH               (generated revival script)
      README.R03BUST.MD       (explains project ledger folder)

    PYTHON/

      CONFIG.TOML             (python capture defaults)

    RUST/

      CONFIG.TOML             (cargo capture defaults)

    ZIG/

      CONFIG.TOML             (zig capture defaults)

  TESTS/

    INIT_CREATES_SHAPE.RS                     (verify project structure creation)
    RUN_WRITES_LEDGER.RS                      (verify ledger recording)
    SUCCESS_PROMOTES_RECEIPT.RS               (verify successful run promotion)
    DOCTOR_DETECTS_DRIFT.RS                   (verify drift detection)
    PYTHON_ADAPTER_DETECTS_VENV.RS            (verify python integration)
    RUST_ADAPTER_DETECTS_CARGO_LOCK.RS        (verify cargo integration)
    ZIG_ADAPTER_DETECTS_BUILD_ZIG.RS          (verify zig integration)
    REVIVE_GENERATES_PLAN.RS                  (verify restoration plans)

  SCRIPTS/

    DEV-BUILD.SH              (developer build workflow)
    DEV-TEST.SH               (developer test workflow)
    DEMO-PYTHON.SH            (python demonstration)
    DEMO-RUST.SH              (rust demonstration)
    DEMO-ZIG.SH               (zig demonstration)
    CLEAN-DEMO.SH             (remove generated artifacts)

  BENCHMARKS/

    COLD-START/               (fresh machine measurements)
    WARM-START/               (cached environment measurements)
    LARGE-REPO/               (folder with teeth testing)
    MILLION-FILE/             (scale and abuse testing)

  FUTURE/

    AGENTS/                   (autonomous revive assistants)
    DISTRIBUTED/              (cross-machine run resurrection)
    TEAM-LEDGERS/             (shared organizational history)
    FORENSICS/                (failure investigation tooling)



```

