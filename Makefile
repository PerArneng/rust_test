
all: hello_world

hello_world: hello_world.rs
	mkdir -p bin
	rustc --out-dir bin hello_world.rs