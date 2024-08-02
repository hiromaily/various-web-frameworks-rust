#PQ_LIB_DIR := $(shell brew --prefix libpq)/lib
#PQ_LIB_DIR := /usr/lib/aarch64-linux-gnu
	
UNAME_S := $(shell uname -s)
UNAME_P := $(shell uname -m) # e.g. aarch64
ifeq ($(UNAME_S),Darwin) # MacOS
	PQ_LIB_DIR := $(shell brew --prefix libpq)/lib
else ifeq ($(UNAME_S),Linux) # Linux
	PQ_LIB_DIR := /usr/lib/$(UNAME_P)-linux-gnu
endif

.PHONY: update-rustc
update-rustc:
	rustup update stable

#------------------------------------------------------------------------------
# main
#------------------------------------------------------------------------------
echo:
	echo $(PQ_LIB_DIR)

.PHONY: lint
lint:
	cargo fmt --all
	cargo clippy --all-targets --all-features

.PHONY: check-deps
check-deps:
	cargo machete

.PHONY: fix
fix:
	cargo fix --allow-staged

# [Problem]
# when using Diesel, you may face error
# - error: linking with `cc` failed
# - ld: library not found for -lpq
# - clang: error: linker command failed with exit code 1
# [Solved]
# set environment variable
# ```zsh
# export PQ_LIB_DIR=/opt/homebrew/opt/libpq/lib
# export PKG_CONFIG_PATH=/opt/homebrew/opt/libpq/lib/pkgconfig
# export PATH="$PATH:/opt/homebrew/opt/libpq/bin"
# ```
# then run `cargo clean` before building
.PHONY: build
build:
	cargo build

.PHONY: clean-build
clean-build: clean
	cargo build

.PHONY: build-all
build-all:
	cargo build --no-default-features --features "argon2"
	cargo build --no-default-features --features "scrypt"
	cargo build

.PHONY: build-release
build-release:
	cargo build --release

.PHONY: compile
compile:
	rustc ./src/main.rs

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	cargo clean

#------------------------------------------------------------------------------
# execute actix
#------------------------------------------------------------------------------
# hash crate is pbkdf2
.PHONY: run
run:
	RUST_LOG=debug cargo run --package actix -- ./config/local.toml -d

# hash crate is argon2
.PHONY: run-argon2
run-argon2:
	RUST_LOG=debug cargo run --package actix --no-default-features --features "argon2" -- ./config/local.toml -d

# hash crate is scrypt
.PHONY: run-scrypt
run-scrypt:
	RUST_LOG=debug cargo run --package actix --no-default-features --features "scrypt" -- ./config/local.toml -d

.PHONY: run-openapi
run-openapi:
	RUST_LOG=debug cargo run --package actix --features "openapi" -- ./config/local.toml -d

# run actix-web with postgresql
.PHONY: run-actix-db
run-actix-db:
	docker compose up -d postgresql actix-server

# docker compose exec -it actix-server sh

#------------------------------------------------------------------------------
# execute axumfw
#------------------------------------------------------------------------------
.PHONY: run-axumfw
run-axumfw:
	RUST_LOG=debug cargo run --package axumfw -- ./config/local.toml -d

#------------------------------------------------------------------------------
# diesel cli
# Refer to
# - https://diesel.rs/guides/getting-started
#------------------------------------------------------------------------------
# prerequirements:
# - `brew install libpq` 
# - create `.env` with your DATABASE_URL
# refer to: https://github.com/sgrif/pq-sys
.PHONY: setup-diesel-postgresql
setup-diesel-postgresql:
	PQ_LIB_DIR=$(PQ_LIB_DIR) cargo install diesel_cli --no-default-features --features postgres
	cargo install diesel_cli_ext
	diesel setup

.PHONY: generate-diesel-entity-from-db
generate-diesel-entity-from-db:
	mkdir -p crates/components/src/schemas/diesel
	diesel print-schema > crates/components/src/schemas/diesel/schema.rs

.PHONY: generate-diesel-model
generate-diesel-model:
	diesel_ext generate schema > crates/components/src/schemas/diesel/model.rs

#------------------------------------------------------------------------------
# sea-orm
# Refer to
# - https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/
#------------------------------------------------------------------------------
.PHONY: setup-sea-orm
setup-sea-orm:
	cargo install sea-orm-cli

# https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html
#.PHONY: migrate-db
# migrate-db:
# 	sea-orm-cli migrate init
# 	rm -rf ./migration/src/m20220101_000001_create_table.rs
# 	@echo create table
# 	sea-orm-cli migrate generate create_table_users

# https://www.sea-ql.org/sea-orm-tutorial/ch01-04-entity-generation.html
#
# FIXME: by shell script
# For now, generated code is modified for apistos, adding ApiComponent, JsonSchema
# ```
# use apistos::ApiComponent;
# use schemars::JsonSchema;
#
# #[derive(
#    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ApiComponent, JsonSchema,
#)]
# ```
#
.PHONY: generate-entity-from-db
generate-entity-from-db:
	rm -rf src/schemas
	sea-orm-cli generate entity -u postgresql://admin:admin@127.0.0.1:5432/example -o crates/components/src/schemas/sea_orm --with-serde both

#------------------------------------------------------------------------------
# docker
#------------------------------------------------------------------------------
# docker version 4.33+
.PHONY: check-dockerfile
check-dockerfile:
	docker build -f ./docker/Dockerfile_rust . --check
	docker build -f ./docker/Dockerfile_pg . --check

.PHONY: build-image
build-image:
	docker compose build --progress=plain

.PHONY: build-image-server
build-image-server:
	docker compose build --progress=plain actix-server
	#docker compose build --progress=plain axum-server

.PHONY: up-db
up-db:
	docker compose up postgresql

.PHONY: up-actix
up-actix:
	docker compose up actix-server

.PHONY: reset-db
reset-db:
	docker compose down -v
	docker compose up postgresql


# docker container exec -it {container_id} bash
#  or
# docker container exec -it actix-web-postgresql bash
# 
# > psql -U postgres example
# > \d users

#------------------------------------------------------------------------------
# Test Server
# - `hurl --verbose`
# - `hurl --very-verbose`
# - `hurl --variable invalid_body_status=400`
#------------------------------------------------------------------------------
.PHONY: req
req:
	hurl --very-verbose ./scripts/admin.hurl
	hurl --very-verbose ./scripts/app.hurl

# .PHONY: req-sh
# req-sh:
# 	./scripts/req.sh

.PHONY: req-check-endpoint
req-check-endpoint:
	hurl --very-verbose ./scripts/check_only_endpoint.hurl

.PHONY: get-token
get-token:
	curl -s curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"email":"john.doe@example.com", "password":"password1234"}' http://127.0.0.1:8080/api/v1/admin/login | jq -r '.token'

# This runs a benchmark for 30 seconds, using 12 threads, and keeping 400 HTTP connections open.
# for now, token needs to be updated at ./scripts/auth.lua
.PHONY: bench-http
bench-http:
	wrk -t12 -c400 -d30s -s ./scripts/auth.lua http://127.0.0.1:8080/api/v1/admin/users

# .PHONY: bench
# bench:
# 	cargo bench

.PHONY: httpstat
httpstat:
	httpstat -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MTg4NTkyMTEsImV4cCI6MTcxODg2MjgxMSwibmJmIjoxNzE4ODU5MjExLCJ1c2VyX2lkIjoxLCJlbWFpbCI6ImpvaG4uZG9lQGV4YW1wbGUuY29tIiwiaXNfYWRtaW4iOnRydWV9.TGQsrzGk57Fh2HNLnuztO7NMxbwljH-y-uDNbzP4SSk' http://127.0.0.1:8080/api/v1/admin/users


#------------------------------------------------------------------------------
# Monitoring
#------------------------------------------------------------------------------

.PHONY: tcpdump
tcpdump:
	sudo tcpdump -i lo0 port 8080 -vv
