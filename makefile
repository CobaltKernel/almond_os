.PHONY: build, clean

debug:
	cargo run --debug

release:
	cargo run --release

shell:
	cargo run --release --features "shell"

clean:
	cargo clean
	qemu-img create mfs.img 128M

build:
	cargo build


