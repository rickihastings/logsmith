packages = $(shell ls packages)

test:
	for package in $(packages) ; do \
		cd packages/$$package ; \
		cargo test ; \
		cd ../ ; \
	done
