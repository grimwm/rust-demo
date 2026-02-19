.PHONY: build clean test

build:
	rustc hello.rs

clean:
	rm -f hello
	rm -f hello_test

test:
	rustc --test hello.rs -o hello_test
	./hello_test
