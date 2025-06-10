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

# Build for macOS Intel (x86_64)
.PHONY: macos_intel
macos_intel:
	@echo "Building for x86_64-apple-darwin (macOS Intel)"
	cargo build --release --target=x86_64-apple-darwin

# Build for macOS Apple Silicon (ARM64)
.PHONY: macos_arm
macos_arm:
	@echo "Building for aarch64-apple-darwin (macOS Apple Silicon)"
	cargo build --release --target=aarch64-apple-darwin

# Build for all macOS targets
.PHONY: macos
macos: macos_intel macos_arm

# Build for all platform
.PHONY: all
all: linux x86_64 x86_32 macos

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
