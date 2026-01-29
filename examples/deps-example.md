# Smart Dependency Installation Example

## Scenario

You have a workspace with multiple packages:

```
my-project/
├── .dev/
│   └── config.toml          # Workspace config
├── packages/
│   ├── api/                 # Rust package
│   │   └── Cargo.toml
│   ├── web/                 # TypeScript package (pnpm)
│   │   ├── package.json
│   │   └── pnpm-lock.yaml
│   └── scripts/             # Python package (poetry)
│       └── pyproject.toml
└── dev.sh
```

## Configuration

### .dev/config.toml

```toml
[workspaces]
packages = ["packages/*"]

[features]
deps = true  # Enable auto-dependency installation
```

## Usage

### Auto-install on startup

```bash
$ ./dev.sh start
```

Output:
```
Found 3 package(s) that need dependencies installed
  api [Rust] via cargo
  web [TypeScript] via pnpm
  scripts [Python] via poetry

Install dependencies now? (Y/n) y

Installing dependencies for 3 package(s)...
  Installing Rust dependencies in api...
  Installing TypeScript dependencies in web...
  Installing Python dependencies in scripts...
✓ All dependencies installed

Starting development environment
✓ Development environment ready!
```

### Manual installation

```bash
$ ./dev.sh deps
```

### List discovered packages

```bash
$ ./dev.sh deps --list
```

Output:
```
Discovered Packages

  api [Rust] via cargo - up to date
  web [TypeScript] via pnpm - needs install
  scripts [Python] via poetry - up to date
```

## Smart Detection Examples

### Node.js Package Managers

devkit automatically detects which package manager you use:

```bash
# pnpm project (has pnpm-lock.yaml)
web/ → pnpm install

# yarn project (has yarn.lock)
app/ → yarn install

# bun project (has bun.lockb)
tools/ → bun install

# npm project (default)
legacy/ → npm install
```

### Python Package Managers

```bash
# Poetry project (has [tool.poetry] in pyproject.toml)
ml-service/ → poetry install

# UV project (has [tool.uv] in pyproject.toml)
fast-api/ → uv pip install -r requirements.txt

# Pipenv project (has Pipfile)
data-processor/ → pipenv install

# Standard pip (has requirements.txt)
scripts/ → pip install -r requirements.txt
```

### Multi-language Workspace

All languages work together seamlessly:

```
monorepo/
├── services/
│   ├── api/ (Rust)
│   ├── web/ (TypeScript + pnpm)
│   └── worker/ (Go)
├── packages/
│   ├── shared/ (TypeScript + npm)
│   └── ui-components/ (TypeScript + yarn)
└── scripts/
    ├── deploy/ (Python + poetry)
    └── maintenance/ (Ruby + bundler)
```

Running `./dev.sh start` automatically:
- Runs `cargo fetch` for Rust services
- Runs `pnpm install` for TypeScript with pnpm
- Runs `go mod download` for Go services
- Runs `npm install` for shared package
- Runs `yarn install` for UI components
- Runs `poetry install` for Python scripts
- Runs `bundle install` for Ruby scripts

## Only Installs When Needed

Smart timestamp checking:
- ✅ **Installs**: When package.json is newer than node_modules
- ✅ **Installs**: When Cargo.toml is newer than Cargo.lock
- ✅ **Installs**: When lock file is newer than dependencies
- ⏭️ **Skips**: When everything is up to date

Example:
```bash
$ ./dev.sh deps
✓ All dependencies up to date

# Edit package.json
$ touch packages/web/package.json

$ ./dev.sh deps
Installing dependencies for 1 package(s)...
  Installing TypeScript dependencies in web...
✓ All dependencies installed
```

## Integration with Existing Commands

Prerun hooks run automatically before any command:

```bash
# Automatically checks and installs deps before starting
$ ./dev.sh start

# Automatically checks and installs deps before running tests
$ ./dev.sh test

# Automatically checks and installs deps before building
$ ./dev.sh cmd build
```

## Quiet Mode

In CI/CD or scripts, run in quiet mode:

```bash
$ ./dev.sh --quiet start
# No prompts, just installs what's needed and continues
```
