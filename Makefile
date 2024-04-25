default: build

build:
	cargo build

install:
	cp ./target/debug/near-burrow ~/bin/

