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
# diesel migration generate create_users
# diesel migration run
# diesel migration redo
```

run server

```
cargo run
```

## Requests

users

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
    -d '{"user":{"bio":"hi"}}' \
    http://localhost:8080/api/user
```

profiles

```sh
# get profile
curl -X GET \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    http://localhost:8080/api/profiles/tom

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

articles

```sh
# get articles
curl -X GET \
    -H "Content-Type: application/json" \
    http://localhost:8080/api/articles

curl -X GET \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    http://localhost:8080/api/articles/feed

# get article by slug
curl -X GET \
    -H "Content-Type: application/json" \
    http://localhost:8080/api/articles/first-article

# create article
curl -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    -d '{"article":{"title":"first article", "description":"this is description", "body":"body", "tagList":["a1","b1"]}}' \
    http://localhost:8080/api/articles

# update article
curl -X PUT \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    -d '{"article":{"body":"updated body"}}' \
    http://localhost:8080/api/articles/first-article

# delete article
curl -X DELETE \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    http://localhost:8080/api/articles/some-article
```

comments

```sh
# get article comments
curl -X GET \
    -H "Content-Type: application/json" \
    http://localhost:8080/api/articles/first-article/comments

# create article comment
curl -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    -d '{"comment":{"body":"this is comment of first"}}' \
    http://localhost:8080/api/articles/first-article/comments

# delete article comment
curl -X DELETE \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    -d '{"comment":{"body":"this is comment of first"}}' \
    http://localhost:8080/api/articles/first-article/comments/64c07418-213d-497a-8938-6902ebad6e84
```

tags

```sh
# get tags
curl -X GET \
    -H "Content-Type: application/json" \
    http://localhost:8080/api/tags
```

favorites

```sh
# create favorite
curl -X PUT \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $YOUR_TOKEN" \
    -d '{"article":{"body":"updated body"}}' \
    http://localhost:8080/api/articles/first-article

# delete favorite
```
