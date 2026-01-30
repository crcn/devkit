# devkit CLI - Complete Usage Example

## Scenario: Building a Full-Stack App

You're building a SaaS app with:
- API backend (Rust)
- Web frontend (TypeScript/React)
- Mobile app (React Native)
- PostgreSQL database
- Docker for local dev

Let's see how devkit handles this entire workflow.

---

## 1. Initial Setup

### Install devkit

```bash
# Start with a new project
mkdir my-saas-app
cd my-saas-app
git init

# Install devkit
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
```

**Output:**
```
==> Checking dependencies...
✓ All dependencies satisfied

==> Installing to: /Users/you/my-saas-app

==> Installing dev.sh wrapper...
✓ Installed dev.sh

==> Creating dev/cli project...
✓ Created dev/cli

==> Creating .dev/config.toml...
✓ Created .dev/config.toml

✓ devkit installed successfully!

Next steps:
  1. Review and customize .dev/config.toml
  2. Add commands to package dev.toml files
  3. Customize dev/cli/src/main.rs for your project
  4. Run the CLI:
     $ ./dev.sh              # Interactive menu
     $ ./dev.sh start        # Start development environment
```

---

## 2. Project Structure Setup

### Create your packages

```bash
# Create workspace structure
mkdir -p packages/{api,web,mobile}
mkdir -p infra

# Initialize packages
cd packages/api
cargo init --name api-server
cd ../web
npm init -y
cd ../mobile
npx create-expo-app@latest --template blank-typescript
cd ../..
```

### Configure devkit

Edit `.dev/config.toml`:

```toml
[project]
name = "my-saas-app"

[workspaces]
packages = ["packages/*"]
exclude = []

[environments]
available = ["dev", "staging", "prod"]
default = "dev"

[services]
api = 8080
postgres = 5432
redis = 6379

[urls.api]
label = "API Server"
url = "http://localhost:8080"

[urls.docs]
label = "API Documentation"
url = "http://localhost:8080/docs"
```

---

## 3. Add Package Commands

### API Package (`packages/api/dev.toml`)

```toml
[database]
migrations = "migrations"
seeds = "seeds/dev.sql"

[cmd.typecheck]
default = "cargo check --all-targets"

[cmd.lint]
default = "cargo clippy --all-targets -- -D warnings"
fix = "cargo clippy --fix --allow-dirty --allow-staged"

[cmd.fmt]
default = "cargo fmt --check"
fix = "cargo fmt"

[cmd.build]
default = "cargo build"
watch = "cargo watch -x run"
release = "cargo build --release"

[cmd]
test = "cargo test"
```

### Web Package (`packages/web/dev.toml`)

```toml
[cmd.typecheck]
default = "npx tsc --noEmit"

[cmd.lint]
default = "npx eslint src"
fix = "npx eslint src --fix"

[cmd.fmt]
default = "npx prettier --check 'src/**/*.{ts,tsx}'"
fix = "npx prettier --write 'src/**/*.{ts,tsx}'"

[cmd.build]
default = "npx vite build"
watch = "npx vite"

[cmd]
test = "npx vitest run"
```

### Mobile Package (`packages/mobile/dev.toml`)

```toml
[mobile]
startup_timeout_secs = 300

[cmd.typecheck]
default = "npx tsc --noEmit"
deps = ["web:build"]  # Share types with web

[cmd.lint]
default = "npx eslint src"
fix = "npx eslint src --fix"

[cmd.build]
default = "npx expo export"

[cmd]
test = "npx jest"
```

### Add Docker Compose (`docker-compose.yml`)

```yaml
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: myapp_dev
      POSTGRES_USER: dev
      POSTGRES_PASSWORD: dev
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

volumes:
  postgres_data:
```

---

## 4. First Time Using the CLI

### Interactive Menu

```bash
./dev.sh
```

**First run** (builds the CLI):
```
Building dev-cli (release mode)...
    Finished release [optimized] target(s) in 28.3s
```

**Interactive menu appears:**
```
What would you like to do?
> Start development environment
  Stop services
  Run commands (cmd)
  Status
  Doctor
  Exit
```

### Check System Health

```bash
./dev.sh doctor
```

**Output:**
```
=== System Health Check ===

✓ git
✓ cargo
✓ docker

Health check complete
```

### Show Status

```bash
./dev.sh status
```

**Output:**
```
=== Development Environment Status ===

Repository: /Users/you/my-saas-app
Project: my-saas-app

✓ Configuration loaded
```

---

## 5. Daily Development Workflow

### Morning: Start Everything

```bash
./dev.sh start
```

**Output:**
```
=== Starting development environment ===

[start] Syncing environment variables...
✓ Pulled environment variables to .env.dev

[start] Starting docker containers...
[+] Running 3/3
 ✔ Network my-saas-app_default      Created
 ✔ Container my-saas-app-postgres-1 Started
 ✔ Container my-saas-app-redis-1    Started

✓ Development environment started!
```

### Run Type Checking Across All Packages

```bash
./dev.sh cmd typecheck
```

**Output:**
```
[typecheck] Running cargo check --all-targets on api...
    Checking api-server v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.2s

[typecheck] Running npx tsc --noEmit on web...
✓ No type errors

[typecheck] Running npx tsc --noEmit on mobile...
✓ No type errors

✓ 3 package(s) succeeded: api, web, mobile
```

### Run Specific Command on Specific Package

```bash
./dev.sh cmd build -p api
```

**Output:**
```
[build] Running cargo build on api...
   Compiling api-server v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 3.4s

✓ 1 package(s) succeeded: api
```

### Use Command Variants

```bash
# Run build in watch mode
./dev.sh cmd build:watch -p api
```

**Output:**
```
[build] Running cargo watch -x run on api...
[Running 'cargo run']
   Compiling api-server v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.1s
     Running `target/debug/api-server`

🚀 API server listening on http://localhost:8080
```

### Auto-Fix All Formatting

```bash
./dev.sh cmd fmt:fix
```

**Output:**
```
[fmt] Running cargo fmt on api...
✓ Formatted

[fmt] Running npx prettier --write 'src/**/*.{ts,tsx}' on web...
src/App.tsx 52ms
src/components/Button.tsx 12ms

[fmt] Running npx prettier --write 'src/**/*.{ts,tsx}' on mobile...
src/App.tsx 48ms

✓ 3 package(s) succeeded: api, web, mobile
```

### Run Linters with Auto-Fix

```bash
./dev.sh cmd lint:fix
```

**Output:**
```
[lint] Running cargo clippy --fix --allow-dirty on api...
    Checking api-server v0.1.0
warning: unused variable: `x`
 --> src/main.rs:42:9
  |
42|     let x = 5;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
✓ Auto-fixed 1 issue

[lint] Running npx eslint src --fix on web...
✓ Fixed 3 issues

[lint] Running npx eslint src --fix on mobile...
✓ No issues

✓ 3 package(s) succeeded: api, web, mobile
```

### Run Tests in Parallel

```bash
./dev.sh cmd test --parallel
```

**Output:**
```
[test] Starting cargo test on api...
[test] Starting npx vitest run on web...
[test] Starting npx jest on mobile...

[api] running 42 tests
[web] ✓ src/components/Button.test.tsx (3)
[mobile] PASS src/App.test.tsx

[api] test result: ok. 42 passed; 0 failed
[web] Test Files  12 passed (12)
[mobile] Test Suites: 5 passed, 5 total

✓ 3 package(s) succeeded: api, web, mobile
```

### List All Available Commands

```bash
./dev.sh cmd --list
```

**Output:**
```
Available commands:

  build (api, web, mobile)
  fmt (api, web, mobile)
  lint (api, web, mobile)
  test (api, web, mobile)
  typecheck (api, web, mobile)
```

---

## 6. Database Operations

### Run Migrations

```bash
./dev.sh db migrate
```

**Output:**
```
=== Running database migrations ===

Applied 1 migration:
  ✓ 20240129_create_users_table
```

### Seed Database

```bash
./dev.sh db seed
```

**Output:**
```
=== Seeding database ===

✓ Inserted 10 users
✓ Inserted 50 posts
✓ Database seeded successfully
```

### Open Database Shell

```bash
./dev.sh db psql
```

**Output:**
```
psql (16.1)
Type "help" for help.

myapp_dev=# \dt
          List of relations
 Schema |   Name    | Type  | Owner
--------+-----------+-------+-------
 public | users     | table | dev
 public | posts     | table | dev
(2 rows)

myapp_dev=#
```

---

## 7. Docker Operations

### View Container Logs

```bash
./dev.sh logs postgres
```

**Output:**
```
[postgres] 2024-01-29 10:23:45.123 UTC [1] LOG:  starting PostgreSQL 16.1
[postgres] 2024-01-29 10:23:45.567 UTC [1] LOG:  listening on IPv4 address "0.0.0.0", port 5432
[postgres] 2024-01-29 10:23:45.890 UTC [1] LOG:  database system is ready to accept connections

[Auto-reconnecting on container restart...]
```

### Shell into Container

```bash
./dev.sh shell postgres
```

**Output:**
```
root@abc123:/# psql -U dev myapp_dev
psql (16.1)
Type "help" for help.

myapp_dev=#
```

### Restart Services

```bash
./dev.sh docker restart
```

**Output:**
```
=== Restarting containers ===

[+] Running 2/2
 ✔ Container my-saas-app-postgres-1 Restarted
 ✔ Container my-saas-app-redis-1    Restarted

✓ Containers restarted
```

---

## 8. Customizing the CLI

### Add a Deploy Command

Edit `dev/cli/src/main.rs`:

```rust
#[derive(Subcommand)]
enum Commands {
    Start,
    Stop,
    Status,
    Doctor,
    Cmd { /* ... */ },

    // Add this:
    /// Deploy to cloud environment
    Deploy {
        /// Environment: dev, staging, prod
        #[arg(short, long)]
        env: String,

        /// Skip confirmation prompts
        #[arg(long)]
        yes: bool,
    },
}

// Add handler in run():
match cli.command {
    // ... other handlers

    Some(Commands::Deploy { env, yes }) => {
        ctx.print_header(&format!("Deploying to {}", env));

        if !yes && env == "prod" {
            if !ctx.confirm("Deploy to PRODUCTION?", false)? {
                ctx.print_warning("Deployment cancelled");
                return Ok(());
            }
        }

        ctx.print_info("Building containers...");
        // Your deployment logic here

        ctx.print_success(&format!("✓ Deployed to {}", env));
        Ok(())
    }
}
```

### Rebuild and Use

```bash
# Rebuild happens automatically on next run
./dev.sh deploy --env staging
```

**Output:**
```
=== Deploying to staging ===

Building containers...
Pushing to registry...
Updating ECS services...

✓ Deployed to staging
```

---

## 9. Adding Extensions

### Add Docker Extension

Edit `dev/cli/Cargo.toml`:

```toml
[dependencies]
devkit-core = { git = "https://github.com/crcn/devkit" }
devkit-tasks = { git = "https://github.com/crcn/devkit" }

# Add Docker extension
devkit-ext-docker = { git = "https://github.com/crcn/devkit" }
```

Edit `dev/cli/src/main.rs`:

```rust
use devkit_ext_docker::{compose_up, compose_down};

// Now use in your commands:
Some(Commands::Start) => {
    ctx.print_header("Starting development environment");
    compose_up(&ctx, &[], false)?;
    ctx.print_success("✓ Services started");
    Ok(())
}
```

---

## 10. End of Day

### Stop Everything

```bash
./dev.sh stop
```

**Output:**
```
=== Stopping development environment ===

[+] Running 3/3
 ✔ Container my-saas-app-postgres-1 Stopped
 ✔ Container my-saas-app-redis-1    Stopped
 ✔ Network my-saas-app_default      Removed

✓ Services stopped
```

---

## 11. Team Workflow

### New Team Member Joins

They run:
```bash
git clone https://github.com/yourorg/my-saas-app.git
cd my-saas-app

# One command setup
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh

# Start developing
./dev.sh start
./dev.sh cmd build
./dev.sh cmd test
```

Done! They have:
- ✅ Rust installed (auto)
- ✅ Dev CLI built
- ✅ Docker containers running
- ✅ Database migrated
- ✅ Ready to code

### CI/CD Integration

In `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install devkit
        run: |
          curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh

      - name: Run checks
        run: |
          ./dev.sh cmd typecheck
          ./dev.sh cmd lint
          ./dev.sh cmd test --parallel

      - name: Build
        run: ./dev.sh cmd build:release
```

---

## Summary: What You Get

### Single Entry Point
```bash
./dev.sh                  # All dev operations
```

### Fast Command Execution
```bash
./dev.sh cmd test         # ~0.1s startup (after first build)
```

### Consistent Interface
```bash
./dev.sh cmd build        # Works on ALL packages
./dev.sh cmd build:watch  # Variants work everywhere
./dev.sh cmd lint:fix     # Auto-fix across all packages
```

### Dependency Awareness
```toml
[cmd.build]
deps = ["common:build"]   # Runs dependencies first
```

### Customizable per Project
- Start with templates
- Add commands as needed
- Extend with devkit extensions
- Keep project-specific logic

### One Command Onboarding
```bash
curl ... | sh && ./dev.sh start
```

That's devkit! 🚀
