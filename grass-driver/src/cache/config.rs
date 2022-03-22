use std::{path::Path, io::Result, fs::File};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CacheConfig {
    /// The amount of disk space to use
    disk_space_limit: f64,
    disk_space_limit_unit: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            disk_space_limit: 4.0,
            disk_space_limit_unit: "G".to_string(),
        }
    }
}

impl CacheConfig {
    pub fn load_cache_config<P: AsRef<Path>>(path: &P) -> Result<CacheConfig> {
        Ok(serde_json::from_reader(File::open(path)?)?)
    }
    pub fn disk_space_limit(&self) -> u64 {
        let size = match self.disk_space_limit_unit.as_str() {
            "T" | "t" => (1024f64).powi(4),
            "G" | "g" => (1024f64).powi(3),
            "M" | "m" => (1024f64).powi(2),
            "K" | "k" => (1024f64).powi(1),
            _ => 1.0
        } * self.disk_space_limit as f64;
        size.round() as u64
    }
}