use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use colored::*;
use std::fs;
use std::path::PathBuf;

use crate::note::{Note, RemoveResult};
use crate::parser::NoteParser;

pub struct NoteManager {
    notes_file: PathBuf,
    notes: Vec<Note>,
}

impl NoteManager {
    pub fn new() -> Result<Self> {
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
        
        self.notes = NoteParser::parse_notes_from_text(&content)
            .context("Failed to parse notes file")?;
        
        Ok(())
    }
    
    fn save_notes(&self) -> Result<()> {
        let mut content = String::new();
        
        // Sort notes by timestamp (newest first) for consistent output
        let mut sorted_notes = self.notes.clone();
        sorted_notes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        for (index, note) in sorted_notes.iter().enumerate() {
            if index > 0 {
                content.push('\n');
            }
            
            // Write header line: #id timestamp
            content.push_str(&format!("#{} {}\n", note.id, note.timestamp.to_rfc3339()));
            
            // Write note content, escaping lines that start with #
            let escaped_content = NoteParser::escape_content(&note.content);
            content.push_str(&escaped_content);
            content.push('\n');
        }
        
        fs::write(&self.notes_file, content)
            .context("Failed to write notes file")?;
        
        Ok(())
    }
    
    pub fn add_note(&mut self, content: String) -> Result<String> {
        let existing_ids: Vec<String> = self.notes.iter().map(|n| n.id.clone()).collect();
        let note = Note::new(content, &existing_ids);
        let note_id = note.id.clone();
        self.notes.push(note);
        self.save_notes()?;
        
        Ok(note_id)
    }
    
    pub fn display_notes(&self, notes: &[Note]) {
        println!();
        
        for (index, note) in notes.iter().enumerate() {
            // Add separating line between notes
            if index > 0 {
                println!("  {}", "────────────────────────────────────".bright_black());
            }
            
            let formatted_time = self.format_natural_date(&note.timestamp);
            
            // Show ID first, then date
            println!("  {} {}", 
                format!("[{}]", note.id).yellow(),
                formatted_time.bright_black()
            );
            
            // Display content with comfortable indentation, no highlighting
            for line in note.content.lines() {
                println!("  {}", line);
            }
        }
        
        println!();
    }
    
    fn format_natural_date(&self, timestamp: &DateTime<Local>) -> String {
        timestamp.format("%b %d").to_string()
    }
    
    pub fn list_notes(&self) {
        if self.notes.is_empty() {
            println!();
            println!("  {} {}", 
                "✨".bright_white(),
                "No notes yet".bright_black()
            );
            println!("     {}", "Create your first note with:".white());
            println!("     {} {}", 
                "note".bright_cyan(),
                "\"your text here\"".bright_black()
            );
            println!();
            return;
        }
        
        // Sort by timestamp, newest first
        let mut sorted_notes = self.notes.clone();
        sorted_notes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        self.display_notes(&sorted_notes);
    }
    
    pub fn remove_note_by_id(&mut self, id: &str) -> Result<RemoveResult> {
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
    
    pub fn get_notes(&self) -> &[Note] {
        &self.notes
    }
    
    pub fn output_raw_content(&self) -> Result<()> {
        if !self.notes_file.exists() {
            // If notes file doesn't exist, output nothing
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.notes_file)
            .context("Failed to read notes file")?;
        
        // Output the raw content directly without any formatting
        print!("{}", content);
        
        Ok(())
    }
    
    pub fn output_raw_content_to_file(&self, file_path: &str) -> Result<()> {
        if !self.notes_file.exists() {
            // If notes file doesn't exist, create empty file
            fs::write(file_path, "").context("Failed to write to output file")?;
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.notes_file)
            .context("Failed to read notes file")?;
        
        fs::write(file_path, content)
            .context("Failed to write to output file")?;
        
        Ok(())
    }
    
    pub fn import_from_file(&mut self, file_path: &str) -> Result<usize> {
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file: {}", file_path))?;
        
        if content.trim().is_empty() {
            return Ok(0);
        }
        
        // Parse the imported notes
        let imported_notes = NoteParser::parse_notes_from_text(&content)
            .context("Failed to parse imported notes")?;
        
        if imported_notes.is_empty() {
            return Ok(0);
        }
        
        // Get existing IDs to avoid conflicts
        let existing_ids: Vec<String> = self.notes.iter().map(|n| n.id.clone()).collect();
        
        // Add imported notes, regenerating IDs if there are conflicts
        let mut imported_count = 0;
        for imported_note in imported_notes {
            let note_content = imported_note.content;
            let mut note_id = imported_note.id;
            
            // Check for ID conflicts and regenerate if needed
            if existing_ids.contains(&note_id) {
                // Generate a new unique ID
                let all_existing_ids: Vec<String> = self.notes.iter()
                    .map(|n| n.id.clone())
                    .chain(std::iter::once(note_id.clone()))
                    .collect();
                
                let new_note = crate::note::Note::new(note_content.clone(), &all_existing_ids);
                note_id = new_note.id;
            }
            
            // Add the note with original timestamp but potentially new ID
            self.notes.push(crate::note::Note {
                id: note_id,
                content: note_content,
                timestamp: imported_note.timestamp,
            });
            
            imported_count += 1;
        }
        
        // Save the updated notes
        self.save_notes()?;
        
        Ok(imported_count)
    }
}