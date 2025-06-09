# Note

A simple, fast, and lightweight command-line note-taking application written in Rust.

## Features

- **Quick note creation**: Add notes instantly from the command line
- **Persistent storage**: Notes are saved in JSON format in your home directory
- **Note listing**: View all notes with timestamps and unique IDs
- **Note deletion**: Remove notes using their unique ID
- **Beautiful output**: Color-coded display with clean formatting
- **Cross-platform**: Build for Linux, Windows (x86_64 and x86_32)

## Installation

### Build from Source

1. Make sure you have [Rust](https://rustup.rs/) installed
2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd note
   ```
3. Build the application:
   ```bash
   make linux      # For Linux (default)
   make x86_64     # For Windows x86_64
   make x86_32     # For Windows x86_32
   make all        # Build for all platforms
   ```
4. The binary will be available in `target/release/note`

## Usage

### Create a Note
```bash
note "This is my first note"
note Remember to buy groceries
```

### List All Notes
```bash
note
```

This displays all notes sorted by timestamp (newest first) with:
- Date (formatted as "Month Day")
- Unique 4-character ID in brackets
- Note content

### Delete a Note
```bash
note delete <note-id>
```

Example:
```bash
note delete a1b2
```

## Storage

Notes are stored in:
- **Linux/macOS**: `~/.local/share/note/notes.txt`
- **Windows**: `%USERPROFILE%\.local\share\note\notes.txt`

The storage file is created automatically when you save your first note.

## Dependencies

- [clap](https://docs.rs/clap/) - Command line argument parsing
- [chrono](https://docs.rs/chrono/) - Date and time handling
- [serde](https://docs.rs/serde/) - Serialization/deserialization
- [anyhow](https://docs.rs/anyhow/) - Error handling
- [dirs](https://docs.rs/dirs/) - Platform-specific directories

## Development

### Build Commands
```bash
make linux      # Build for Linux
make x86_64     # Build for Windows 64-bit
make x86_32     # Build for Windows 32-bit
make all        # Build for all platforms
make clean      # Clean build artifacts
```

### Project Structure
```
.
├── src/
│   └── main.rs         # Main application code
├── Cargo.toml          # Package configuration
├── Cargo.lock          # Dependency lock file
├── Makefile            # Build automation
└── README.md           # This file
```

## License

This project is open source. Please check the repository for license information.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests. 