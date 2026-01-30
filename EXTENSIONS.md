# External Extensions

Devkit supports custom extensions defined by simple TOML configuration files. Extensions can be written in **any language** - shell scripts, Python, Node.js, Rust, or any executable.

## Quick Start

1. Create an extension directory:
   ```bash
   mkdir -p .dev/extensions/my-tools
   ```

2. Create a `config.toml`:
   ```toml
   name = "my-tools"
   version = "1.0.0"

   [[action]]
   id = "hello"
   label = "👋 Say Hello"
   group = "Custom"
   command = "hello.sh"
   ```

3. Create your script:
   ```bash
   cat > .dev/extensions/my-tools/hello.sh <<'EOF'
   #!/usr/bin/env bash
   echo "Hello from $DEVKIT_REPO_ROOT!"
   EOF
   chmod +x .dev/extensions/my-tools/hello.sh
   ```

4. Run devkit:
   ```bash
   devkit
   # Your extension appears in the menu!
   ```

## How It Works

### Discovery

Devkit automatically scans `.dev/extensions/*/config.toml` files and loads them as extensions. Each subdirectory in `.dev/extensions/` can contain an extension.

### Configuration Format

Each extension needs a `config.toml` file:

```toml
name = "extension-name"
version = "1.0.0"
description = "What this extension does" # optional

[[action]]
id = "unique-id"
label = "🎯 Menu Label"
group = "Menu Group"              # optional, for organizing menu items
description = "What this does"    # optional
command = "script.sh"             # path relative to extension directory
args = ["--flag", "value"]        # optional arguments
[action.env]                      # optional environment variables
CUSTOM_VAR = "value"
```

### Directory Structure

Extensions are self-contained in their own directories:

```
.dev/extensions/
  deploy/
    config.toml
    deploy.sh
    rollback.sh
    lib/
      helper.sh
  analysis/
    config.toml
    analyze.py
    report.py
  build-tools/
    config.toml
    build.js
    test.js
```

### Environment Variables

Your scripts automatically receive context via environment variables:

| Variable | Description |
|----------|-------------|
| `DEVKIT_REPO_ROOT` | Absolute path to repository root |
| `DEVKIT_QUIET` | "1" if quiet mode, "0" otherwise |
| `DEVKIT_FEATURE_DOCKER` | "1" if Docker available |
| `DEVKIT_FEATURE_GIT` | "1" if Git available |
| `DEVKIT_FEATURE_CARGO` | "1" if Cargo available |
| `DEVKIT_FEATURE_NODE` | "1" if Node.js available |
| `DEVKIT_FEATURE_DATABASE` | "1" if database configured |

### Exit Codes

- Return exit code `0` for success
- Return non-zero exit code for failure (devkit will display the error)

## Language Examples

### Shell Script

**.dev/extensions/deploy/config.toml**:
```toml
name = "deploy"
version = "1.0.0"

[[action]]
id = "deploy-prod"
label = "🚀 Deploy Production"
group = "Deploy"
command = "deploy.sh"
args = ["production"]
```

**.dev/extensions/deploy/deploy.sh**:
```bash
#!/usr/bin/env bash
set -euo pipefail

ENV="${1:-staging}"
echo "🚀 Deploying to $ENV..."

cd "$DEVKIT_REPO_ROOT"
./scripts/build.sh
./scripts/push.sh "$ENV"

echo "✓ Deployed!"
```

### Python

**.dev/extensions/analysis/config.toml**:
```toml
name = "analysis"
version = "1.0.0"

[[action]]
id = "analyze"
label = "📊 Analyze Code"
group = "Tools"
command = "analyze.py"
```

**.dev/extensions/analysis/analyze.py**:
```python
#!/usr/bin/env python3
import os
from pathlib import Path

repo_root = Path(os.environ['DEVKIT_REPO_ROOT'])
print(f"📊 Analyzing {repo_root}...")

# Your analysis logic here
py_files = list(repo_root.rglob('*.py'))
print(f"Found {len(py_files)} Python files")

print("✓ Analysis complete!")
```

### Node.js

**.dev/extensions/build/config.toml**:
```toml
name = "build"
version = "1.0.0"

[[action]]
id = "build"
label = "🔨 Build Assets"
group = "Build"
command = "build.js"
```

**.dev/extensions/build/build.js**:
```javascript
#!/usr/bin/env node
const { execSync } = require('child_process');
const repoRoot = process.env.DEVKIT_REPO_ROOT;

console.log(`🔨 Building in ${repoRoot}...`);
execSync('npm run build', { cwd: repoRoot, stdio: 'inherit' });
console.log('✓ Build complete!');
```

### Rust (Compiled Binary)

You can even use compiled binaries:

**.dev/extensions/optimizer/config.toml**:
```toml
name = "optimizer"
version = "1.0.0"

[[action]]
id = "optimize"
label = "⚡ Optimize"
group = "Build"
command = "optimizer"  # compiled binary
```

Build your Rust binary and place it in the extension directory.

## Features

### Multiple Actions Per Extension

Extensions can provide multiple actions:

```toml
name = "deploy"
version = "1.0.0"

[[action]]
id = "deploy-staging"
label = "🚀 Deploy Staging"
group = "Deploy"
command = "deploy.sh"
args = ["staging"]

[[action]]
id = "deploy-prod"
label = "🚀 Deploy Production"
group = "Deploy"
command = "deploy.sh"
args = ["production"]

[[action]]
id = "rollback"
label = "⏮️ Rollback"
group = "Deploy"
command = "rollback.sh"
```

### Custom Environment Variables

Pass additional environment variables to your scripts:

```toml
[[action]]
id = "deploy"
label = "🚀 Deploy"
command = "deploy.sh"
[action.env]
DEPLOY_TIMEOUT = "300"
DEPLOY_REGION = "us-east-1"
```

### Organized Menu Groups

Use the `group` field to organize actions in the menu:

```toml
[[action]]
label = "🔨 Build"
group = "Build"
# ...

[[action]]
label = "🧪 Test"
group = "Build"
# ...

[[action]]
label = "🚀 Deploy"
group = "Deploy"
# ...
```

## Best Practices

1. **Keep extensions simple** - They're just executables
2. **One extension per logical grouping** - Don't put unrelated actions together
3. **Use meaningful groups** - Organize actions in the menu
4. **Include descriptions** - Help users understand what actions do
5. **Version your extensions** - Track changes with version field
6. **Make scripts executable** - `chmod +x script.sh`
7. **Use exit codes correctly** - 0 for success, non-zero for failure
8. **Print clear output** - Users see stdout/stderr
9. **Handle errors gracefully** - Provide helpful error messages
10. **Commit to git** - Share extensions with your team

## Templates

Example templates are available in `extensions/templates/`:

- `example-config.toml` - Configuration template
- `shell-script.sh` - Shell script template
- `python-script.py` - Python script template
- `nodejs-script.js` - Node.js script template

Copy and customize these for your own extensions.

## Troubleshooting

### Extension not appearing

1. Check that `.dev/extensions/{name}/config.toml` exists
2. Verify TOML syntax with `toml-cli` or online validator
3. Look for error messages when running devkit
4. Check directory structure matches expected format

### Script execution fails

1. Ensure script is executable: `chmod +x script.sh`
2. Verify shebang line: `#!/usr/bin/env bash` (or python3, node, etc.)
3. Test script manually: `.dev/extensions/{name}/script.sh`
4. Check that `command` path in config.toml is correct

### Environment variables not available

1. Verify you're reading from `os.environ` or `process.env` correctly
2. Check that devkit is setting variables (look at template examples)
3. Print environment variables to debug: `env | grep DEVKIT`

## Distribution

### Within Your Team

Simply commit extensions to git:

```bash
git add .dev/extensions/
git commit -m "Add custom devkit extensions"
git push
```

Team members automatically get them:

```bash
git pull
devkit  # Extensions appear automatically!
```

### Sharing Publicly

You can share extension templates by:

1. Publishing a git repository with example extensions
2. Providing copy-paste instructions in documentation
3. Creating a script to install extensions

Example install script:

```bash
#!/bin/bash
# install-my-extension.sh
mkdir -p .dev/extensions/my-extension
curl -o .dev/extensions/my-extension/config.toml https://example.com/config.toml
curl -o .dev/extensions/my-extension/script.sh https://example.com/script.sh
chmod +x .dev/extensions/my-extension/script.sh
```

## Advantages

| Feature | Description |
|---------|-------------|
| **Language agnostic** | Use any language you know |
| **No build required** | Scripts run directly (unless you choose to compile) |
| **Easy to debug** | Use native debugging tools for your language |
| **Simple to share** | Commit config + scripts to git |
| **Fast discovery** | No subprocess execution needed at startup |
| **Self-contained** | Each extension in its own directory |
| **Clear configuration** | TOML is easy to read and write |

## Examples in This Repository

The devkit repository includes a test extension at `.dev/extensions/test/` that demonstrates the system. Run `devkit` in this repo to see it in action.

## See Also

- [Templates Documentation](extensions/templates/README.md) - Detailed templates and examples
- [Main README](README.md) - General devkit documentation
