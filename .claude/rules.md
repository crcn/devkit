# devkit Architecture Rules

## Core Purity

**devkit-core MUST remain pure** - it should only contain foundational abstractions, NOT specific feature implementations.

### ✅ What belongs in `devkit-core`

- **Config system**: Loading `.dev/config.toml` and `dev.toml` files
- **Context**: `AppContext` with shared state (repo root, quiet mode, config)
- **Feature detection**: Auto-detecting what's available (Docker, Git, Node, etc.)
- **Utilities**: Helper functions (repo_root, cmd_exists, etc.)
- **Types**: Core types like `Config`, `PackageConfig`, `Features`

### ❌ What does NOT belong in `devkit-core`

- **Operations**: Docker operations, database migrations, dependency installation, etc.
- **Commands**: Specific CLI commands (start, stop, test, etc.)
- **External integrations**: GitHub API, Docker API, package manager operations
- **Business logic**: Anything that "does" something beyond configuration and detection

### 🎯 Rule of Thumb

If it **executes external commands** or **modifies state**, it belongs in an **extension**, not core.

Examples:
- ❌ Installing npm packages → Extension (`devkit-ext-deps`)
- ❌ Running Docker compose → Extension (`devkit-ext-docker`)
- ❌ Running migrations → Extension (`devkit-ext-database`)
- ✅ Detecting if Docker is available → Core (detection)
- ✅ Loading database config from TOML → Core (config)
- ✅ Providing shared context → Core (context)

## Extension Guidelines

Extensions should:
- Have a single, clear responsibility
- Accept `&AppContext` for shared state
- Be optional (users pick what they need)
- Not depend on each other
- Follow the naming pattern: `devkit-ext-{feature}`

## When in Doubt

Ask: "Is this foundational infrastructure that ALL devkit users need, or is this a specific feature?"

- Foundational → Core
- Specific feature → Extension
