// src/logger/file_logger.rs

use anyhow::{Context, Result};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

/// Simple CSV file logger
pub struct FileLogger {
    path: String,
}

impl FileLogger {
    /// Create a new FileLogger pointing at `path`.  
    /// If file does not exist, it will be created.
    pub fn new(path: impl Into<String>) -> Result<Self> {
        let path = path.into();
        let p = Path::new(&path);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create log directory")?;
        }
        // Ensure file exists
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .with_context(|| format!("Failed to create or open log file {}", path))?;
        Ok(FileLogger { path })
    }

    /// Write CSV header line, e.g. `"timestamp,reg,value"`
    pub fn write_header(&mut self, header: &str) -> Result<()> {
        let mut f = File::create(&self.path)
            .with_context(|| format!("Failed to overwrite log file {}", self.path))?;
        writeln!(f, "{}", header)
            .context("Failed to write CSV header")?;
        Ok(())
    }

    /// Append a CSV row: `timestamp,register,value`
    pub fn log(&mut self, timestamp: &str, register: u16, value: f32) -> Result<()> {
        let mut f = OpenOptions::new()
            .append(true)
            .open(&self.path)
            .with_context(|| format!("Failed to open log file {}", self.path))?;
        writeln!(f, "{},{},{}", timestamp, register, value)
            .context("Failed to write log entry")?;
        Ok(())
    }
}
