# Todo Application API with Rust + Actix Web

## What is it

Simple todo-app

## How to use it

This project has a Makefile with the following goals:

- `make db`: Starts ScyllaDB using docker-compose. Make sure it is installed in your machine.
- `make stop`: Stops ScyllaDB.
- `make bld`: Builds the application.
- `make run`: Runs application.
- `make build`: Builds docker image.
- `make push`: Pushes docker image.

## REST API

### Data Ingestion

#### POST /ingest

Triggers the ingestion process from one or more files:

Body:

```
{
    "ingestion_id": "test",
    "files": [
        "s3://rust-s3-scylladb/data_example.json"
    ]
}
```

- `ingestion_id` is a unique ID that you can use to identify a single ingestion.
- `files`: contains a list of files to be processed.

The idea is that you will deploy multiple replicas of this service to run in parallel.
I have included a small [Python Script](/job/ingestion_job.py) that you can use to read from a bucket and call this REST
API to distribute the load.

#### GET /node/{id}

Get a specific node by ID.

Query Parameters:

- `get_tags`: If `true` it will also return the tags.
- `get_relations`: If `true` it will also return the relations.


