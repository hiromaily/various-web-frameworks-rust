.PHONY: run
run:
	cargo run

.PHONY: req
req: req-get req-post

.PHONY: req-get
req-get:
	curl http://127.0.0.1:8080/

.PHONY: req-post
req-post:
	curl -X POST http://127.0.0.1:8080/submit -d "key=value"
