#!/bin/sh
diesel migration run
./target/release/todo-app config.yml
