SRC = $(wildcard src/*.rs)
BIN = $(patsubst src/%.rs,%,$(SRC))

LIBSHARED_FILENAME = $(shell rustc --crate-file-name lib/shared.rs --out-dir build/)
LIBSHARED          = $(addprefix build/, $(LIBSHARED_FILENAME))

.PHONY: clean all

all: $(addprefix bin/, $(BIN))

bin/%: src/%.rs $(LIBSHARED) | builddirs
	rustc -L build/ -O $< -o $@

builddirs: | bin build

bin:
	mkdir -p bin/

build:
	mkdir -p build/

clean:
	rm -rf build bin

$(LIBSHARED): lib/*.rs | builddirs
	rustc -O lib/shared.rs --out-dir build/
