# best-practice

See the [Rust workspace README](rust/README.md) for application setup, running,
and testing instructions.

## Local database

Use [Podman](https://podman.io/) (not Docker) to run Postgres 18 from
[`compose.yml`](compose.yml):

```bash
podman compose up -d
```

Stop it with:

```bash
podman compose down
```

Connection string: `postgres://postgres:postgres@localhost:5432/app`
