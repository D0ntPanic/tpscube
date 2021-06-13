bootstrap:
	cargo build --release --target x86_64-unknown-linux-musl
	x86_64-unknown-linux-musl-strip target/x86_64-unknown-linux-musl/release/sync
	cp target/x86_64-unknown-linux-musl/release/sync bootstrap && zip lambda.zip bootstrap && rm bootstrap

deploy: bootstrap
	aws lambda update-function-code --profile personal --function-name tpscube_sync --zip-file fileb://./lambda.zip
