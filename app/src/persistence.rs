use acso7::persistence::PersistenceBackend;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileBackend {
    dir: PathBuf,
}

impl FileBackend {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }
}

impl PersistenceBackend for FileBackend {
    fn save(&self, key: &str, value: Vec<u8>) {
        if !self.dir.exists() {
            std::fs::create_dir_all(&self.dir).unwrap();
        }
        let path = self.dir.join(key).with_extension("tmp");
        std::fs::write(&path, value).unwrap();
        std::fs::rename(path, self.dir.join(key)).unwrap();
    }

    fn load(&self, key: &str) -> Option<Vec<u8>> {
        let path = self.dir.join(key);
        if path.exists() {
            Some(std::fs::read(path).unwrap())
        } else {
            None
        }
    }

    fn delete(&self, key: &str) {
        let path = self.dir.join(key);
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
    }

    fn list(&self) -> Vec<String> {
        if !self.dir.exists() {
            return Vec::new();
        }
        self.dir
            .read_dir()
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect()
    }
}
