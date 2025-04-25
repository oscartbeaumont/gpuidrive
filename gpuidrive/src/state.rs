use std::{ffi::OsString, fs::FileType, os::unix::fs::MetadataExt, path::PathBuf};

use chrono::{DateTime, Local};
use gpui::{Context, EventEmitter};

pub struct State {
    path: PathBuf,
    nodes: Vec<Node>,
}

/// Represents a node on the filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub path: PathBuf,
    pub name: OsString,
    pub kind: NodeKind,
    pub size: u64,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
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
    pub fn init(cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            path: Default::default(),
            nodes: Default::default(),
        };
        this.set_path(cx, PathBuf::from("/Users/oscar/Desktop")); // TODO: Don't hardcode username
        this.set_path(
            cx,
            PathBuf::from("/Users/oscar/Library/pnpm/store/v10/files"),
        );
        this
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn set_path(&mut self, cx: &mut Context<Self>, path: PathBuf) {
        let changed = self.path != path;
        self.path = path;
        cx.emit(PathChange);
        cx.notify();

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
                }
                Err(_) => self.nodes = vec![], // TODO: Proper error handling
            }
        }
    }
}

pub struct PathChange;
impl EventEmitter<PathChange> for State {}
