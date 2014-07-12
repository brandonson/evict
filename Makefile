RUST_FILES = $(shell find src -type f -name '*.rs') 


default: bin/evict

bin/evict: bin/test lib $(RUST_FILES)
	rustc --out-dir=lib src/fsm/lib.rs
	rustc -L ./lib -o bin/evict src/evict/main.rs

bin/test:
	mkdir -p bin/test

lib:
	mkdir lib

install: default
	cp ./bin/evict /usr/local/bin/evict

test: default bin/test $(RUST_FILES)
	rustc --test -L./lib -o bin/test/evict src/evict/main.rs
	rustc --test -o bin/test/fsm src/fsm/lib.rs
	./bin/test/fsm
	./bin/test/evict

clean:
	rm -rf bin
	rm -rf lib

