# various-web-frameworks-rust

This is composed of various web frameworks such as [actix-web](https://actix.rs/docs/) and [axum](https://github.com/tokio-rs/axum) in workspace. It's easy to try various frameworks.

## Options to implement

- [actix-web](https://github.com/actix/actix-web)
- [axum](https://github.com/tokio-rs/axum)

## Workspace

[crates directory](./crates)

| crate      | type | explanation                                        |
| ---------- | ---- | -------------------------------------------------- |
| actix      | bin  | actix-web framework                                |
| axum       | bin  | [WIP] axum web framework                           |
| components | lib  | framework common packages depended from bin crates |

## TODO

### common

- [diesel](https://diesel.rs/)

### axum

- [x] middleware implementation
- [x] when body is blank, request doesn't reach to handler though middleware passed.
  - It happens on test `./scripts/admin.hurl:101:6`, 415 returns thought 400 expected
- [ ] open api spec generation from code
  - [aide](https://crates.io/crates/aide) looks good!

## References

- [Web Framework Benchmarks](https://www.techempower.com/benchmarks/#hw=ph&test=fortune&section=data-r22)
