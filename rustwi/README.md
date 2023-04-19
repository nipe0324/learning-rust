# Rust Webapp Tutorial

- https://rust-webapp-tutorial.teruru.net/

## Libraries

- [axum](https://github.com/tokio-rs/axum) :  is a web application framework that focuses on ergonomics and modularity.
- [askama](https://github.com/djc/askama) : Type-safe, compiled Jinja-like templates for Rust
- [tokio](https://github.com/tokio-rs/tokio) : Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with the Rust programming language
- [serde](https://github.com/serde-rs/serde) : Serialization framework for Rust
- [dotenvy](https://github.com/allan2/dotenvy) : A well-maintained fork of the Rust dotenv crate
- [chrono](https://github.com/chronotope/chrono) : Date and time library for Rust
- [bb8](https://github.com/djc/bb8) : Full-featured async (tokio-based) postgres connection pool (like r2d2)
- [tokio-postgres](https://github.com/sfackler/rust-postgres) : A native, asynchronous PostgreSQL client.

## Setup

run postgres db

```
docker-compose up -d
```

create table

```
docker exec -i rustwi-db-1 psql -U postgres rustwi < sql/table.sql

# show talbe
docker exec -i rustwi-db-1 psql -U postgres rustwi -c '\dt'
```

run web server

```
cargo run
```
