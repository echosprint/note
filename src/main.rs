use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "note")]
#[command(about = "A simple command-line note-taking application")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Text content for a new note (when no subcommand is used)
    text: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Remove a note by ID
    #[command(name = "rm")]
    Remove {
        /// Note ID to delete
        id: String,
    },
}

#[derive(Debug)]
enum RemoveResult {
    Removed(String),
    NotFound,
    Ambiguous(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone)]
struct Note {
    id: String,
    content: String,
    timestamp: DateTime<Local>,
}

impl Note {
    fn new(content: String, existing_ids: &[String]) -> Self {
        let timestamp = Local::now();
        let id = Self::generate_unique_id(&content, &timestamp, existing_ids);
        Self {
            id,
            content,
            timestamp,
        }
    }
    
    fn generate_unique_id(content: &str, timestamp: &DateTime<Local>, existing_ids: &[String]) -> String {
        let mut counter = 0u32;
        loop {
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            timestamp.timestamp_nanos_opt().unwrap_or(0).hash(&mut hasher);
            // Add counter for uniqueness in case of collision
            counter.hash(&mut hasher);
            
            let hash = hasher.finish();
            let id: String = format!("{:x}", hash).chars().take(4).collect();
            
            // Check if this ID already exists
            if !existing_ids.contains(&id) {
                return id;
            }
            
            counter += 1;
            // Safety check to prevent infinite loop (though extremely unlikely)
            if counter > 65536 {
                // Fallback to a longer ID if we somehow exhaust all possibilities
                return format!("{:x}", hash).chars().take(8).collect();
            }
        }
    }
}

struct NoteManager {
    notes_file: PathBuf,
    notes: Vec<Note>,
}

impl NoteManager {
    fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().context("Failed to get home directory")?;
        let notes_dir = home_dir.join(".local").join("share").join("note");
        let notes_file = notes_dir.join("notes.txt");
        
        // Create parent directories if they don't exist
        if let Some(parent) = notes_file.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create notes directory")?;
        }
        
        let mut manager = Self {
            notes_file,
            notes: Vec::new(),
        };
        
        manager.load_notes()?;
        Ok(manager)
    }
    
    fn load_notes(&mut self) -> Result<()> {
        if !self.notes_file.exists() {
            // File doesn't exist, start with empty list
            self.notes = Vec::new();
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.notes_file)
            .context("Failed to read notes file")?;
        
        if content.trim().is_empty() {
            self.notes = Vec::new();
            return Ok(());
        }
        
        self.notes = serde_json::from_str(&content)
            .context("Failed to parse notes file")?;
        
        Ok(())
    }
    
    fn save_notes(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.notes)
            .context("Failed to serialize notes")?;
        
        fs::write(&self.notes_file, content)
            .context("Failed to write notes file")?;
        
        Ok(())
    }
    
    fn add_note(&mut self, content: String) -> Result<String> {
        let existing_ids: Vec<String> = self.notes.iter().map(|n| n.id.clone()).collect();
        let note = Note::new(content, &existing_ids);
        let note_id = note.id.clone();
        self.notes.push(note);
        self.save_notes()?;
        
        Ok(note_id)
    }
    
    fn display_notes(&self, notes: &[Note]) {
        println!();
        for (index, note) in notes.iter().enumerate() {
            let formatted_time = note.timestamp.format("%b %d");
            
            // Add subtle visual separator between notes
            if index > 0 {
                println!("  {}", "────────────────────────────────────".bright_black());
            }
            
            // Color-coded output with better spacing
            println!("  {} {}", 
                format!("{:>6}", formatted_time).cyan(),
                format!("[{}]", note.id).yellow()
            );
            println!("  {}", note.content);
        }
        println!();
    }
    
    fn list_notes(&self) {
        if self.notes.is_empty() {
            println!("{} No notes yet. Create your first note with: {}", 
                "✨".bright_black(), 
                "note <your text>".bright_cyan()
            );
            return;
        }
        
        // Sort by timestamp, newest first
        let mut sorted_notes = self.notes.clone();
        sorted_notes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        self.display_notes(&sorted_notes);
    }
    
    fn remove_note_by_id(&mut self, id: &str) -> Result<RemoveResult> {
        // Find all notes that start with the given partial ID
        let matching_notes: Vec<&Note> = self.notes.iter()
            .filter(|note| note.id.starts_with(id))
            .collect();
        
        match matching_notes.len() {
            0 => Ok(RemoveResult::NotFound),
            1 => {
                let note_id = matching_notes[0].id.clone();
                self.notes.retain(|note| note.id != note_id);
                self.save_notes()?;
                Ok(RemoveResult::Removed(note_id))
            }
            _ => {
                let ambiguous_ids: Vec<String> = matching_notes.iter()
                    .map(|note| note.id.clone())
                    .collect();
                Ok(RemoveResult::Ambiguous(ambiguous_ids))
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut note_manager = NoteManager::new()?;
    
    match &cli.command {
        Some(Commands::Remove { id }) => {
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
                        if let Some(note) = note_manager.notes.iter().find(|n| n.id == matching_id) {
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
        }
        None => {
            if !cli.text.is_empty() {
                // Join all text arguments with spaces to form the note content
                let text = cli.text.join(" ");
                let note_id = note_manager.add_note(text)?;
                println!("{} Note saved {}", 
                    "✓".green(), 
                    format!("[{}]", note_id).yellow()
                );
            } else {
                // List all notes
                note_manager.list_notes();
            }
        }
    }
    
    Ok(())
}
