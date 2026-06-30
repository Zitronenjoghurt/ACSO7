use acso7::persistence::PersistenceBackend;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileBackend {
    dir: PathBuf,
}

impl FileBackend {
    pub fn new(dir: PathBuf) -> Self {
        let backend = Self { dir };
        backend.sweep_temps(&backend.dir);
        backend
    }

    fn sweep_temps(&self, dir: &Path) {
        let Ok(entries) = dir.read_dir() else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                self.sweep_temps(&path);
            } else if path.extension().is_some_and(|ext| ext == "tmp") {
                let _ = std::fs::remove_file(&path);
            }
        }
    }

    fn dir_of(&self, key: &[&str]) -> PathBuf {
        let mut path = self.dir.clone();
        for &segment in key {
            path.push(segment);
        }
        path
    }

    fn file_of(&self, key: &[&str]) -> PathBuf {
        let mut path = self.dir_of(key);
        path.set_extension(EXT);
        path
    }
}

const EXT: &str = "dat";

impl PersistenceBackend for FileBackend {
    fn save(&self, key: &[&str], value: Vec<u8>) {
        let path = self.file_of(key);
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();

        let tmp = path.with_extension("tmp");
        {
            let mut file = std::fs::File::create(&tmp).unwrap();
            file.write_all(&value).unwrap();
            file.sync_all().unwrap();
        }
        std::fs::rename(&tmp, &path).unwrap();
        if let Ok(dir) = std::fs::File::open(parent) {
            let _ = dir.sync_all();
        }
    }

    fn load(&self, key: &[&str]) -> Option<Vec<u8>> {
        let path = self.file_of(key);
        path.is_file().then(|| std::fs::read(path).unwrap())
    }

    fn delete(&self, key: &[&str]) {
        let path = self.file_of(key);
        if path.is_file() {
            std::fs::remove_file(&path).unwrap();
        }
        let mut current = path.parent();
        while let Some(dir) = current {
            if dir == self.dir {
                break;
            }
            match std::fs::remove_dir(dir) {
                Ok(()) => current = dir.parent(),
                Err(_) => break,
            }
        }
    }

    fn list(&self, prefix: &[&str]) -> Vec<String> {
        let Ok(entries) = self.dir_of(prefix).read_dir() else {
            return Vec::new();
        };
        entries
            .filter_map(|e| {
                let name = e.ok()?.file_name().into_string().ok()?;
                if name.ends_with(".tmp") {
                    return None;
                }
                Some(
                    name.strip_suffix(&format!(".{EXT}"))
                        .map(String::from)
                        .unwrap_or(name),
                )
            })
            .collect()
    }
}
