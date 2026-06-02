```
r03bust/
  README.md
  LICENSE
  Cargo.toml
  .gitignore

  docs/
    r03bust-standard.md
    r03bust-method.md
    r03ust-python.md
    repo-shape.md
    ledger-format.md
    revive-flow.md

  examples/
    python-app/
      README.md
      requirements.txt
      pyproject.toml
      src/
      tests/
      .r03bust-example/

    rust-app/
      README.md
      Cargo.toml
      src/
      tests/
      .r03bust-example/

  src/
    main.rs
    cli.rs
    config.rs
    error.rs

    commands/
      mod.rs
      init.rs
      run.rs
      last.rs
      ledger.rs
      status.rs
      diff.rs
      doctor.rs
      revive.rs
      seal.rs
      explain.rs

    core/
      mod.rs
      project.rs
      run_receipt.rs
      ledger_writer.rs
      success_store.rs
      snapshot.rs
      artifact.rs

    capture/
      mod.rs
      machine.rs
      os.rs
      cpu.rs
      memory.rs
      disk.rs
      env.rs
      git.rs
      path.rs
      process.rs
      dependencies.rs

    adapters/
      mod.rs
      python.rs
      rust.rs
      zig.rs
      node.rs
      shell.rs

    compare/
      mod.rs
      drift.rs
      machine_diff.rs
      env_diff.rs
      dependency_diff.rs
      artifact_diff.rs

    output/
      mod.rs
      human.rs
      json.rs
      toml.rs
      report.rs

    util/
      mod.rs
      hash.rs
      time.rs
      fs.rs
      command.rs
      sanitize.rs

  tests/
    init_creates_shape.rs
    run_writes_ledger.rs
    success_promotes_receipt.rs
    doctor_detects_drift.rs
    python_adapter_detects_venv.rs

  scripts/
    dev-build.sh
    dev-test.sh
    demo-python.sh
    clean-demo.sh

  templates/
    r03bust/
      config.toml
      README.r03bust.md
      revive.sh

  target/
    # ignored
```
