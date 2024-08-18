.PHONY: run
run:
	RUST_LOG=debug cargo run -- --port 8080

.PHONY: req
req: req-get req-post

.PHONY: req-get
req-get:
	curl http://127.0.0.1:8080/
	curl 'http://127.0.0.1:8080/?foo=bar&item=5'

.PHONY: req-post
req-post:
	curl -X POST http://127.0.0.1:8080/submit -H 'Content-Type: application/json' -d '{"id":"my_id","password":"my_password"}'
	curl -X POST http://127.0.0.1:8080/submit -H 'Content-Type: application/json'
	curl -X POST http://127.0.0.1:8080/submit -H 'Content-Type: application/xml' -d '{"id":"my_id","password":"my_password"}'
	curl -X POST http://127.0.0.1:8080/submit -d '{"id":"my_id","password":"my_password"}'
