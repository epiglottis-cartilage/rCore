build: build-usr build-fs build-os

build-usr:
	@./scripts/build-usr

build-fs:
	@./scripts/build-fs

build-os:
	@./scripts/build-os

clean:
	@cargo clean

qemu: build
	@./scripts/qemu-run

qemu-debug: build
	@./scripts/qemu-debug

debug: build
	@./scripts/dgb
