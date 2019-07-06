all: target/release/swift-demangle
.PHONY: all

target/release/swift-demangle:
	cargo build --release
.PHONY: target/release/swift-demangle

install: target/release/swift-demangle
	./alfred-install-workflow/install-workflow.sh target/release/swift-demangle
.PHONY: install

update-plist:
	./alfred-install-workflow/install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
.PHONY: clean
