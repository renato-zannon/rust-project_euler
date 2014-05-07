SRC = $(wildcard src/*.rs)
BIN = $(patsubst src/%.rs,%,$(SRC))

LIBSHARED_FILENAME = $(shell rustc --crate-file-name lib/shared.rs --out-dir build/)
LIBSHARED          = $(addprefix build/, $(LIBSHARED_FILENAME))
RUSTFLAGS          ?= -O

.PHONY: clean all libshared

all: $(addprefix bin/, $(BIN))

bin/%: src/%.rs libshared | builddirs
	rustc $(RUSTFLAGS) -L build/ $< -o $@

builddirs: | bin build

bin:
	mkdir -p bin/

build:
	mkdir -p build/

clean:
	rm -rf build bin

libshared: $(LIBSHARED)

$(LIBSHARED): lib/*.rs | builddirs
	rustc $(RUSTFLAGS) lib/shared.rs --out-dir build/
