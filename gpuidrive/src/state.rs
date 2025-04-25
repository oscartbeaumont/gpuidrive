use std::path::PathBuf;

pub struct State {
    path: PathBuf,
    // TODO: make this reactive to `path` changing
    files: Vec<String>,
}

impl State {
    pub fn init() -> Self {
        Self {
            path: "/Users/oscar/Desktop".into(), // TODO: Don't hardcode
            files: vec![],
        }
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        let changed = self.path != path;
        self.path = path;

        if changed {
            // TODO: Error handing + run off the main thread
            self.files = std::fs::read_dir(&self.path)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                .collect();
        }
    }
}
