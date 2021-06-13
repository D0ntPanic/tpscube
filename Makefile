.PHONY: native web server

native:
	cargo build --release

web:
	cargo build --release --lib --target wasm32-unknown-unknown --no-default-features --features wasm
	wasm-bindgen target/wasm32-unknown-unknown/release/tpscube.wasm --out-dir web --no-modules --no-typescript

server: web
	cd web && python3 -m http.server

deploy: web
	aws s3 cp --profile personal web/index.html s3://tpscube/index.html
	aws s3 cp --profile personal web/tpscube.js s3://tpscube/tpscube.js
	aws s3 cp --profile personal web/tpscube_bg.wasm s3://tpscube/tpscube_bg.wasm
