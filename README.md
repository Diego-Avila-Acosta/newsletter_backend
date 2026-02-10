# Newsletter Backend

## Pre-requisites

 - [Rust]()
 - [Docker]()
 - [sqlx-cli](https://crates.io/crates/sqlx-cli)
 - postgresql-client

### Linux

```sh
# Ubuntu 
sudo apt-get install lld clang libssl-dev postgresql-client
# Arch 
sudo pacman -S lld clang postgresql
```

```sh
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
```

## How to run

Launch a Postgres and Redis database via Docker:

```sh
./scripts/init_db.sh
./scripts/init_redis.sh.sh
```

Launch backend:

```sh
cargo run --released
```

After the backend started you can visit `http://127.0.0.1:8000/login`. Default user for testing: `admin` with password `12345678`

Available entrypoints are listed in [src/startup.rs](https://github.com/Diego-Avila-Acosta/newsletter_backend/blob/main/src/startup.rs#L114)

### With Docker Compose

A docker compose file is available at the root project. This compose file specifies the following services:

- Newsletter backend
- Postgres database (profile: `database`)
- Redis database    (profile: `redis`)
- Prometheus        (profile: `observability`)
- Jaeger            (profile: `observability`)
- Grafana           (profile: `observability`)
- cAdvisor          (profile: `cadvisor`)

Profiles are used with the `COMPOSE_PROFILES` environment variable in order to specify which services to launch with the newsletter backend.

Before running docker compose, a `.env` file **MUST** be created with the following variables:

|Name|Usage|Example|
|---|---|---|
|`POSTGRES_USER`|Postgres admin username|`postgres`|
|`POSTGRES_PW`|Postgres admin password|`12345678`|
|`POSTGRES_DB`|Postgres database name created on launch|`newsletter`|
|`COMPOSE_PROFILES` (Optional)|Profiles to launch with compose|`database,redis`|

#### Examples

Running with databases:

```sh
COMPOSE_PROFILES=database,redis docker compose up -d
```

Running with all services:

```sh
COMPOSE_PROFILES=database,redis,observability,cadvisor docker compose up -d
```

Or by specifying `COMPOSE_PROFILES` in .env file:

```sh
docker compose up -d
```

## Configuration

This project manages settings through a layered configuration based on the active environment. The backend relies on a base configuration which is then merged with environment-specific overrides.

### Configuration Files

All files are located inside the `configuration/` directory.

* **`base.yaml`**: Contains all default settings the backend depends on.
* **Environment Overrides**: Specific settings that take precedence over `base.yaml` based on the active environment:
    * `dev.yaml`
    * `docker.yaml`
    * `production.yaml`

To run with a specific environment, set the `APP_ENVIRONMENT` variable when executing the application.

**Example:**
```bash
APP_ENVIRONMENT=production cargo run
```

**By default the backend will run with the `dev` environment**

## Used libraries

- actix-web
- actix-web-flash-messages
- actix-web-metrics
- actix-session
- anyhow
- argon2
- config
- chrono
- metrics-exporter-prometheus
- opentelemetry
- reqwest
- serde
- sqlx
- secrecy
- thiserror
- tokio
- tracing
- tracing-bunyan-formatter
- validator

### Dev libraries:

- tokio
- quickcheck
- fake
- wiremock
- linkify
- claims
- once_cell
- serde_json


## Credits

This project is based on the book *[Zero To Production in Rust](zero2prod.com)* By [Luca Palmieri](lpalmieri.com)
