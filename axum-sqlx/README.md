# axum-sqlx

- https://github.com/launchbadge/realworld-axum-sqlx

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

requests

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
```
