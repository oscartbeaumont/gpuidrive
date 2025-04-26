use anyhow::Result;
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
use walkdir::WalkDir;

/// Represents the results of counting files in a directory
#[derive(Debug, Clone, Copy)]
pub struct FileCountResult {
    /// Number of files found
    pub file_count: usize,
    /// Number of filesystem errors encountered
    pub error_count: usize,
}

/// Counts files in a directory recursively using parallel processing
///
/// This function will traverse the given directory recursively and count:
/// - Total number of files (not directories)
/// - Number of filesystem errors encountered during traversal
///
/// # Arguments
///
/// * `path` - The directory path to count files in
///
/// # Returns
///
/// Returns a `Result` containing `FileCountResult` with the counts
///
/// # Example
///
/// ```rust
/// use gpuidrive::file_counter::count_files;
///
/// let result = count_files("some/directory").unwrap();
/// println!("Found {} files with {} errors", result.file_count, result.error_count);
/// ```
pub fn count_files<P: AsRef<Path>>(path: P) -> Result<FileCountResult> {
    let file_count = AtomicUsize::new(0);
    let error_count = AtomicUsize::new(0);

    // Create an iterator over the directory entries
    let walker = WalkDir::new(path).into_iter();

    // Process entries in parallel
    walker.par_bridge().for_each(|entry| {
        match entry {
            Ok(entry) => {
                // Only count files, not directories
                if entry.file_type().is_file() {
                    file_count.fetch_add(1, Ordering::Relaxed);
                }
            }
            Err(_) => {
                error_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    });

    Ok(FileCountResult {
        file_count: file_count.load(Ordering::Relaxed),
        error_count: error_count.load(Ordering::Relaxed),
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::{self, File};
//     use std::io;
//     use tempfile::tempdir;

//     #[test]
//     fn test_count_files() -> io::Result<()> {
//         // Create a temporary directory structure
//         let temp_dir = tempdir()?;
//         let temp_path = temp_dir.path();

//         // Create some test files and directories
//         File::create(temp_path.join("file1.txt"))?;
//         File::create(temp_path.join("file2.txt"))?;

//         let subdir = temp_path.join("subdir");
//         fs::create_dir(&subdir)?;
//         File::create(subdir.join("file3.txt"))?;

//         // Count files
//         let result = count_files(temp_path).unwrap();

//         assert_eq!(result.file_count, 3);
//         assert_eq!(result.error_count, 0);

//         Ok(())
//     }
// }

fn main() {
    let result = count_files(PathBuf::from("/Users/oscar")).unwrap();
    println!("{:?}", result);
}
