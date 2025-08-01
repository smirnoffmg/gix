use crate::models::GixError;
use std::fs;
use std::path::Path;

/// Read a .gitignore file safely
pub fn read_gitignore_file(path: &Path) -> Result<String, GixError> {
    if !path.exists() {
        return Err(GixError::FileNotFound(path.to_string_lossy().to_string()));
    }

    fs::read_to_string(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            GixError::PermissionDenied(path.to_string_lossy().to_string())
        } else {
            GixError::IoError(e)
        }
    })
}

/// Write a .gitignore file safely with atomic operation
pub fn write_gitignore_file(path: &Path, content: &str) -> Result<(), GixError> {
    // Create a temporary file in the same directory
    let temp_path = path.with_extension("tmp");

    // Write to temporary file first
    fs::write(&temp_path, content).map_err(GixError::IoError)?;

    // Atomically rename the temporary file to the target file
    fs::rename(&temp_path, path).map_err(|e| {
        // Clean up temp file on error
        let _ = fs::remove_file(&temp_path);
        GixError::IoError(e)
    })
}

/// Create a backup of the original .gitignore file
pub fn create_backup(path: &Path) -> Result<(), GixError> {
    if !path.exists() {
        return Ok(()); // Nothing to backup
    }

    let backup_path = path.with_extension("backup");
    fs::copy(path, &backup_path).map_err(GixError::IoError)?;

    Ok(())
}

/// Check if a file is a .gitignore file
pub fn is_gitignore_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == ".gitignore")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_gitignore_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let content = "*.log\nbuild/";
        writeln!(temp_file.as_file(), "{}", content).unwrap();

        let result = read_gitignore_file(temp_file.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), content);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().with_file_name("nonexistent");

        let result = read_gitignore_file(&path);
        assert!(result.is_err());
        match result.unwrap_err() {
            GixError::FileNotFound(_) => {}
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_write_gitignore_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let content = "*.log\nbuild/";

        let result = write_gitignore_file(temp_file.path(), content);
        assert!(result.is_ok());

        let read_content = fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_create_backup() {
        let temp_file = NamedTempFile::new().unwrap();
        let content = "*.log\nbuild/";
        writeln!(temp_file.as_file(), "{}", content).unwrap();

        let result = create_backup(temp_file.path());
        assert!(result.is_ok());

        let backup_path = temp_file.path().with_extension("backup");
        assert!(backup_path.exists());

        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content.trim(), content);
    }

    #[test]
    fn test_is_gitignore_file() {
        let gitignore_path = Path::new(".gitignore");
        let other_path = Path::new("README.md");

        assert!(is_gitignore_file(gitignore_path));
        assert!(!is_gitignore_file(other_path));
    }
}
