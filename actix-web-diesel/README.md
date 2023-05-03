# actix-web with diesel

- https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel

Libraries

- [actix-web](https://actix.rs/): fast web framework for Rust
- [diesel](https://diesel.rs/): ORM and Query Builder for Rust

## Setup

start postgres server

```
docker compose up -d
```

setup db

```
# Install the CLI tool
cargo install diesel_cli

# create database
diesel setup

# migration commands
# diesel migration generate create_xxx
# diesel migration run
# diesel migration redo
```

run server

```
cargo run
```
