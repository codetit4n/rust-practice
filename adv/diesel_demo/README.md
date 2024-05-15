# Diesel Demo

Ref: https://diesel.rs/guides/getting-started

## How to run?

### Run a PostgreSQL database in a Docker container

```bash
docker run --rm -P -p 127.0.0.1:[CHOOSE A PORT]:5432 -e POSTGRES_PASSWORD=[CHOOSE A PASSWORD] --name pg postgres:alpine
```

> NOTE: Replace `[CHOOSE A PASSWORD]` with a password of your choice and `[CHOOSE A PORT]` with a port of your choice.

### Add the diesel db url to the `.env` file. Refer to the `.env.example` file.

### Some useful scripts

- Write a new article: `cargo run --bin write_post`
- Save a post: `cargo run --bin save_post` (You will need to write a post first, which is saved as a draft).
- Show the saved posts: `cargo run --bin show_posts`
- Delete a post: `cargo run --bin delete_post [POST ID]` (Post ID is incremental starting from 1).
