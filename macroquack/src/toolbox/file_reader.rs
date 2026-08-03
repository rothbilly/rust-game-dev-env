

use std::fs;
use std::path::Path;

/// A simple file reader for loading text files
pub struct FileReader;

impl FileReader {
    /// Reads the entire contents of a text file
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Returns
    /// * `Result<String, String>` - File contents or error message
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, String> {
        fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read file: {}", e))
    }

    /// Reads a file and returns it as a vector of lines
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Returns
    /// * `Result<Vec<String>, String>` - Vector of lines or error message
    pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, String> {
        let content = Self::read_file(path)?;
        Ok(content.lines().map(|s| s.to_string()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    /// Helper function to create a temporary test file
    fn create_test_file(filename: &str, content: &str) -> PathBuf {
        let test_dir = PathBuf::from("target/test_files");
        fs::create_dir_all(&test_dir).unwrap();
        
        let file_path = test_dir.join(filename);
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", content).unwrap();
        
        file_path
    }

    #[test]
    fn test_read_file_basic() {
        let content = "Hello, World!\nThis is a test file.\nWith multiple lines.";
        let test_file = create_test_file("test_basic.txt", content);
        
        let result = FileReader::read_file(&test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_read_file_empty() {
        let test_file = create_test_file("test_empty.txt", "");
        
        let result = FileReader::read_file(&test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_read_lines() {
        let content = "Line 1\nLine 2\nLine 3";
        let test_file = create_test_file("test_lines.txt", content);
        
        let result = FileReader::read_lines(&test_file);
        assert!(result.is_ok());
        
        let lines = result.unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
        assert_eq!(lines[2], "Line 3");
    }

    #[test]
    fn test_read_file_not_found() {
        let result = FileReader::read_file("nonexistent_file.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to read file"));
    }

    #[test]
    fn test_read_lines_with_empty_lines() {
        let content = "Line 1\n\nLine 3\n";
        let test_file = create_test_file("test_empty_lines.txt", content);
        
        let result = FileReader::read_lines(&test_file);
        assert!(result.is_ok());
        
        let lines = result.unwrap();
        assert_eq!(lines.len(), 3); // Empty lines are preserved
        assert_eq!(lines[1], "");
    }
}