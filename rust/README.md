# Rust workspace

Axum HTTP app split into a library crate (`api`) and a binary crate (`server`).

## Running and testing

Human contributors should install [`bacon`](https://dystroy.org/bacon/) and
[`cargo-nextest`](https://nexte.st/), then use the configured bacon jobs:

```bash
cargo install --locked bacon cargo-nextest
bacon          # Run the server
```

Press `t` in bacon to run the test suite with nextest.

AI agents should use non-interactive Cargo commands instead:

```bash
cargo run -p server
cargo test --workspace
```

## `api`

Owns the HTTP application surface: routes, handlers, and the shared `AppState`.
`AppState` contains the Postgres connection pool and is passed to
`api::router(state)`.

- Reused by the server binary and by tests
- Supports black-box HTTP API tests via `api::router(state)` + `tower::ServiceExt::oneshot` without binding a TCP port
- Handlers can also be called directly for unit-style checks
- `/health` checks that the HTTP process is responsive
- `/live` executes `SELECT 1` through the pool, returning `503 Service Unavailable`
  when Postgres cannot be reached

```bash
cargo test -p api
```

The API live tests require the repository's Postgres instance to be running.
Health/live API tests and the server TCP smoke test use
[`expect-test`](https://docs.rs/expect-test) snapshots (`expect![[...]]`).
When a snapshot is intentionally out of date, regenerate the expected literals with:

```bash
UPDATE_EXPECT=1 cargo test --workspace
```

That rewrites the `expect![[...]]` strings in source to match the actual output.

## `database`

Owns the Postgres connection pool and embedded SQL migrations. `database::setup`
connects and runs all pending migrations, while `database::connect` and
`database::migrate` can be used separately.

The server passes the connection URL to `database::setup`. It reads
`DATABASE_URL` when set and otherwise uses the Postgres instance in the
repository's `compose.yml`:

```text
postgres://postgres:postgres@localhost:5432/app
```

Start Postgres from the repository root before running the integration tests:

```bash
podman compose up -d
cd rust
cargo test -p database
```

## `server`

Process entrypoint: initializes the Postgres pool and migrations, builds the API
state, binds `127.0.0.1:3000`, and serves `api::router(state)` via
`server::run(listener, state)`. Route logic lives in `api`, not here.

A TCP smoke test binds an ephemeral port, spawns `server::run`, and HTTP GETs
`/`, `/health`, and `/live`. Start Postgres before running or testing the
server.

```bash
cargo run -p server
cargo test -p server
```

Both the server and the API integration tests go through the same `api::router()`.
