# Extension Templates

This directory contains templates for creating custom devkit extensions.

## Quick Start

1. **Create extension directory**:
   ```bash
   mkdir -p .dev/extensions/my-tools
   ```

2. **Copy config template**:
   ```bash
   cp extensions/templates/example-config.toml .dev/extensions/my-tools/config.toml
   ```

3. **Copy a script template**:
   ```bash
   cp extensions/templates/shell-script.sh .dev/extensions/my-tools/hello.sh
   chmod +x .dev/extensions/my-tools/hello.sh
   ```

4. **Edit config to reference your script**:
   ```toml
   name = "my-tools"
   version = "1.0.0"

   [[action]]
   id = "hello"
   label = "👋 Say Hello"
   group = "Custom"
   command = "hello.sh"
   ```

5. **Test**:
   ```bash
   devkit
   # Your extension appears in the menu!
   ```

## Directory Structure

Extensions live in `.dev/extensions/{name}/`:

```
.dev/extensions/
  my-tools/               # Your extension
    config.toml           # Extension configuration
    hello.sh              # Shell script
    deploy.py             # Python script
    lib/                  # Helper modules (optional)
      helper.py
  another-extension/      # Another extension
    config.toml
    build.js
```

## Configuration Format

Each extension needs a `config.toml` file:

```toml
name = "extension-name"
version = "1.0.0"
description = "What this extension does"

[[action]]
id = "unique-id"
label = "🎯 Menu Label"
group = "Menu Group"
description = "What this action does"
command = "script.sh"           # Path relative to extension dir
args = ["--flag", "value"]      # Optional arguments
[action.env]                    # Optional environment variables
CUSTOM_VAR = "value"
```

## Environment Variables

Your scripts receive context through environment variables:

- `DEVKIT_REPO_ROOT` - Repository root path
- `DEVKIT_QUIET` - "1" if quiet mode, "0" otherwise
- `DEVKIT_FEATURE_DOCKER` - "1" if Docker available
- `DEVKIT_FEATURE_GIT` - "1" if Git available
- `DEVKIT_FEATURE_CARGO` - "1" if Cargo available
- `DEVKIT_FEATURE_NODE` - "1" if Node.js available
- `DEVKIT_FEATURE_DATABASE` - "1" if database configured

## Language Support

Extensions can be written in **any language**:

### Shell Scripts (.sh, .bash)
```bash
#!/usr/bin/env bash
set -euo pipefail
echo "Hello from $DEVKIT_REPO_ROOT"
```

### Python (.py)
```python
#!/usr/bin/env python3
import os
repo_root = os.environ['DEVKIT_REPO_ROOT']
print(f"Hello from {repo_root}")
```

### Node.js (.js, .mjs)
```javascript
#!/usr/bin/env node
const repoRoot = process.env.DEVKIT_REPO_ROOT;
console.log(`Hello from ${repoRoot}`);
```

### Compiled Binaries (Rust, Go, etc.)
Any executable binary works! Just compile and reference it in config.toml.

## Examples

### Simple Shell Command

**.dev/extensions/deploy/config.toml**:
```toml
name = "deploy"
version = "1.0.0"

[[action]]
id = "deploy-prod"
label = "🚀 Deploy Production"
group = "Deploy"
command = "deploy.sh"
args = ["--env", "production"]
```

**.dev/extensions/deploy/deploy.sh**:
```bash
#!/usr/bin/env bash
set -euo pipefail

ENV="${1:---env}"
ENV="${2:-staging}"

echo "🚀 Deploying to $ENV from $DEVKIT_REPO_ROOT..."
./scripts/build.sh
./scripts/push.sh "$ENV"
echo "✓ Deployment complete!"
```

### Python Analysis Tool

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

# Your analysis logic
py_files = list(repo_root.rglob('*.py'))
print(f"Found {len(py_files)} Python files")
```

### Node.js Build Tool

**.dev/extensions/build/config.toml**:
```toml
name = "build"
version = "1.0.0"

[[action]]
id = "build-assets"
label = "🔨 Build Assets"
group = "Build"
command = "build.js"
```

**.dev/extensions/build/build.js**:
```javascript
#!/usr/bin/env node
const { execSync } = require('child_process');
const repoRoot = process.env.DEVKIT_REPO_ROOT;

console.log(`🔨 Building assets in ${repoRoot}...`);
execSync('npm run build', { cwd: repoRoot, stdio: 'inherit' });
console.log('✓ Build complete!');
```

## Best Practices

1. **Keep it simple** - Extensions are just executables
2. **Use exit codes** - Exit 0 for success, non-zero for failure
3. **Make scripts executable** - `chmod +x script.sh`
4. **Organize by purpose** - One extension per logical grouping
5. **Version your extensions** - Include version in config.toml
6. **Document your actions** - Use description field in config
7. **Use groups** - Group related actions together in the menu
8. **Test locally first** - Run your scripts manually before adding to config
9. **Commit to git** - Share extensions with your team

## Troubleshooting

### Extension not appearing in menu
- Check that `.dev/extensions/{name}/config.toml` exists
- Verify TOML syntax is valid
- Check devkit output for loading errors

### Script fails to execute
- Ensure script is executable: `chmod +x script.sh`
- Check shebang line: `#!/usr/bin/env bash`
- Verify script exists at path specified in config
- Test script manually: `.dev/extensions/{name}/script.sh`

### Command not found
- Use absolute paths or repo-relative paths
- Check `DEVKIT_REPO_ROOT` environment variable
- Ensure dependencies are installed

## Distribution

To share extensions with your team:

1. Commit to git:
   ```bash
   git add .dev/extensions/
   git commit -m "Add custom devkit extensions"
   git push
   ```

2. Team members automatically get them:
   ```bash
   git pull
   devkit  # Extensions appear automatically!
   ```

## More Information

See the main devkit README for more details on creating and distributing extensions.
