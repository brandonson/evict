default: bin src/fsm/lib.rs src/evict/main.rs
	rustc --out-dir=bin src/fsm/lib.rs
	rustc -L ./bin -o bin/evict src/evict/main.rs

bin:
	mkdir bin


install: default
	cp ./bin/evict /usr/local/bin/evict

bin/test: bin
	mkdir bin/test

test: default bin/test src/evict/main.rs src/fsm/lib.rs
	rustc --test -L./bin -o bin/test/evict src/evict/main.rs
	rustc --test -o bin/test/fsm src/fsm/lib.rs
	./bin/test/fsm
	./bin/test/evict
