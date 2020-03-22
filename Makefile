packages = $(shell ls packages)
modules = $(shell ls modules)

clean:
	cargo clean
	cd modules/input-stdin && cargo clean

test:
	for package in $(packages) ; do \
		cd packages/$$package ; \
		cargo test ; \
		cd ../ ; \
	done

build:
	cargo build

build-modules: build-input-stdin

build-input-stdin:
	cd modules/input-stdin && \
	cargo build && \
	mkdir -p ../../target/debug/plugins && \
	cp target/debug/libinput_stdin.dylib ../../target/debug/plugins
