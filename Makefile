build: build-usr build-fs build-os

build-usr:
	@./scripts/build-usr

build-fs:
	@./scripts/build-fs

build-os:
	@./scripts/build-os

clean:
	@cargo clean

qemu:
	@./scripts/qemu-run

qemu-debug:
	@./scripts/qemu-debug

debug:
	@./scripts/dgb
