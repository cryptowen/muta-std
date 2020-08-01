TARGET := riscv64imac-unknown-none-elf
DOCKER_IMAGE := jjy0/ckb-capsule-recipe-rust:2020-5-9

build-contract:
	docker run --rm -it -v `pwd`/..:/code -v `pwd`/contract/target/.cargo/git:/root/.cargo/git -v `pwd`/contract/target/.cargo/registry:/root/.cargo/registry ${DOCKER_IMAGE} bash -c 'cd /code/test/contract && make'

test: build-contract
	cargo test -- --nocapture

.PHONY: test