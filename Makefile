APP = hexeditor

build:
	cargo build --release

run:
	./target/release/$(APP)

debug:
	cargo build
	./target/debug/$(APP)
clean:
	cargo clean
