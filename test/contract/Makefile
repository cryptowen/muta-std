TARGET := riscv64imac-unknown-none-elf
DOCKER_IMAGE := jjy0/ckb-capsule-recipe-rust:2020-5-9

# debug-in-docker:
# 	docker run --rm -v `pwd`:/code -w/code ${DOCKER_IMAGE} bash -c 'make debug'

release:
	mkdir -p build/release
	cargo build --release
	ckb-binary-patcher -i target/riscv64imac-unknown-none-elf/release/contract -o target/riscv64imac-unknown-none-elf/release/contract
	cp target/riscv64imac-unknown-none-elf/release/contract build/release/contract

debug:
	mkdir -p build/debug
	cargo build
	ckb-binary-patcher -i target/riscv64imac-unknown-none-elf/debug/contract -o target/riscv64imac-unknown-none-elf/debug/contract
	cp target/riscv64imac-unknown-none-elf/debug/contract build/debug/contract

.PHONY: debug release