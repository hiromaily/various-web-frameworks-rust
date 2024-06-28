#------------------------------------------------------------------------------
# main
#------------------------------------------------------------------------------

.PHONY: lint
lint:
	cargo fmt --all --check
	cargo clippy --all-targets --all-features

.PHONY: check-deps
check-deps:
	cargo machete

.PHONY: fix
fix:
	cargo fix --allow-staged

.PHONY: build
build:
	cargo build

.PHONY: build-release
build-release:
	cargo build --release

# hash crate is pbkdf2
.PHONY: run
run:
	RUST_LOG=debug cargo run -- ./config/local.toml -d

# hash crate is argon2
.PHONY: run-argon2
run-argon2:
	RUST_LOG=debug cargo run --no-default-features --features "argon2" -- ./config/local.toml -d

# hash crate is scrypt
.PHONY: run-scrypt
run-scrypt:
	RUST_LOG=debug cargo run --no-default-features --features "scrypt" -- ./config/local.toml -d

.PHONY: compile
compile:
	rustc ./src/main.rs

.PHONY: test
test:
	cargo test

.PHONY: update-deps
update-deps:
	cargo machete

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
	sea-orm-cli generate entity -u postgresql://admin:admin@127.0.0.1:5432/example -o src/schemas --with-serde both

#------------------------------------------------------------------------------
# docker
#------------------------------------------------------------------------------
.PHONY: build-image
build-image:
	docker compose build --progress=plain

.PHONY: build-image-server
build-image-server:
	docker compose build --progress=plain server

.PHONY: up-db
up-db:
	docker compose up postgresql

.PHONY: up-web
up-web:
	docker compose up server

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
#------------------------------------------------------------------------------
.PHONY: req
req:
	hurl --very-verbose ./scripts/admin.hurl
	hurl --very-verbose ./scripts/app.hurl
	

.PHONY: req-sh
req-sh:
	./scripts/req.sh

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
