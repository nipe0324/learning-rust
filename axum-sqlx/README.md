# axum-sqlx

- https://github.com/launchbadge/realworld-axum-sqlx (axum 0.3.4)
- https://github.com/davidpdrsn/realworld-axum-sqlx (axum 0.6.0)

## Run

start postgres server

```sh
docker compose up -d

# psql
docker compose exec postgres psql -U postgres axum_sqlx
```

run server

```sh
cargo run
```

requests users

```sh
# create user
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"user":{"username":"john_doe","email":"john@example.com","password":"password123"}}' \
    http://localhost:8080/api/users

# login user
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"user":{"email":"john@example.com","password":"password123"}}' \
    http://localhost:8080/api/users/login

# get current user
curl -X GET \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    http://localhost:8080/api/user

# update current user
curl -X PUT \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    -d '{"user":{"bio":"hello"}}' \
    http://localhost:8080/api/user
```

requests profiles

```sh
# get user profile
curl -X GET \
    -H "Content-Type: application/json" \
    http://localhost:8080/api/profiles/john_doe

# follow
curl -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    http://localhost:8080/api/profiles/tom/follow

# unfollow
curl -X DELETE \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    http://localhost:8080/api/profiles/tom/follow
```

requests articles

```sh
# create article
curl -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    -d '{"article":{"title":"my first article","description":"this is description.","body":"body","tagList":["tag1","tag2"]}}' \
    http://localhost:8080/api/articles

# list articles
curl -X GET \
    -H "Content-Type: application/json" \
    http://localhost:8080/api/articles
```
