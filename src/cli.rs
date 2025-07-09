use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

use crate::manager::NoteManager;
use crate::note::RemoveResult;

fn get_storage_help() -> String {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    let storage_path = home_dir.join(".local").join("share").join("note").join("notes.txt");
    
    format!("STORAGE:\n  Notes are stored in: {}", storage_path.display())
}

#[derive(Parser)]
#[command(name = "note")]
#[command(about = "A simple command-line note-taking application")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(after_help = get_storage_help())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Text content for a new note (when no subcommand is used)
    pub text: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Remove a note by ID
    #[command(name = "rm")]
    Remove {
        /// Note ID to delete
        id: String,
    },
    /// Output raw content of notes file
    #[command(name = "output")]
    Output {
        /// Optional file path to write output to (defaults to stdout)
        file: Option<String>,
    },
    /// Import notes from a text file
    #[command(name = "import")]
    Import {
        /// Path to the text file to import
        file: String,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    let mut note_manager = NoteManager::new()?;
    
    match &cli.command {
        Some(Commands::Remove { id }) => {
            handle_remove_command(&mut note_manager, id)?;
        }
        Some(Commands::Output { file }) => {
            handle_output_command(&note_manager, file.as_deref())?;
        }
        Some(Commands::Import { file }) => {
            handle_import_command(&mut note_manager, file)?;
        }
        None => {
            if !cli.text.is_empty() {
                handle_add_command(&mut note_manager, cli.text)?;
            } else {
                // List all notes
                note_manager.list_notes();
            }
        }
    }
    
    Ok(())
}

fn handle_remove_command(note_manager: &mut NoteManager, id: &str) -> Result<()> {
    match note_manager.remove_note_by_id(id)? {
        RemoveResult::Removed(note_id) => {
            println!("{} Note {} removed", 
                "✓".green(), 
                format!("[{}]", note_id).yellow()
            );
        }
        RemoveResult::NotFound => {
            println!("{} No notes found matching {}", 
                "✗".red(), 
                format!("[{}]", id).yellow()
            );
        }
        RemoveResult::Ambiguous(matching_ids) => {
            println!("{} Multiple notes match {}:", 
                "⚠".yellow(), 
                format!("[{}]", id).yellow()
            );
            println!("  Please be more specific. Matching notes:");
            for matching_id in matching_ids {
                if let Some(note) = note_manager.get_notes().iter().find(|n| n.id == matching_id) {
                    let formatted_time = note.timestamp.format("%b %d");
                    println!("    {} {} {}", 
                        format!("{:>6}", formatted_time).cyan(),
                        format!("[{}]", note.id).yellow(),
                        note.content.chars().take(50).collect::<String>()
                            + if note.content.len() > 50 { "..." } else { "" }
                    );
                }
            }
        }
    }
    Ok(())
}

fn handle_add_command(note_manager: &mut NoteManager, text: Vec<String>) -> Result<()> {
    // Join all text arguments with spaces to form the note content
    let content = text.join(" ");
    let note_id = note_manager.add_note(content)?;
    println!("{} Note saved {}", 
        "✓".green(), 
        format!("[{}]", note_id).yellow()
    );
    Ok(())
}

fn handle_output_command(note_manager: &NoteManager, file_path: Option<&str>) -> Result<()> {
    match file_path {
        Some(path) => {
            note_manager.output_raw_content_to_file(path)?;
            println!("{} Notes exported to {}", 
                "✓".green(),
                path.bright_cyan()
            );
        }
        None => {
            note_manager.output_raw_content()?;
        }
    }
    Ok(())
}

fn handle_import_command(note_manager: &mut NoteManager, file_path: &str) -> Result<()> {
    let imported_count = note_manager.import_from_file(file_path)?;
    println!("{} {} imported from {}", 
        "✓".green(),
        if imported_count == 1 { "note" } else { "notes" },
        file_path.bright_cyan()
    );
    Ok(())
}