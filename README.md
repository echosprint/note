# Note

A simple, fast, and lightweight command-line note-taking application written in Rust.

## Features

- **Quick note creation**: Add notes instantly from the command line
- **Human-readable storage**: Notes are saved in plain text format for easy viewing and editing
- **Beautiful note listing**: Clean, appealing display with color-coded IDs and natural date formatting
- **Note removal**: Remove notes using their unique ID (supports partial ID matching)
- **Import/Export**: Backup and restore notes with simple commands
- **Multiline support**: Create notes with multiple lines and preserve formatting
- **Cross-platform**: Build for Linux, Windows (x86_64 and x86_32), and macOS

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
4. The binary will be available in `target/x86_64-unknown-linux-gnu/release/note` (Linux)

## Usage

### Create a Note
```bash
note "This is my first note"
note Remember to buy groceries
note "Multi-line note
This is line 2
And this is line 3"
```

### List All Notes
```bash
note
```

This displays all notes sorted by timestamp (newest first) with:
- Unique 4-character ID in yellow brackets `[abc1]`
- Date formatted as "Month Day" (e.g., "Dec 05")
- Note content with proper indentation for multiline notes

Example output:
```
[abc1] Mar 21
Note with slash date format
────────────────────────────────────
[def2] Jul 31
Multi-line note example
Line 2 of the note
Line 3 with more content
```

### Remove a Note
```bash
note rm <note-id>
```

Examples:
```bash
# Remove with full ID
note rm a1b2

# Remove with partial ID (as long as it's unique)
note rm a1     # Works if only one note starts with "a1"
note rm a1b    # More specific partial ID

# If partial ID matches multiple notes, you'll see all matches
note rm a      # Shows all notes starting with "a" if ambiguous
```

### Export Notes
```bash
# Export to stdout (pipe-friendly)
note output

# Export to file
note output backup.txt
note output backup-$(date +%Y%m%d).txt
```

### Import Notes
```bash
# Import notes from a file
note import backup.txt
note import exported-notes.txt
```

## Storage

Notes are stored in a human-readable text format:
- **Linux/macOS**: `~/.local/share/note/notes.txt`
- **Windows**: `%USERPROFILE%\.local\share\note\notes.txt`

The storage file is created automatically when you save your first note.

### Storage Format
Notes are stored in a simple, readable format:
```
#abc1 2025-03-21T00:00:00+08:00
This is the note content
Multiple lines are supported

#def2 2025-07-31T00:00:00+08:00
Another note here
```

This format supports:
- Various date formats: `2025/3/21`, `2025-7-31`, or full timestamps
- Multiline content with proper escaping
- Notes starting with `#` (automatically escaped as `\#`)
- Easy manual editing if needed

### Backup and Restore
```bash
# Create a backup
note output backup-$(date +%Y%m%d).txt

# Restore from backup
note import backup-20250709.txt

# View storage location
note -h  # Shows storage path in help
```

## Command Reference

```bash
# Basic usage
note                           # List all notes
note "content"                 # Create a new note
note rm <id>                   # Remove a note by ID (supports partial matching)

# Import/Export
note output                    # Export to stdout
note output <file>             # Export to file
note import <file>             # Import from file

# Help
note -h                        # Show help and storage location
note help <command>            # Show help for specific command
```

## Dependencies

- [clap](https://docs.rs/clap/) - Command line argument parsing
- [chrono](https://docs.rs/chrono/) - Date and time handling
- [serde](https://docs.rs/serde/) - Serialization/deserialization
- [anyhow](https://docs.rs/anyhow/) - Error handling
- [dirs](https://docs.rs/dirs/) - Platform-specific directories
- [colored](https://docs.rs/colored/) - Terminal colors

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
│   ├── main.rs         # Application entry point
│   ├── cli.rs          # Command-line interface
│   ├── manager.rs      # Note management and storage
│   ├── note.rs         # Note data structure
│   └── parser.rs       # Text format parsing
├── Cargo.toml          # Package configuration
├── Cargo.lock          # Dependency lock file
├── Makefile            # Build automation
└── README.md           # This file
```

## License

This project is open source. Please check the repository for license information.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests. 