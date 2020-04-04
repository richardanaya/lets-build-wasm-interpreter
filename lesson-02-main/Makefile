build:
	@RUSTFLAGS='-C link-arg=-s' cargo +nightly build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/lesson.wasm ./interpreter.wasm
serve:
	@python3 -m http.server 8080
