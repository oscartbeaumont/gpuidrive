use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Instant;

// Custom error type to handle different kinds of errors
#[derive(Debug)]
enum FileCollectionError {
    IoError(io::Error, PathBuf),
    PermissionDenied(PathBuf),
    InvalidFileName(PathBuf),
    Other(String),
}

impl fmt::Display for FileCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileCollectionError::IoError(err, path) => {
                write!(f, "IO error for path {:?}: {}", path, err)
            }
            FileCollectionError::PermissionDenied(path) => {
                write!(f, "Permission denied for path {:?}", path)
            }
            FileCollectionError::InvalidFileName(path) => {
                write!(f, "Invalid file name at path {:?}", path)
            }
            FileCollectionError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for FileCollectionError {}

#[derive(Debug)]
struct FileInfo {
    size: u64,
    path: PathBuf,
}

struct FileCollection {
    files: HashMap<String, FileInfo>,
    errors: Vec<FileCollectionError>,
}

impl FileCollection {
    fn new() -> Self {
        FileCollection {
            files: HashMap::new(),
            errors: Vec::new(),
        }
    }

    fn add_error(&mut self, error: FileCollectionError) {
        self.errors.push(error);
    }

    fn collect_files(&mut self, start_path: &Path) {
        self.collect_files_recursive(start_path);
    }

    fn collect_files_recursive(&mut self, dir: &Path) {
        // Handle directory access
        let read_dir = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(err) => {
                let error = if err.kind() == io::ErrorKind::PermissionDenied {
                    FileCollectionError::PermissionDenied(dir.to_path_buf())
                } else {
                    FileCollectionError::IoError(err, dir.to_path_buf())
                };
                self.add_error(error);
                return;
            }
        };

        // Process each entry
        for entry_result in read_dir {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path();

                    if path.is_dir() {
                        self.collect_files_recursive(&path);
                    } else {
                        self.process_file(&path);
                    }
                }
                Err(err) => {
                    self.add_error(FileCollectionError::IoError(err, dir.to_path_buf()));
                }
            }
        }
    }

    fn process_file(&mut self, path: &Path) {
        // Get file name
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => {
                self.add_error(FileCollectionError::InvalidFileName(path.to_path_buf()));
                return;
            }
        };

        // Get file metadata
        match fs::metadata(path) {
            Ok(metadata) => {
                self.files.insert(
                    file_name,
                    FileInfo {
                        size: metadata.len(),
                        path: path.to_path_buf(),
                    },
                );
            }
            Err(err) => {
                self.add_error(FileCollectionError::IoError(err, path.to_path_buf()));
            }
        }
    }

    fn print_summary(&self) {
        println!("\n=== File Collection Summary ===");
        println!("Successfully processed {} files:", self.files.len());
        for (name, info) in &self.files {
            println!("  {} ({} bytes)", name, info.size);
            println!("    Path: {:?}", info.path);
        }

        if !self.errors.is_empty() {
            println!("\nEncountered {} errors:", self.errors.len());
            for (i, error) in self.errors.iter().enumerate() {
                println!("{}. {}", i + 1, error);
            }
        }
    }
}

fn main() {
    println!("Starting file collection...");
    let now = Instant::now();

    let start_path = Path::new("/");
    let mut collection = FileCollection::new();

    collection.collect_files(start_path);
    // collection.print_summary();

    println!("DONE {:?}", now);
}
