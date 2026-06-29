use crate::error::Acos7Result;
use jiff::Zoned;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::io::{Read, Write};

pub trait PersistenceBackend: Debug {
    fn save(&self, key: &str, value: Vec<u8>);
    fn load(&self, key: &str) -> Option<Vec<u8>>;
    fn delete(&self, key: &str);
    fn list(&self) -> Vec<String>;

    fn stamped_save(&self, key: &str, value: Vec<u8>, max_count: usize) {
        let now = Zoned::now();
        let stamp = now.strftime("%Y%m%d%H%M%S%4f");
        let name = format!("autosave_{key}_{stamp}");

        self.save(&name, value);
        self.cleanup_stamped(key, max_count);
    }

    fn load_latest_stamped(&self, key: &str) -> Option<Vec<u8>> {
        let prefix = format!("autosave_{key}_");
        let latest = self
            .list()
            .into_iter()
            .filter(|k| k.starts_with(&prefix))
            .max()?;
        self.load(&latest)
    }

    fn cleanup_stamped(&self, key: &str, max_count: usize) {
        let prefix = format!("autosave_{key}_");
        let mut keys = self
            .list()
            .into_iter()
            .filter(|k| k.starts_with(&prefix))
            .collect::<Vec<_>>();
        keys.sort();
        let remove = keys.len().saturating_sub(max_count);
        for k in &keys[..remove] {
            self.delete(k);
        }
    }
}

#[derive(Debug, Default)]
pub struct NullBackend;
impl PersistenceBackend for NullBackend {
    fn save(&self, _key: &str, _value: Vec<u8>) {}
    fn load(&self, _key: &str) -> Option<Vec<u8>> {
        None
    }
    fn delete(&self, _key: &str) {}
    fn list(&self) -> Vec<String> {
        Vec::new()
    }
}

const COMPRESS_QUALITY: u32 = 9;
const COMPRESS_WINDOW: u32 = 22;
const COMPRESS_BUF: usize = 4096;

pub trait CompressedSerde: Serialize + DeserializeOwned {
    fn to_compressed(&self) -> Acos7Result<Vec<u8>> {
        let packed = rmp_serde::to_vec_named(self)?;
        let mut out = Vec::new();
        {
            let mut w = brotli::CompressorWriter::new(
                &mut out,
                COMPRESS_BUF,
                COMPRESS_QUALITY,
                COMPRESS_WINDOW,
            );
            w.write_all(&packed)?;
        }
        Ok(out)
    }

    fn from_compressed(bytes: &[u8]) -> Acos7Result<Self> {
        let mut packed = Vec::new();
        brotli::Decompressor::new(bytes, COMPRESS_BUF).read_to_end(&mut packed)?;
        Ok(rmp_serde::from_slice(&packed)?)
    }
}

impl<T: Serialize + DeserializeOwned> CompressedSerde for T {}
