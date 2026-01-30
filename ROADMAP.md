# devkit Feature Roadmap

## Overview
This roadmap outlines planned features for devkit, organized by implementation phases with dependencies and effort estimates.

---

## Phase 1: Foundation Enhancements (Weeks 1-2)

### 1.1 Auto-Update Checker ✨ **STARTING HERE**
**Effort**: 2-3 hours
**Priority**: High
**Dependencies**: None

**Features**:
- Check GitHub releases for new versions
- Compare current version with latest
- Show update notification in CLI
- Optional auto-download and install
- Configurable update check frequency
- Respect `--no-update-check` flag

**Implementation**:
- Add version checking module to `devkit-core`
- Cache last check timestamp
- Integrate into CLI startup (non-blocking)
- Add `devkit update` command for manual updates

---

### 1.2 Generic --watch Flag
**Effort**: 4-6 hours
**Priority**: High
**Dependencies**: None
**Enables**: All watch-based extensions

**Features**:
- Add `--watch` flag to any command
- File pattern detection based on command context
- Debouncing (configurable delay)
- Clear terminal on rerun
- Keyboard shortcuts (r=rerun, q=quit, c=clear)
- Watch configuration in `dev.toml`

**Implementation**:
```toml
[cmd.build]
default = "cargo build"
watch_patterns = ["src/**/*.rs", "Cargo.toml"]
watch_debounce_ms = 500
```

---

### 1.3 Command Templates
**Effort**: 3-4 hours
**Priority**: Medium
**Dependencies**: None

**Features**:
- Variable substitution in commands
- Environment-aware templates
- Prompt for missing variables
- Template validation

**Implementation**:
```toml
[cmd.deploy]
default = "kubectl apply -f k8s/{env}.yaml"
vars = ["env"]

[cmd.run]
default = "{runtime} {entrypoint} --port {port}"
vars = { runtime = "node", entrypoint = "index.js", port = "3000" }
```

---

## Phase 2: Core Extensions (Weeks 3-4)

### 2.1 devkit-ext-secrets
**Effort**: 10-15 hours
**Priority**: High
**Dependencies**: Command templates (optional)

**Providers**:
- AWS Secrets Manager
- HashiCorp Vault
- 1Password CLI
- Doppler
- Azure Key Vault
- GCP Secret Manager
- Local encrypted file

**Features**:
- `devkit secrets pull` - Fetch secrets to `.env`
- `devkit secrets push` - Upload local secrets
- `devkit secrets list` - Show available secrets
- `devkit secrets rotate` - Rotate secrets
- `devkit secrets audit` - Show access logs
- Template support: `{secret:aws:db-password}`

**Configuration**:
```toml
[secrets]
provider = "aws"  # or "vault", "1password", "doppler"
region = "us-east-1"
prefix = "myapp/"

[secrets.mapping]
DATABASE_URL = "aws:myapp/db-url"
API_KEY = "1password:vault/api-key"
```

---

### 2.2 devkit-ext-cache
**Effort**: 6-8 hours
**Priority**: Medium
**Dependencies**: None

**Features**:
- `devkit cache clean` - Clear all build caches
- `devkit cache clean --target` - Clean specific cache
- `devkit cache stats` - Show cache sizes
- `devkit cache prune` - Remove old cache entries
- Auto-detect cache locations (cargo, npm, gradle, maven, etc.)

**Configuration**:
```toml
[cache]
locations = [
  "target/",
  "node_modules/",
  ".gradle/",
  "~/.cargo/registry",
]
max_age_days = 30
max_size_gb = 10
```

---

### 2.3 devkit-ext-security
**Effort**: 8-10 hours
**Priority**: Medium
**Dependencies**: None

**Features**:
- `devkit security scan` - Full security scan
- `devkit security deps` - Dependency vulnerabilities
- `devkit security secrets` - Find exposed secrets
- `devkit security licenses` - License compliance
- `devkit security sbom` - Generate SBOM

**Integrations**:
- cargo audit
- npm audit / yarn audit
- snyk
- gitleaks / trufflehog
- SPDX/CycloneDX for SBOM

---

## Phase 3: Advanced Extensions (Weeks 5-6)

### 3.1 devkit-ext-k8s
**Effort**: 12-15 hours
**Priority**: Medium
**Dependencies**: Command templates

**Features**:
- `devkit k8s status` - Cluster and pod status
- `devkit k8s logs [pod]` - Stream logs
- `devkit k8s shell [pod]` - Interactive shell
- `devkit k8s port-forward` - Port forwarding
- `devkit k8s deploy` - Deploy manifests
- `devkit k8s scale` - Scale deployments
- `devkit k8s restart` - Restart pods
- Context management

**Configuration**:
```toml
[k8s]
context = "minikube"
namespace = "default"
manifests = "k8s/"

[k8s.port_forwards]
api = { pod = "api-*", ports = ["8080:8080"] }
postgres = { service = "postgres", ports = ["5432:5432"] }
```

---

### 3.2 devkit-ext-watch
**Effort**: 8-10 hours
**Priority**: Medium
**Dependencies**: Generic --watch flag

**Features**:
- Multi-file pattern watching
- Conditional rebuilds
- Browser live reload
- Notification on completion
- Smart ignoring (git-ignored files)
- Multiple watchers in parallel

**Configuration**:
```toml
[watch.backend]
patterns = ["src/**/*.rs"]
command = "cargo build"
notify = true

[watch.frontend]
patterns = ["ui/**/*.{ts,tsx,css}"]
command = "npm run build"
reload_browser = true
```

---

### 3.3 devkit-ext-monitoring
**Effort**: 10-12 hours
**Priority**: Low
**Dependencies**: Docker extension

**Features**:
- `devkit monitoring up` - Start monitoring stack
- Pre-configured Prometheus + Grafana
- Application metrics collection
- Log aggregation (Loki)
- Pre-built dashboards
- Alert configuration

**Stack**:
- Prometheus (metrics)
- Grafana (visualization)
- Loki (logs)
- Tempo (traces)

---

## Phase 4: Developer Experience (Weeks 7-8)

### 4.1 devkit init
**Effort**: 6-8 hours
**Priority**: High
**Dependencies**: None

**Features**:
- Interactive project setup wizard
- Detect existing tools/frameworks
- Generate `.dev/config.toml`
- Generate package `dev.toml` files
- Common templates (Rust, Node, Python, Go, etc.)
- Git integration

**Flow**:
```bash
$ devkit init
? Project name: my-app
? Project type: (detected: Rust workspace)
? Packages: packages/*, apps/*
? Docker: Yes
? Database: PostgreSQL
✓ Created .dev/config.toml
✓ Created packages/api/dev.toml
✓ Added docker-compose.yml template
```

---

### 4.2 Visual Dashboard (TUI)
**Effort**: 15-20 hours
**Priority**: Medium
**Dependencies**: None

**Features**:
- Split-pane terminal UI (using `ratatui`)
- Live service status panel
- Log streaming panel
- Resource usage (CPU/memory)
- Command palette
- Keyboard shortcuts
- Mouse support

**Panels**:
- Services (Docker, databases, etc.)
- Running commands
- System resources
- Recent logs
- Quick actions

**Command**: `devkit dashboard`

---

### 4.3 Remote Development Support
**Effort**: 12-15 hours
**Priority**: Medium
**Dependencies**: None

**Features**:
- SSH connection management
- File synchronization (rsync/watchexec)
- Remote command execution
- Port forwarding
- Remote environment setup
- Context switching (local/remote)

**Configuration**:
```toml
[remote.staging]
host = "staging.example.com"
user = "deploy"
path = "/app"
sync_patterns = ["src/**", "Cargo.toml"]
port_forwards = ["8080:8080", "5432:5432"]
```

**Commands**:
```bash
devkit remote connect staging
devkit remote sync
devkit remote exec "cargo build"
devkit remote forward
```

---

## Implementation Priority Matrix

### High Priority (Do First)
1. ✨ **Auto-update checker** (2-3h) - Foundation for updates
2. **Generic --watch flag** (4-6h) - Enables many other features
3. **devkit init** (6-8h) - Critical for adoption
4. **devkit-ext-secrets** (10-15h) - High user value

### Medium Priority (Do Next)
5. **Command templates** (3-4h) - Enhances flexibility
6. **devkit-ext-cache** (6-8h) - Performance improvement
7. **devkit-ext-security** (8-10h) - Important for production
8. **devkit-ext-k8s** (12-15h) - Cloud-native support
9. **devkit-ext-watch** (8-10h) - Developer experience

### Lower Priority (Nice to Have)
10. **devkit-ext-monitoring** (10-12h) - Advanced use case
11. **Visual Dashboard** (15-20h) - Polish
12. **Remote development** (12-15h) - Specific use case

---

## Quick Wins (Bonus Features)

These can be implemented quickly between larger features:

### Command Aliases (1-2h)
```toml
[aliases]
t = "test"
b = "build"
d = "docker"
```

### Notification System (2-3h)
Desktop notifications for command completion:
```bash
devkit cmd build --notify
```

### Output Formatting (3-4h)
```bash
devkit status --format json
devkit docker ps --format table
```

### Command History (2-3h)
```bash
devkit history
devkit history --search test
devkit !!  # Re-run last command
```

---

## Dependencies Graph

```
Auto-Update Checker (none)
    ↓
Generic --watch Flag (none)
    ↓
devkit-ext-watch

Command Templates (none)
    ↓
devkit-ext-k8s
devkit-ext-secrets

devkit init (none)

devkit-ext-cache (none)
devkit-ext-security (none)

Docker extension (existing)
    ↓
devkit-ext-monitoring

Visual Dashboard (none)
Remote Development (none)
```

---

## Estimated Timeline

**Total**: ~8 weeks for all features

- **Week 1-2**: Foundation (auto-update, --watch, templates)
- **Week 3-4**: Core extensions (secrets, cache, security)
- **Week 5-6**: Advanced extensions (k8s, watch, monitoring)
- **Week 7-8**: DX improvements (init, dashboard, remote)

---

## Success Metrics

- Time saved in typical dev workflow
- Number of manual steps eliminated
- User adoption rate
- GitHub stars and community engagement
- Issue resolution time
- Extension usage statistics

---

## Next Steps

1. ✅ Implement auto-update checker
2. Gather community feedback on roadmap
3. Create issues for each feature
4. Set up project board
5. Begin Phase 1 implementation

---

*Last updated: 2026-01-29*
