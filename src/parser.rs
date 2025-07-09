use anyhow::Result;
use chrono::{DateTime, Local};
use crate::note::Note;

pub struct NoteParser;

impl NoteParser {
    pub fn parse_notes_from_text(content: &str) -> Result<Vec<Note>> {
        let mut notes = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Skip empty lines
            if line.is_empty() {
                i += 1;
                continue;
            }
            
            // Look for lines starting with #
            if line.starts_with('#') {
                // Parse the header line: #id date
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let id = parts[0][1..].to_string(); // Remove the # prefix
                    let date_str = parts[1..].join(" ");
                    
                    // Parse the timestamp
                    let timestamp = DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| DateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S %z"))
                        .or_else(|_| DateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S"))
                        .map(|dt| dt.with_timezone(&Local))
                        .or_else(|_| {
                            // Try parsing simple date formats and assume current time
                            Self::parse_simple_date(&date_str)
                                .map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Local).single().unwrap())
                        })
                        .unwrap_or_else(|_| Local::now());
                    
                    // Collect content lines until next note or end of file
                    let mut content_lines = Vec::new();
                    i += 1;
                    
                    while i < lines.len() {
                        let content_line = lines[i];
                        if content_line.trim().starts_with('#') && !content_line.trim_start().starts_with("\\#") {
                            break;
                        }
                        content_lines.push(content_line);
                        i += 1;
                    }
                    
                    let content = Self::unescape_content(&content_lines.join("\n")).trim().to_string();
                    if !content.is_empty() {
                        notes.push(Note {
                            id,
                            content,
                            timestamp,
                        });
                    }
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        
        Ok(notes)
    }
    
    fn parse_simple_date(date_str: &str) -> Result<chrono::NaiveDate> {
        // Try parsing various date formats
        let formats = [
            "%Y/%m/%d",   // 2025/3/21 or 2025/03/21
            "%Y-%m-%d",   // 2025-7-31 or 2025-07-31
            "%Y/%#m/%#d", // 2025/3/21 (non-zero-padded) - Windows style
            "%Y-%#m-%#d", // 2025-7-31 (non-zero-padded) - Windows style
        ];
        
        for format in &formats {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, format) {
                return Ok(date);
            }
        }
        
        // Manual parsing for non-zero-padded dates (cross-platform)
        if let Some(date) = Self::parse_manual_date(date_str) {
            return Ok(date);
        }
        
        Err(anyhow::anyhow!("Could not parse date: {}", date_str))
    }
    
    fn parse_manual_date(date_str: &str) -> Option<chrono::NaiveDate> {
        // Handle formats like "2025/3/21" or "2025-7-31"
        let parts: Vec<&str> = if date_str.contains('/') {
            date_str.split('/').collect()
        } else if date_str.contains('-') {
            date_str.split('-').collect()
        } else {
            return None;
        };
        
        if parts.len() != 3 {
            return None;
        }
        
        let year: i32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;
        
        chrono::NaiveDate::from_ymd_opt(year, month, day)
    }
    
    pub fn escape_content(content: &str) -> String {
        content.lines()
            .map(|line| {
                if line.trim_start().starts_with('#') {
                    format!("\\{}", line)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn unescape_content(content: &str) -> String {
        content.lines()
            .map(|line| {
                if line.trim_start().starts_with("\\#") {
                    &line[line.find("\\#").unwrap() + 1..]
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}