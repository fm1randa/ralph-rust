.PHONY: build install archive clean test

SCRIPTS_DIR := /Users/filipe/Scripts
ARCHIVE_DIR := $(SCRIPTS_DIR)/archive
TIMESTAMP := $(shell date +%Y-%m-%d)

build:
	cargo build --release

archive:
	@mkdir -p $(ARCHIVE_DIR)
	@if [ -f "$(SCRIPTS_DIR)/ralph" ] && [ ! -L "$(SCRIPTS_DIR)/ralph" ]; then \
		cp "$(SCRIPTS_DIR)/ralph" "$(ARCHIVE_DIR)/ralph.bash.$(TIMESTAMP)"; \
		echo "Archived original to $(ARCHIVE_DIR)/ralph.bash.$(TIMESTAMP)"; \
	else \
		echo "Original already archived or ralph is a symlink"; \
	fi

install: build archive
	cp target/release/ralph $(SCRIPTS_DIR)/ralph-rs
	@echo "Installed Rust binary to $(SCRIPTS_DIR)/ralph-rs"
	@if [ -f "$(SCRIPTS_DIR)/ralph" ] && [ ! -L "$(SCRIPTS_DIR)/ralph" ]; then \
		mv "$(SCRIPTS_DIR)/ralph" "$(SCRIPTS_DIR)/ralph.bash"; \
		ln -s ralph-rs "$(SCRIPTS_DIR)/ralph"; \
		echo "Created symlink: ralph -> ralph-rs"; \
		echo "Original bash script preserved as: ralph.bash"; \
	elif [ -L "$(SCRIPTS_DIR)/ralph" ]; then \
		rm "$(SCRIPTS_DIR)/ralph"; \
		ln -s ralph-rs "$(SCRIPTS_DIR)/ralph"; \
		echo "Updated symlink: ralph -> ralph-rs"; \
	fi

restore-bash:
	@if [ -f "$(SCRIPTS_DIR)/ralph.bash" ]; then \
		rm -f "$(SCRIPTS_DIR)/ralph"; \
		mv "$(SCRIPTS_DIR)/ralph.bash" "$(SCRIPTS_DIR)/ralph"; \
		echo "Restored original bash script"; \
	else \
		echo "No ralph.bash backup found"; \
	fi

clean:
	cargo clean

test:
	cargo test
