SRC = $(wildcard src/*.rs)
BIN = $(patsubst src/%.rs,%,$(SRC))

LIBSHARED_FILENAME = $(shell rustc --crate-file-name lib/shared.rs --out-dir build/)
LIBDIR             = build
LIBSHARED          = $(addprefix $(LIBDIR)/, $(LIBSHARED_FILENAME))
RUSTFLAGS          ?= -O

.PHONY: clean all libshared builddirs

all: $(addprefix bin/, $(BIN))

bin/%: src/%.rs | libshared builddirs
	rustc -L $(LIBDIR) $(RUSTFLAGS) $< -o $@

%-test: src/%.rs | libshared builddirs
	rustc -L $(LIBDIR) $(RUSTFLAGS) --test $< -o bin/$@
	bin/$@ --test

$(LIBSHARED): lib/*.rs | builddirs
	rustc $(RUSTFLAGS) lib/shared.rs --out-dir $(LIBDIR)

libshared-test: lib/*.rs | builddirs
	rustc $(RUSTFLAGS) --test lib/shared.rs --out-dir $(LIBDIR)
	build/shared --test

libshared: $(LIBSHARED) | builddirs


builddirs: bin $(LIBDIR)

bin:
	mkdir -p bin

$(LIBDIR):
	mkdir -p $(LIBDIR)

clean:
	rm -rf build bin
