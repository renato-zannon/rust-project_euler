.PHONY: all build clean

all: Cargo.toml build

build: Cargo.toml
	cargo build

clean: Cargo.toml
	cargo clean

Cargo.toml: src/*.rs
	@sed -e '/# SOLUTIONS BEGIN/q' -i Cargo.toml
	@for solution_path in src/*.rs; do \
	  solution="$$(basename $${solution_path%%.rs})"; \
		echo; \
	  echo "[[bin]]"; \
		echo "name = \"$$solution\""; \
		echo "path = \"$$solution_path\""; \
	done >> Cargo.toml
