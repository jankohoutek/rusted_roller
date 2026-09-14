.PHONY: all build clean

all: build

build:
	@echo "--- Building project with cargo ---"
	cargo build --release --target x86_64-pc-windows-gnu

clean:
	@echo "--- Cleaning build artifacts ---"
	cargo clean