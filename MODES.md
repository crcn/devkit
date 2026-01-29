# devkit Installation Modes

devkit offers two installation modes to match your project's needs:

## 🍽️ Kitchen Sink Mode (Recommended)

**Best for:** Most projects, teams, getting started quickly

### What You Get

A **batteries-included CLI** with all features available out of the box:
- ✅ Docker operations (compose, logs, shell)
- ✅ Database management (migrate, seed, reset)
- ✅ Code quality tools (fmt, lint, test)
- ✅ CI/CD integration
- ✅ Environment management
- ✅ Deployment tools
- ✅ And more...

### How It Works

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
# Choose option 1 (Kitchen Sink)

# Use immediately
./dev.sh              # Interactive menu with all features
./dev.sh start        # Start development environment
./dev.sh cmd build    # Run package commands
./dev.sh docker up    # Docker operations
./dev.sh db migrate   # Database operations
```

### Configuration

**All configuration via `.dev/config.toml`** - no Rust code required!

```toml
[features]
# Enable/disable features as needed
docker = true
database = true
quality = true
ci = false
env = false
deploy = false
```

### Pros

- ✅ **Zero Rust code** - pure TOML configuration
- ✅ **Instant start** - all features available immediately
- ✅ **Simple updates** - just update devkit version
- ✅ **Perfect for teams** - consistent experience for everyone
- ✅ **Easy onboarding** - new team members get everything

### Cons

- ❌ Less customization - can't add project-specific commands to the CLI
- ❌ Larger binary - includes all extensions (even if disabled)

### When to Use

- ✅ Most projects
- ✅ Standard workflows
- ✅ Teams that want consistency
- ✅ Quick prototypes
- ✅ When you want to avoid writing Rust code

---

## 🔧 Custom CLI Mode

**Best for:** Complex projects, specific requirements, full control

### What You Get

A **customizable CLI project** where you build YOUR tool:
- Generate `dev/cli/` Cargo project
- Add only the extensions you need
- Write custom commands in Rust
- Full control over behavior

### How It Works

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
# Choose option 2 (Custom CLI)

# Customize dev/cli/src/main.rs
# Add commands, logic, project-specific workflows

# Use your custom CLI
./dev.sh              # Your custom commands
./dev.sh deploy       # Your deployment logic
./dev.sh whatever     # Whatever you built
```

### Configuration

**Edit `dev/cli/src/main.rs`** and **`dev/cli/Cargo.toml`**

Add extensions you need:

```toml
# dev/cli/Cargo.toml
[dependencies]
devkit-core = { git = "https://github.com/crcn/devkit" }
devkit-tasks = { git = "https://github.com/crcn/devkit" }

# Add only what you need:
devkit-ext-docker = { git = "https://github.com/crcn/devkit" }
devkit-ext-database = { git = "https://github.com/crcn/devkit" }
```

Add custom commands:

```rust
// dev/cli/src/main.rs
#[derive(Subcommand)]
enum Commands {
    Start,
    Stop,

    // Your custom command
    Deploy {
        #[arg(short, long)]
        env: String,
    },

    // Your unique workflow
    GenerateReports {
        #[arg(short, long)]
        month: String,
    },
}
```

### Pros

- ✅ **Full control** - build exactly what you need
- ✅ **Smaller binary** - only include extensions you use
- ✅ **Project-specific logic** - add custom commands
- ✅ **Maximum flexibility** - modify anything
- ✅ **Type safety** - Rust compiler catches errors

### Cons

- ❌ Requires Rust knowledge
- ❌ More setup/maintenance
- ❌ Each project diverges (harder for teams)

### When to Use

- ✅ Complex projects with unique workflows
- ✅ When you need project-specific commands
- ✅ When you want minimal binary size
- ✅ When you enjoy writing Rust
- ✅ When kitchen sink doesn't fit your needs

---

## Comparison

| Feature | Kitchen Sink | Custom CLI |
|---------|--------------|------------|
| **Setup Time** | Instant | 5-10 minutes |
| **Configuration** | TOML only | TOML + Rust code |
| **Customization** | Feature flags | Full control |
| **Binary Size** | Larger (~10MB) | Smaller (~3-5MB) |
| **Rust Knowledge** | Not required | Required |
| **Update Process** | Simple | Rebuild required |
| **Team Consistency** | Perfect | Varies |
| **Unique Commands** | No | Yes |

---

## Examples

### Kitchen Sink Example

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
# Choose: 1 (Kitchen Sink)

# Configure
cat > .dev/config.toml <<EOF
[project]
name = "my-app"

[features]
docker = true
database = true
quality = true
ci = true
EOF

# Use immediately
./dev.sh
# ┌─ What would you like to do? ─┐
# │ > Start development environment│
# │   Docker operations            │
# │   Database operations          │
# │   Code quality                 │
# │   CI/CD                        │
# └────────────────────────────────┘
```

### Custom CLI Example

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
# Choose: 2 (Custom CLI)

# Add your deployment logic
cat > dev/cli/src/main.rs <<'EOF'
#[derive(Subcommand)]
enum Commands {
    Deploy {
        #[arg(short, long)]
        env: String,
    },
}

// Handler
Commands::Deploy { env } => {
    ctx.print_header(&format!("Deploying to {}", env));

    // Your custom deployment logic
    run_tests()?;
    build_containers()?;
    push_to_registry(&env)?;
    update_k8s_deployment(&env)?;

    ctx.print_success("✓ Deployed!");
    Ok(())
}
EOF

# Use your custom command
./dev.sh deploy --env prod
```

---

## Switching Modes

### From Kitchen Sink to Custom CLI

```bash
# Create custom CLI
mkdir -p dev/cli/src

# Copy kitchen sink as starting point
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/crates/devkit-cli/src/main.rs > dev/cli/src/main.rs
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/templates/cli/Cargo.toml > dev/cli/Cargo.toml

# Update wrapper
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/templates/dev.sh > dev.sh
chmod +x dev.sh

# Now customize dev/cli/src/main.rs as needed
```

### From Custom CLI to Kitchen Sink

```bash
# Backup your CLI
mv dev/cli dev/cli.bak

# Install kitchen sink wrapper
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/templates/dev-kitchen-sink.sh > dev.sh
chmod +x dev.sh

# Configure features in .dev/config.toml
```

---

## Recommendations

### Use Kitchen Sink If:

- 🟢 Standard web/API project
- 🟢 Team of 2+ developers
- 🟢 Want to avoid Rust code
- 🟢 Need quick setup
- 🟢 Standard workflows (Docker, DB, CI/CD)

### Use Custom CLI If:

- 🟢 Unique deployment process
- 🟢 Complex orchestration needs
- 🟢 Solo developer or small team
- 🟢 Want minimal binary
- 🟢 Enjoy writing Rust
- 🟢 Need project-specific commands

### Start Kitchen Sink, Switch If Needed

Most projects should start with **Kitchen Sink**:
1. Instant productivity
2. Discover what features you need
3. Switch to Custom CLI later if you hit limitations

The switch is easy and you keep all your configuration!

---

## FAQ

**Q: Can I use both modes?**
A: No, pick one. But you can switch between them easily.

**Q: Does kitchen sink support all features?**
A: Yes! It includes all extensions, just enable/disable in `.dev/config.toml`.

**Q: Is kitchen sink slower?**
A: No! The binary is cached. Startup time is ~0.1s for both modes.

**Q: Can I customize kitchen sink commands?**
A: Not the CLI commands themselves, but you can add package commands via `dev.toml`.

**Q: Can I contribute to kitchen sink?**
A: Yes! Add features to `devkit-cli` in the main repo.

**Q: Which mode does the shay example use?**
A: Custom CLI - it has unique deployment, mobile, and AI features.

**Q: Can I share my custom CLI as a template?**
A: Yes! Publish it and others can copy your `dev/cli/` directory.

---

## Summary

| | Kitchen Sink | Custom CLI |
|-|--------------|------------|
| **Philosophy** | "Batteries included, configure via TOML" | "Build your own, use devkit as library" |
| **Sweet Spot** | 80% of projects | 20% with unique needs |
| **Learning Curve** | Flat | Moderate |
| **Flexibility** | High (via config) | Maximum (via code) |
| **Maintenance** | Low | Medium |

**Most projects should use Kitchen Sink.** It's fast, simple, and powerful enough for most needs. Switch to Custom CLI only if you have specific requirements that kitchen sink can't meet.
