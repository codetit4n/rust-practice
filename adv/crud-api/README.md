# CRUD API

A very basic CRUD API in rust containerized using docker.

Ref: https://www.youtube.com/watch?v=vhNoiBOuW94

### How to use the project?

#### Run postgres db

```bash
docker compose up -d db
```

#### Build

```bash
docker compose build
```

#### Run

```bash
docker compose up rustcrudapi
```

### API docs

- GET `/status`: Get the current status of the API. Wheather it is running or not.
- GET `/users`: Get all the users.
- POST `/users`: Add a new user. Pass in raw json in the format:
  ```json
  {
    "name": "lokesh",
    "email": "hello@lokeshkr.com"
  }
  ```
- GET `/users/{id}`: Get a particular user using id.
- PUT `/users/{id}`: Update a particular user using id.
- DELETE `/users/{id}`: Delete a particular user using id.
- Any other route: 404 NOT Found
