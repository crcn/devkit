# devkit Deployment Guide

## Using the Installer

### Option 1: GitHub Raw URL (Works Now!)

Once you push to GitHub, users can install immediately:

```bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
```

**Pros:**
- No setup required
- Works immediately after pushing
- Free forever
- Can pin to specific branches/tags

**Examples:**
```bash
# From main branch
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh

# From specific tag
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/v0.1.0/install.sh | sh

# From development branch
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/dev/install.sh | sh
```

### Option 2: Custom Domain (devkit.sh)

For a prettier URL like `https://devkit.sh/install`:

#### Using GitHub Pages (Free)

1. **Enable GitHub Pages:**
   ```bash
   # Push docs/index.html
   git add docs/
   git commit -m "Add landing page"
   git push
   ```

2. **Configure in GitHub:**
   - Go to repo Settings → Pages
   - Source: Deploy from branch
   - Branch: `main`, folder: `/docs`
   - Save

3. **Add custom domain (optional):**
   - Buy `devkit.sh` domain
   - Add CNAME record: `CNAME devkit.sh.pages.dev`
   - In GitHub Settings → Pages → Custom domain: `devkit.sh`

4. **Create install redirect:**
   Create `docs/install` (no extension):
   ```bash
   #!/bin/sh
   curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
   ```

#### Using Cloudflare Pages (Free)

1. **Connect repo to Cloudflare Pages**
2. **Build settings:**
   - Framework: None
   - Build command: (empty)
   - Build output: `/docs`

3. **Add redirect rule:**
   Create `docs/_redirects`:
   ```
   /install https://raw.githubusercontent.com/crcn/devkit/main/install.sh 200
   ```

### Option 3: Self-Host

Host `install.sh` on your own server:

```bash
# On your server
curl -fsSL https://yoursite.com/install.sh | sh
```

## Testing Locally

### Test the Installer

```bash
# In a test project directory
cd /tmp/test-project
git init

# Run the installer locally
bash ~/Developer/crcn/devkit/install.sh

# Verify it created:
ls -la dev.sh .dev/config.toml dev/cli/

# Test the CLI
./dev.sh doctor
./dev.sh status
```

### Test Template Modifications

After modifying templates:

```bash
# Clean test
rm -rf /tmp/test-project
mkdir /tmp/test-project
cd /tmp/test-project
git init

# Run installer
bash ~/Developer/crcn/devkit/install.sh

# Test the generated CLI
./dev.sh
```

## Publishing to GitHub

### 1. Commit Everything

```bash
cd ~/Developer/crcn/devkit

git add .
git commit -m "Add rustup-style installer"
git push origin main
```

### 2. Create a Release Tag

```bash
# Tag the release
git tag -a v0.1.0 -m "Initial installer release"
git push origin v0.1.0
```

Now users can:
```bash
# Install from main (latest)
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh

# Install from tag (pinned)
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/v0.1.0/install.sh | sh
```

### 3. Enable GitHub Pages (Optional)

1. Push `docs/index.html`
2. Go to repo Settings → Pages
3. Enable: Deploy from branch `main` → `/docs`
4. Visit `https://crcn.github.io/devkit`

### 4. Update README

Add the install command to your README:

```markdown
## Quick Start

```bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
\```
```

## Promoting Your Installer

### In Documentation

```markdown
# Quick Install
\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
\`\`\`

Or inspect first:
\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh > /tmp/install.sh
less /tmp/install.sh
bash /tmp/install.sh
\`\`\`
```

### In Examples

Show the full flow:

```bash
# Create new project
mkdir my-app && cd my-app
git init

# Install devkit
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh

# Start developing
./dev.sh
```

### Social Media

Tweet/post:
```
🚀 Just released devkit - a rustup-style installer for dev environment orchestration!

One command to set up your entire dev workflow:

curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh

✅ Auto-installs Rust
✅ Creates custom CLI
✅ Smart project detection
✅ Zero config needed

https://github.com/crcn/devkit
```

## Security Best Practices

### For Installer Users

Always inspect before running:
```bash
# Download and inspect
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh > /tmp/install.sh
less /tmp/install.sh

# Run when satisfied
bash /tmp/install.sh
```

### For You (Maintainer)

1. **Sign releases:**
   ```bash
   git tag -s v0.1.0 -m "Signed release"
   git push origin v0.1.0
   ```

2. **Provide checksums:**
   ```bash
   sha256sum install.sh > install.sh.sha256
   ```

3. **Keep installer simple:**
   - Minimal dependencies
   - Clear, auditable code
   - No eval or hidden commands

## Updating the Installer

### For New Features

1. Update `install.sh`
2. Update templates in `templates/`
3. Test locally
4. Commit and push
5. Tag new version

Users on `main` get it automatically:
```bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
```

Users on tags need to update manually:
```bash
# They were on v0.1.0, now update to v0.2.0
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/v0.2.0/install.sh | sh
```

## Analytics (Optional)

Track installations without compromising privacy:

### Using a redirect service

```bash
# Instead of direct raw link, use a redirect
curl -fsSL https://devkit.sh/install | sh

# Which redirects to:
https://raw.githubusercontent.com/crcn/devkit/main/install.sh
```

The redirect service can count hits without seeing who.

### GitHub Traffic

GitHub shows:
- Clones/visits to repo
- Raw file downloads
- Release download counts

Check in: Insights → Traffic

## Next Steps

1. **Test locally** - Run installer in test projects
2. **Push to GitHub** - Make it available via raw URL
3. **Share** - Add to README, tweet, blog post
4. **Iterate** - Gather feedback, improve templates
5. **Document** - Add examples, guides, videos

The beauty of this approach:
- ✅ Works immediately after pushing
- ✅ No infrastructure needed
- ✅ Users can inspect source
- ✅ Version pinning available
- ✅ Can upgrade to custom domain later
