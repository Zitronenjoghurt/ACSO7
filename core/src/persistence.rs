pub mod world;

pub use world::WorldStore;

use crate::error::Acos7Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::io::{Read, Write};

pub trait PersistenceBackend: Debug {
    fn save(&self, key: &[&str], value: Vec<u8>);
    fn load(&self, key: &[&str]) -> Option<Vec<u8>>;
    fn delete(&self, key: &[&str]);
    fn list(&self, prefix: &[&str]) -> Vec<String>;
}

#[derive(Debug, Default)]
pub struct NullBackend;
impl PersistenceBackend for NullBackend {
    fn save(&self, _key: &[&str], _value: Vec<u8>) {}
    fn load(&self, _key: &[&str]) -> Option<Vec<u8>> {
        None
    }
    fn delete(&self, _key: &[&str]) {}
    fn list(&self, _prefix: &[&str]) -> Vec<String> {
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
