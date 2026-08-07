# best-practice

## Local database

Use [Podman](https://podman.io/) (not Docker) to run Postgres from [`compose.yml`](compose.yml):

```bash
podman compose up -d
```

Stop it with:

```bash
podman compose down
```

Connection string: `postgres://postgres:postgres@localhost:5432/app`
