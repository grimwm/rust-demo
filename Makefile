.PHONY: build clean test

build:
	make -C hello build

clean:
	make -C hello clean

test:
	make -C hello test
