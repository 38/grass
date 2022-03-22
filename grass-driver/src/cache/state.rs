#![allow(dead_code, unused_variables)]

use std::{
    io::{Result, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use file_lock::{FileLock, FileOptions};
use serde::{Deserialize, Serialize};

use super::config::CacheConfig;


#[derive(Serialize, Deserialize, Default)]
pub(crate) struct CacheState {
    disk_space_used: u64,
    cache_state: Vec<String>,
    #[serde(skip)]
    cache_root: PathBuf,
    #[serde(skip)]
    lock: Option<FileLock>,
    #[serde(skip)]
    config: CacheConfig,
}

impl CacheState {
    const STATE_FILE_NAME : &'static str = "cache_state.json";
    fn lock_cache_state<P: AsRef<Path>>(root: &P) -> Result<FileLock> {
        let mut cache_state_file = root.as_ref().to_path_buf();
        cache_state_file.push(Self::STATE_FILE_NAME);
        let lock_opt = FileOptions::new()
            .read(true)
            .write(true);
        Ok(FileLock::lock(&cache_state_file, true, lock_opt)?)
    }

    fn compose_binary_path(&self, hash: &str) -> PathBuf {
        let mut binary_file = self.cache_root.clone();
        binary_file.push(&hash[..2]);
        binary_file.push(&hash);
        binary_file
    }

    fn write_state_file(&mut self) -> Result<()> {
        let mut lock = self.lock.take().unwrap();
        lock.file.set_len(0)?;
        lock.file.seek(SeekFrom::Start(0))?;
        serde_json::to_writer(&mut lock.file, self)?;
        self.lock = Some(lock);
        Ok(())
    }

    pub fn load_cache<P: AsRef<Path>>(root: &P) -> Result<Self> {
        let mut state_path = root.as_ref().to_path_buf();
        state_path.push(Self::STATE_FILE_NAME);
        
        if !root.as_ref().exists() ||  !state_path.exists() {
            std::fs::DirBuilder::new().recursive(true).create(root)?;
            serde_json::to_writer(std::fs::File::create(state_path)?, &Self::default())?;
        }

        let mut lock = Self::lock_cache_state(root)?;
        let mut ret: CacheState = serde_json::from_reader(&mut lock.file)?;
        ret.lock = Some(lock);
        ret.cache_root = root.as_ref().to_path_buf();

        let mut config_path = root.as_ref().to_path_buf();
        config_path.push("config.json");
        ret.config = CacheConfig::load_cache_config(&config_path).unwrap_or_default();

        Ok(ret)
    }

    pub fn query_cache_entry(&mut self, hash_code: &str, buf: &mut PathBuf) -> Result<bool> {
        let binary_file = self.compose_binary_path(hash_code);
        if binary_file.exists() {
            if let Some((mut idx, _)) = {
                self.cache_state
                    .iter()
                    .enumerate()
                    .find(|(_, current)| hash_code == current.as_str()) 
            }{
                while idx > 0 {
                    self.cache_state.swap(idx, idx - 1);
                    idx -= 1;
                }
            }
            self.write_state_file()?;
            *buf = binary_file;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn update_cache<BuildFn: FnOnce(&mut PathBuf) -> Result<()>>(
        &mut self,
        hash_code: &str,
        build: BuildFn,
        buf: &mut PathBuf,
    ) -> Result<()> {
        let mut artifact_path = PathBuf::new();
        build(&mut artifact_path)?;

        let file_size = std::fs::metadata(&artifact_path)?.len();
        let size_limit = self.config.disk_space_limit();

        while size_limit < file_size + self.disk_space_used {
            if let Some(victim) = self.cache_state.pop() {
                let path = self.compose_binary_path(&victim);
                let victim_size = std::fs::metadata(&path)?.len();
                self.disk_space_used -= victim_size;
            } else {
                break;
            }
        }

        let binary_path = self.compose_binary_path(hash_code);

        if let Some(parent) = binary_path.parent() {
            std::fs::DirBuilder::new()
                .recursive(true)
                .create(parent)?;
        }

        std::fs::copy(artifact_path.as_path(), binary_path.as_path())?;

        self.cache_state.push(hash_code.to_string());
        
        let mut idx = self.cache_state.len() - 1;
        while idx > 0 {
            self.cache_state.swap(idx - 1, idx);
            idx -= 1;
        }

        self.disk_space_used += file_size;

        self.write_state_file()?;

        *buf = binary_path;

        Ok(())
    }
}
