# axum-sqlx

- https://github.com/launchbadge/realworld-axum-sqlx (axum 0.3.4)
- https://github.com/davidpdrsn/realworld-axum-sqlx (axum 0.6.0)

## Run

start postgres server

```
docker compose up -d

docker compose exec postgres psql -U postgres axum_sqlx
```

run server

```
cargo run
```

requests users

```
-- create user
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"user":{"username":"john_doe","email":"john@example.com","password":"password123"}}' \
    http://localhost:8000/api/users

-- login user
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"user":{"email":"john@example.com","password":"password123"}}' \
    http://localhost:8080/api/users/login

-- get current user
curl -X GET \
    -H "Content-Type: application/json" \
    -H "Authorization: Token your_token_here" \
    http://localhost:8080/api/user

-- update current user
curl -X PUT \
    -H "Content-Type: application/json" \
    -H "Authorization: Token your_token_here" \
    -d '{"user":{"bio":"hello"}}' \
    http://localhost:8080/api/user
```
