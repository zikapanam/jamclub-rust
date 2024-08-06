#!/bin/bash

cd src/entities && sea-orm-cli generate entity --database-url=postgres://postgres:example@localhost:5432/seaorm
