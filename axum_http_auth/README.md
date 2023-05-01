# axum http auth

- https://github.com/i0n/axum-http-auth-example


## Libraries

- database: bb8 bb8-postgres tokio-postgre
- session: axum-sessions async-redis-session
- password hash: rust-argon2
- template: askama
- validation: validator

## Run

Start postgres and redis servers

```
docker-compose up -d
```

Start axum http server

```
cargo run
```
