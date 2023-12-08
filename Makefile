.PHONY: build-registry
build-registry:
	cd ./contracts/registry/; \
	cargo wasm

.PHONY: build-record-manager
build-record-manager:
	cd ./contracts/record-manager/; \
	cargo wasm