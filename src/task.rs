use crate::error::{RalphError, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct TaskContext {
    pub dir: PathBuf,
    pub prd_file: PathBuf,
    pub progress_file: PathBuf,
}

impl TaskContext {
    pub fn discover(input: &str) -> Result<Self> {
        let dir = Self::resolve_directory(input)?;
        let prd_file = Self::find_prd_file(&dir)?;
        let progress_file = Self::find_or_create_progress_file(&dir)?;

        Ok(Self {
            dir,
            prd_file,
            progress_file,
        })
    }

    pub fn discover_from_prd(prd_input: &str) -> Result<Self> {
        let prd_file = Self::resolve_prd_file(prd_input)?;
        let dir = prd_file
            .parent()
            .ok_or_else(|| RalphError::NoPrdFile {
                dir: prd_file.clone(),
            })?
            .to_path_buf();
        let progress_file = Self::find_or_create_progress_file(&dir)?;

        Ok(Self {
            dir,
            prd_file,
            progress_file,
        })
    }

    fn resolve_directory(input: &str) -> Result<PathBuf> {
        let path = Path::new(input);
        if path.is_dir() {
            return Ok(path.to_path_buf());
        }

        let task_path = Path::new(".ai/tasks").join(input);
        if task_path.is_dir() {
            return Ok(task_path);
        }

        Err(RalphError::TaskNotFound {
            path: path.to_path_buf(),
        })
    }

    fn resolve_prd_file(input: &str) -> Result<PathBuf> {
        let path = Path::new(input);
        if path.is_file() {
            return Ok(path.to_path_buf());
        }

        let task_prd_path = Path::new(".ai/tasks").join(input).join("PRD.md");
        if task_prd_path.is_file() {
            return Ok(task_prd_path);
        }

        Err(RalphError::NoPrdFile {
            dir: path.to_path_buf(),
        })
    }

    fn find_prd_file(dir: &Path) -> Result<PathBuf> {
        for entry in fs::read_dir(dir).map_err(|_| RalphError::TaskNotFound {
            path: dir.to_path_buf(),
        })? {
            let entry = entry.map_err(|_| RalphError::TaskNotFound {
                path: dir.to_path_buf(),
            })?;
            let name = entry.file_name().to_string_lossy().to_uppercase();
            if name.contains("PRD") && entry.path().is_file() {
                return Ok(entry.path());
            }
        }
        Err(RalphError::NoPrdFile {
            dir: dir.to_path_buf(),
        })
    }

    fn find_or_create_progress_file(dir: &Path) -> Result<PathBuf> {
        // First try to find existing PROGRESS file
        for entry in fs::read_dir(dir).map_err(|_| RalphError::TaskNotFound {
            path: dir.to_path_buf(),
        })? {
            let entry = entry.map_err(|_| RalphError::TaskNotFound {
                path: dir.to_path_buf(),
            })?;
            let name = entry.file_name().to_string_lossy().to_uppercase();
            if name.contains("PROGRESS") && entry.path().is_file() {
                return Ok(entry.path());
            }
        }

        // Create new PROGRESS.md if not found
        let progress_path = dir.join("PROGRESS.md");
        fs::write(&progress_path, "# Progress\n\n").ok();
        println!("Created empty PROGRESS.md at: {}", progress_path.display());
        Ok(progress_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_directory_absolute_path() {
        let temp = tempdir().unwrap();
        let result = TaskContext::resolve_directory(temp.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp.path());
    }

    #[test]
    fn test_resolve_directory_nonexistent() {
        let result = TaskContext::resolve_directory("/nonexistent/path/that/does/not/exist");
        assert!(result.is_err());
        match result {
            Err(RalphError::TaskNotFound { path }) => {
                assert_eq!(path, PathBuf::from("/nonexistent/path/that/does/not/exist"));
            }
            _ => panic!("Expected TaskNotFound error"),
        }
    }

    #[test]
    fn test_find_prd_file_exists() {
        let temp = tempdir().unwrap();
        let prd_path = temp.path().join("PRD.md");
        fs::write(&prd_path, "# PRD").unwrap();

        let result = TaskContext::find_prd_file(temp.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), prd_path);
    }

    #[test]
    fn test_find_prd_file_lowercase() {
        let temp = tempdir().unwrap();
        let prd_path = temp.path().join("prd.md");
        fs::write(&prd_path, "# PRD").unwrap();

        let result = TaskContext::find_prd_file(temp.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), prd_path);
    }

    #[test]
    fn test_find_prd_file_missing() {
        let temp = tempdir().unwrap();
        let result = TaskContext::find_prd_file(temp.path());
        assert!(result.is_err());
        match result {
            Err(RalphError::NoPrdFile { dir }) => {
                assert_eq!(dir, temp.path());
            }
            _ => panic!("Expected NoPrdFile error"),
        }
    }

    #[test]
    fn test_find_or_create_progress_file_exists() {
        let temp = tempdir().unwrap();
        let progress_path = temp.path().join("PROGRESS.md");
        fs::write(&progress_path, "# Progress").unwrap();

        let result = TaskContext::find_or_create_progress_file(temp.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), progress_path);
    }

    #[test]
    fn test_find_or_create_progress_file_creates_new() {
        let temp = tempdir().unwrap();
        let result = TaskContext::find_or_create_progress_file(temp.path());
        assert!(result.is_ok());

        let expected_path = temp.path().join("PROGRESS.md");
        assert_eq!(result.unwrap(), expected_path);
        assert!(expected_path.exists());
    }

    #[test]
    fn test_discover_full_context() {
        let temp = tempdir().unwrap();
        let prd_path = temp.path().join("PRD.md");
        let progress_path = temp.path().join("PROGRESS.md");
        fs::write(&prd_path, "# PRD").unwrap();
        fs::write(&progress_path, "# Progress").unwrap();

        let result = TaskContext::discover(temp.path().to_str().unwrap());
        assert!(result.is_ok());

        let ctx = result.unwrap();
        assert_eq!(ctx.dir, temp.path());
        assert_eq!(ctx.prd_file, prd_path);
        assert_eq!(ctx.progress_file, progress_path);
    }
}
