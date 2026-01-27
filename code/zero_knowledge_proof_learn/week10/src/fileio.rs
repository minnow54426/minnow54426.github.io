//! File I/O utilities for JSON artifact files
//!
//! This module provides utilities for loading and saving JSON files
//! used by the zk-proof-artifacts CLI tool.

use std::path::PathBuf;
use std::fs;
use crate::error::{CliError, Result};

/// Load and parse JSON from file
///
/// # Arguments
/// * `path` - Path to the JSON file to load
///
/// # Returns
/// Deserialized data structure from the JSON file
///
/// # Errors
/// Returns `CliError::FileNotFound` if the file doesn't exist
/// Returns `CliError::InvalidJson` if the file cannot be read or parsed
pub fn load_json_file<T>(path: PathBuf) -> Result<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    if !path.exists() {
        return Err(CliError::FileNotFound(path));
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| CliError::InvalidJson(
            path.to_string_lossy().to_string(),
            e.to_string(),
        ))?;

    serde_json::from_str(&content).map_err(|e| {
        CliError::InvalidJson(path.to_string_lossy().to_string(), e.to_string())
    })
}

/// Save data as JSON to file
///
/// # Arguments
/// * `path` - Path where to save the JSON file
/// * `data` - Data structure to serialize as JSON
///
/// # Returns
/// Ok(()) if successful
///
/// # Errors
/// Returns `CliError::SerializationError` if serialization fails
/// Returns `CliError::IoError` if writing fails
pub fn save_json_file<T>(path: PathBuf, data: &T) -> Result<()>
where
    T: serde::Serialize,
{
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| CliError::SerializationError(e.to_string()))?;

    fs::write(&path, json).map_err(CliError::IoError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_json_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let json = r#"{"test": "data"}"#;
        fs::write(temp_file.path(), json).unwrap();

        let data: serde_json::Value = load_json_file(temp_file.path().to_path_buf()).unwrap();
        assert_eq!(data["test"], "data");
    }

    #[test]
    fn test_load_json_file_not_found() {
        let result: std::result::Result<serde_json::Value, _> =
            load_json_file(std::path::PathBuf::from("/nonexistent/file.json"));
        assert!(result.is_err());
    }

    #[test]
    fn test_save_json_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let data = serde_json::json!({"test": "data"});

        save_json_file(temp_file.path().to_path_buf(), &data).unwrap();
        let content = fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("test"));
    }
}
