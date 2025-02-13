# Makefile
.PHONY: release-local release-push install uninstall clean

# Default version bump type
BUMP ?= patch

# Get current version from Cargo.toml
VERSION := $(shell grep '^version = ' Cargo.toml | cut -d '"' -f2)

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Local Release Steps
release-local:
	@echo "${YELLOW}Running local release steps for v${VERSION}...${NC}"
	@echo "${GREEN}1. Running tests...${NC}"
	cargo test
	@echo "${GREEN}2. Running clippy...${NC}"
	cargo clippy
	@echo "${GREEN}3. Formatting code...${NC}"
	cargo fmt
	@echo "${GREEN}4. Building release...${NC}"
	cargo build --release
	@echo "${GREEN}5. Installing locally...${NC}"
	cargo install --path .
	@echo "${GREEN}✓ Local release steps completed${NC}"
	@echo "${YELLOW}Run 'make release-push' to push the release to git${NC}"

# Git Release Steps
release-push:
	@echo "${YELLOW}Pushing release v${VERSION} to git...${NC}"
	@# Check if working directory is clean
	@if [ -n "$(shell git status --porcelain)" ]; then \
		echo "${RED}Error: Working directory not clean${NC}"; \
		exit 1; \
	fi
	@echo "${GREEN}1. Creating git tag...${NC}"
	git tag -a "v${VERSION}" -m "Release v${VERSION}"
	@echo "${GREEN}2. Pushing to main...${NC}"
	git push origin main
	@echo "${GREEN}3. Pushing tags...${NC}"
	git push origin "v${VERSION}"
	@echo "${GREEN}✓ Release v${VERSION} pushed to git${NC}"

# Install locally
install:
	@echo "${YELLOW}Installing workspace-aggregator...${NC}"
	cargo install --path .
	@echo "${GREEN}✓ Installation complete${NC}"

# Uninstall
uninstall:
	@echo "${YELLOW}Uninstalling workspace-aggregator...${NC}"
	cargo uninstall workspace-aggregator
	@echo "${GREEN}✓ Uninstallation complete${NC}"

# Clean build artifacts
clean:
	@echo "${YELLOW}Cleaning build artifacts...${NC}"
	cargo clean
	rm -rf target/
	@echo "${GREEN}✓ Clean complete${NC}"

# Update version
bump:
	@echo "${YELLOW}Current version: ${VERSION}${NC}"
	@cargo install cargo-edit
	@cargo set-version --bump $(BUMP)
	@echo "${GREEN}✓ Version bumped to: $$(grep '^version = ' Cargo.toml | cut -d '"' -f2)${NC}"
