SRC = $(wildcard src/*.rs)
BIN = $(patsubst src/%.rs,%,$(SRC))

LIBSHARED_FILENAME = $(shell rustc --crate-file-name lib/shared.rs --out-dir build/)
HOST_TARGET        = $(shell rustc -v | grep host | awk '{print $$2}')
LIBDIR             = $(addprefix .rust/lib/, $(HOST_TARGET))
LIBSHARED          = $(addprefix $(LIBDIR)/, $(LIBSHARED_FILENAME))
RUSTFLAGS          ?= -O

.PHONY: clean all libshared builddirs

all: $(addprefix bin/, $(BIN))

bin/%: src/%.rs | libshared builddirs
	rustc $(RUSTFLAGS) $< -o $@

%-test: src/%.rs | libshared builddirs
	rustc $(RUSTFLAGS) --test $< -o bin/$@
	bin/$@ --test

$(LIBSHARED): lib/*.rs | builddirs
	rustc $(RUSTFLAGS) lib/shared.rs --out-dir $(LIBDIR)

libshared: $(LIBSHARED) | builddirs

builddirs: bin $(LIBDIR)

bin:
	mkdir -p bin

$(LIBDIR):
	mkdir -p $(LIBDIR)

clean:
	rm -rf .rust bin
