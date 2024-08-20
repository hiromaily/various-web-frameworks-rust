# various-web-frameworks-rust

This is composed of various web frameworks such as [actix-web](https://actix.rs/docs/) and [axum](https://github.com/tokio-rs/axum) in workspace. It's easy to try various frameworks.

## Options to implement

- [actix-web](https://github.com/actix/actix-web)
- [axum](https://github.com/tokio-rs/axum)

## Workspace

[crates directory](./crates)

| crate              | type | explanation                                        |
| ------------------ | ---- | -------------------------------------------------- |
| actix              | bin  | actix-web framework                                |
| axum               | bin  | axum web framework                                 |
| [WIP] web-server   | bin  | single thread web server without framework         |
| [WIP] grpc-servers | bin  | gRPC server and client                             |
| components         | lib  | framework common packages depended from bin crates |

## Run web server

- web server with actix

```sh
cargo run --package actix --features "openapi" -- ./config/local.toml -d
```

- web server with axum

```sh
cargo run --package axumfw -- ./config/local.toml -d
```

## TODO

### common

- [x] [diesel](https://diesel.rs/) implementation
- [x] integration test for diesel implementation
- maybe better to use [diesel-derive-enum](https://github.com/adwhit/diesel-derive-enum) for enum

### web server (single thread)

- [x] implement parser
- [x] implement handler
- [x] implement router
- [x] implement middleware
- [ ] implement responser
  - Error
  - HTML
  - JSON

### axum

- [x] when body is blank, request doesn't reach to handler though middleware passed.
  - It happens on test `./scripts/admin.hurl:101:6`, 415 returns thought 400 expected
- [ ] open api spec generation from code
  - [aide](https://crates.io/crates/aide) looks good!

## References

- [Web Framework Benchmarks](https://www.techempower.com/benchmarks/#hw=ph&test=fortune&section=data-r22)
