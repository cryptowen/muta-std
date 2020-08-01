default: debug release

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