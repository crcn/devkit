# devkit-ext-docker

Docker and Docker Compose extension for devkit.

## Features

- Docker Compose orchestration (up, down, restart, build)
- Container log following with auto-reconnect
- Interactive shell access
- Service management
- Auto-detection of docker-compose.yml

## Auto-Enable

This extension automatically enables when:
- `docker` or `docker-compose` command is available
- `docker-compose.yml` (or variants) exists in project root

## Usage

### As part of devkit-cli

Already included - just run:
```bash
devkit docker up
devkit logs api
devkit shell postgres
```

### As a library

```rust
use devkit_core::AppContext;
use devkit_ext_docker::{compose_up, logs_follow};

fn main() -> anyhow::Result<()> {
    let ctx = AppContext::new(false)?;

    if ctx.features.docker {
        compose_up(&ctx, &[], false)?;
    }

    Ok(())
}
```

## Commands

- `docker up` - Start containers
- `docker down` - Stop containers
- `docker restart` - Restart containers
- `docker build` - Build images
- `docker logs <service>` - Follow logs
- `docker shell <service>` - Open shell
- `docker ps` - List containers
