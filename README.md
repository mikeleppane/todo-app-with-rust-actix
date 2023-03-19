# Todo Application API with Rust + Actix Web

## What is it

Simple todo-app

## How to use it

The project run inside a docker compose. Make sure you have docker and docker compose installed (and .env defined, see
the next section). After that simply run
the following command:

```
docker-compose run
``` 

## Used Technologies

- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [Diesel](https://diesel.rs/)
- Docker
- PostgreSQL (db)
- Redis (cache)

## Environment Variables

Make sure there's .env file defined at the root of the project. Below is defined all the required envs:

- DATABASE_URL
- POSTGRES_USER
- POSTGRES_DB
- POSTGRES_PASSWORD
- SECRET_KEY

Example:

```
DATABASE_URL=postgres://username:password@postgres/to_do
POSTGRES_USER=username
POSTGRES_DB=to_do
POSTGRES_PASSWORD=password
SECRET_KEY="top secret"
```

## REST API

### Authentication

#### POST /v1/auth/login

Triggers the ingestion process from one or more files:

Body:

```
{
    "username": "user",
    "password": "my-password"
}
```

- `ingestion_id` is a unique ID that you can use to identify a single ingestion.
- `files`: contains a list of files to be processed.

#### GET /v1/auth/logout

### Users

#### POST /v1/user/create

Body:

```
{
    "name": "user",
    "email": "valid-email@gmail.com",
    "password": "my-password"
}
```

### Todos

#### POST /v1/item/create/{title}

Header:

```
{
    "token": "valid token",
}
```

#### GET /v1/item/get

Header:

```
{
    "token": "valid token",
}
```

Returns all users todos

#### POST /v1/item/edit

Body:

```
{
    "title": "todo item title",
    "status": "",
}
```

Set a todo item's status to DONE with the given title

Header:

```
{
    "token": "valid token",
}
```

#### POST /v1/item/delete

```
{
    "title": "todo item title",
    "status": "",
}
```

Deletes a todo item with the given title

Header:

```
{
    "token": "valid token",
}
```

