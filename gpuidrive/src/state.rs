use std::{ffi::OsString, fs::FileType, os::unix::fs::MetadataExt, path::PathBuf};

use chrono::{DateTime, Local};

pub struct State {
    path: PathBuf,
    nodes: Vec<Node>,
}

/// Represents a node on the filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    path: PathBuf,
    name: OsString,
    kind: NodeKind,
    size: u64,
    created: DateTime<Local>,
    modified: DateTime<Local>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    File,
    Directory,
    Unknown, // TODO: Removing this
             // TODO: Handle symbolic links, etc
}

impl From<FileType> for NodeKind {
    fn from(value: FileType) -> Self {
        if value.is_dir() {
            NodeKind::Directory
        } else if value.is_file() {
            NodeKind::File
        } else {
            NodeKind::Unknown
        }
    }
}

impl State {
    pub fn init() -> Self {
        let mut this = Self {
            path: Default::default(),
            nodes: Default::default(),
        };
        this.set_path(PathBuf::from("/Users/oscar/Desktop")); // TODO: Don't hardcode username
        this
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        let changed = self.path != path;
        self.path = path;

        if changed {
            match std::fs::read_dir(&self.path) {
                Ok(dir) => {
                    // TODO: Error handing
                    // TODO: Is this running off the main thread?
                    self.nodes = dir
                        .map(|entry| {
                            let entry = entry.unwrap();
                            let metadata = entry.metadata().unwrap();

                            Node {
                                path: entry.path(),
                                name: entry.file_name(),
                                kind: entry.file_type().unwrap().into(),
                                size: metadata.size(),
                                created: metadata.created().unwrap().into(),
                                modified: metadata.modified().unwrap().into(),
                            }
                        })
                        .collect();

                    println!("{:?}", self.nodes); // TODO
                }
                Err(_) => self.nodes = vec![], // TODO: Proper error handling
            }
        }
    }
}
