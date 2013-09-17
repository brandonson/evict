
default: bin/evict

bin/evict: bin lib src/fsm/*.rs src/evict/*.rs src/evict/commands/*.rs 
	rustc --out-dir=lib src/fsm/lib.rs
	rustc -L ./lib -o bin/evict src/evict/main.rs

bin:
	mkdir bin

lib:
	mkdir lib

install: default
	cp ./bin/evict /usr/local/bin/evict

bin/test: bin
	mkdir bin/test

test: default bin/test src/evict/main.rs src/fsm/lib.rs
	rustc --test -L./lib -o bin/test/evict src/evict/main.rs
	rustc --test -o bin/test/fsm src/fsm/lib.rs
	./bin/test/fsm
	./bin/test/evict

clean:
	rm -rf bin
	rm -rf lib

