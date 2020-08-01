ci: fmt test

fmt:
	cargo fmt --all -- --check

test:
	make -C test test

.PHONY: ci fmt test