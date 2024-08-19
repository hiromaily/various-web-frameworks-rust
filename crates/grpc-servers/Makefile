# gen code from build.rs as well
.PHONY: build
build:
	cargo build

.PHONY: run-server
run-server:
	RUST_LOG=debug cargo run --bin grpc-server

.PHONY: run-client
run-client:
	RUST_LOG=debug cargo run --bin grpc-client

.PHONY: req
req:
	grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051' helloworld.Greeter/SayHello
