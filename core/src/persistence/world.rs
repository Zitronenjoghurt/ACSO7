use super::{CompressedSerde, PersistenceBackend};
use crate::error::Acos7Result;
use crate::world::{World, WorldMeta};
use jiff::Zoned;

const WORLDS: &str = "worlds";
const AUTO: &str = "auto";

fn auto_ns(id: &str) -> [&str; 3] {
    [WORLDS, id, AUTO]
}

fn auto_key<'a>(id: &'a str, stamp: &'a str) -> [&'a str; 4] {
    [WORLDS, id, AUTO, stamp]
}

fn stamp() -> String {
    Zoned::now().strftime("%Y%m%d%H%M%S%4f").to_string()
}

pub struct WorldStore<'a> {
    backend: &'a dyn PersistenceBackend,
}

impl<'a> WorldStore<'a> {
    pub fn new(backend: &'a dyn PersistenceBackend) -> Self {
        Self { backend }
    }

    pub fn list_meta(&self) -> Vec<WorldMeta> {
        let mut metas: Vec<WorldMeta> = self
            .backend
            .list(&[WORLDS])
            .into_iter()
            .filter_map(|id| self.load_latest(&id).map(|world| world.meta))
            .collect();
        metas.sort_by_key(|b| std::cmp::Reverse(b.last_played));
        metas
    }

    pub fn save(&self, world: &World, max_count: usize) -> Acos7Result<()> {
        let id = &world.meta.id;
        self.backend
            .save(&auto_key(id, &stamp()), world.to_compressed()?);
        self.rotate(id, max_count);
        Ok(())
    }

    pub fn load_latest(&self, id: &str) -> Option<World> {
        let mut stamps = self.backend.list(&auto_ns(id));
        stamps.sort();
        for stamp in stamps.into_iter().rev() {
            if let Some(bytes) = self.backend.load(&auto_key(id, &stamp))
                && let Ok(world) = World::from_compressed(&bytes)
            {
                return Some(world);
            }
        }
        None
    }

    pub fn delete(&self, id: &str) {
        for stamp in self.backend.list(&auto_ns(id)) {
            self.backend.delete(&auto_key(id, &stamp));
        }
    }

    fn rotate(&self, id: &str, max_count: usize) {
        let mut stamps = self.backend.list(&auto_ns(id));
        stamps.sort();
        let remove = stamps.len().saturating_sub(max_count);
        for stamp in &stamps[..remove] {
            self.backend.delete(&auto_key(id, stamp));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    use std::thread::sleep;
    use std::time::Duration;

    #[derive(Debug, Default)]
    struct MemBackend {
        store: Mutex<BTreeMap<String, Vec<u8>>>,
    }

    impl PersistenceBackend for MemBackend {
        fn save(&self, key: &[&str], value: Vec<u8>) {
            self.store.lock().unwrap().insert(key.join("/"), value);
        }
        fn load(&self, key: &[&str]) -> Option<Vec<u8>> {
            self.store.lock().unwrap().get(&key.join("/")).cloned()
        }
        fn delete(&self, key: &[&str]) {
            self.store.lock().unwrap().remove(&key.join("/"));
        }
        fn list(&self, prefix: &[&str]) -> Vec<String> {
            let joined = prefix.join("/");
            let needle = if joined.is_empty() {
                String::new()
            } else {
                format!("{joined}/")
            };
            let mut children: Vec<String> = self
                .store
                .lock()
                .unwrap()
                .keys()
                .filter_map(|k| k.strip_prefix(&needle))
                .map(|rest| rest.split('/').next().unwrap().to_string())
                .collect();
            children.sort();
            children.dedup();
            children
        }
    }

    fn tick() {
        sleep(Duration::from_millis(1));
    }

    #[test]
    fn autosaves_are_namespaced_per_world() {
        let backend = MemBackend::default();
        let store = WorldStore::new(&backend);

        let a = World::new("Endeavour");
        tick();
        let b = World::new("Nostromo");

        store.save(&a, 10).unwrap();
        tick();
        store.save(&a, 10).unwrap();
        store.save(&b, 10).unwrap();

        assert_eq!(backend.list(&auto_ns(&a.meta.id)).len(), 2);
        assert_eq!(backend.list(&auto_ns(&b.meta.id)).len(), 1);
        assert_eq!(store.list_meta().len(), 2);
    }

    #[test]
    fn rotation_keeps_only_the_newest() {
        let backend = MemBackend::default();
        let store = WorldStore::new(&backend);
        let w = World::new("Endeavour");

        for _ in 0..5 {
            store.save(&w, 3).unwrap();
            tick();
        }

        assert_eq!(backend.list(&auto_ns(&w.meta.id)).len(), 3);
        assert!(store.load_latest(&w.meta.id).is_some());
    }

    #[test]
    fn load_latest_skips_corrupt_snapshots() {
        let backend = MemBackend::default();
        let store = WorldStore::new(&backend);
        let w = World::new("Endeavour");
        store.save(&w, 10).unwrap();

        backend.save(
            &auto_key(&w.meta.id, "99999999999999999999"),
            vec![0xff, 0x00, 0x13],
        );

        let loaded = store.load_latest(&w.meta.id);
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().meta.id, w.meta.id);
    }
}
