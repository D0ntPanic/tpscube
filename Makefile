.PHONY: native mac web server

native:
	cargo build --release

mac: native
	mkdir -p TPS\ Cube.app/Contents/MacOS
	mkdir -p TPS\ Cube.app/Contents/Resources
	cp Info.plist TPS\ Cube.app/Contents/
	cp target/release/tpscube TPS\ Cube.app/Contents/MacOS/
	cp images/icon.icns TPS\ Cube.app/Contents/Resources/

web:
	RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --release --lib --target wasm32-unknown-unknown --no-default-features --features wasm
	wasm-bindgen target/wasm32-unknown-unknown/release/tpscube.wasm --out-dir web --no-modules --no-typescript
	cat web/manifest.appcache.template | sed s/HASH/`dd if=/dev/urandom bs=1 count=32 | md5 -q`/ > web/manifest.appcache

server: web
	cd web && python3 -m http.server

deploy: web
	gzip -9 -k -f web/tpscube_bg.wasm
	aws s3 cp --profile personal web/index.html s3://tpscube/index.html
	aws s3 cp --profile personal web/tpscube.js s3://tpscube/tpscube.js
	aws s3 cp --profile personal web/favicon_16x16.png s3://tpscube/favicon_16x16.png
	aws s3 cp --profile personal web/favicon_32x32.png s3://tpscube/favicon_32x32.png
	aws s3 cp --profile personal web/favicon_128x128.png s3://tpscube/favicon_128x128.png
	aws s3 cp --profile personal web/favicon_180x180.png s3://tpscube/favicon_180x180.png
	aws s3 cp --profile personal web/manifest.appcache s3://tpscube/manifest.appcache
	aws s3 cp --profile personal --content-encoding gzip web/tpscube_bg.wasm.gz s3://tpscube/tpscube_bg.wasm
	aws cloudfront create-invalidation --profile personal --distribution-id E1CI0QZD6NW0J0 --paths "/*"
