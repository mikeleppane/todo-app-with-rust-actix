#!/bin/sh
export DATABASE_URL=postgres://username:password@postgres/to_do
diesel migration run
./target/debug/todo-app config.yml
