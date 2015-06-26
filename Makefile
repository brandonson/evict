install:
	cargo build --release
	cp ./target/release/evict /usr/local/bin/evict

