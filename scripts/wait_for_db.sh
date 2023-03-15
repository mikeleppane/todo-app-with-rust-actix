#!/bin/bash

set -x
set -eo pipefail

if ! [ -x "$(command -v pg_isready)" ]; then
  echo >&2 "Error: pg_isready is not installed."
  exit 1
fi

cd ..
docker-compose up -d
until pg_isready -h localhost -p 5432 -U username; do
  echo "Waiting for postgres"
  sleep 2
done
echo "docker is now running"
docker-compose down
