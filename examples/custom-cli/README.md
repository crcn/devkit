# Example Custom CLI

This example demonstrates how to build a custom development CLI using devkit as a library.

## What It Shows

1. **Using devkit extensions**: Docker operations via `devkit-ext-docker`
2. **Feature detection**: Auto-hides Docker commands if not detected
3. **Package commands**: Runs commands from `dev.toml` files
4. **Custom commands**: Add your own project-specific functionality
5. **Interactive menu**: User-friendly TUI with feature-based items

## Structure

```
custom-cli/
├── Cargo.toml          # Dependencies on devkit
├── src/
│   └── main.rs         # CLI implementation
└── README.md           # This file
```

## Build & Run

```bash
cd examples/custom-cli
cargo build
cargo run
```

Or run commands directly:
```bash
cargo run -- start
cargo run -- docker up
cargo run -- custom "Hello world"
```

## Key Features

### Auto-Detection
Commands automatically show/hide based on detection:
- No docker? Docker menu hidden
- No `dev.toml` commands? Package commands hidden

### Extension-Based
Uses devkit extensions for common functionality:
```rust
// Generic docker operations
devkit_ext_docker::compose_up(ctx, &[], false)?;

// Generic command execution
devkit_tasks::run_cmd(ctx, &command, &opts)?;
```

### Custom Commands
Easy to add project-specific commands:
```rust
Commands::Custom { message } => {
    ctx.print_header("Custom Project Command");
    ctx.print_info(&format!("Message: {}", message));
    // Your logic here
    Ok(())
}
```

## For Your Project

To adapt this for your project:

1. **Copy the structure**
2. **Add your dependencies**: Include the extensions you need
3. **Add your commands**: Create enum variants for your commands
4. **Implement handlers**: Add your logic in `run_command()`
5. **Update menu**: Add items to `run_interactive()`

See [CUSTOM_CLI.md](../../docs/CUSTOM_CLI.md) for a complete guide with Shay-like examples.
