# Default build target
.PHONY: default
default: linux 

# Build for Linux x86_64 (default)
.PHONY: linux
linux:
	@echo "Building for ubuntu(linux) (default)"
	cargo build --release 

# Build for Windows x86_64 explicitly
.PHONY: x86_64
x86_64:
	@echo "Building for x86_64-pc-windows-gnu"
	cargo build --release --target=x86_64-pc-windows-gnu

# Build for Windows x86_32
.PHONY: x86_32
x86_32:
	@echo "Building for i686-pc-windows-gnu (Win32)"
	cargo build --release --target=i686-pc-windows-gnu

# Build for all platform
.PHONY: all
all: linux x86_64 x86_32

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
