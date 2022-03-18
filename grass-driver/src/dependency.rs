use std::{
    path::PathBuf,
    io::{Result, Write},
};

use serde::{Deserialize, Serialize};

use crate::return_true;

#[derive(Serialize ,Deserialize, Clone)]
#[serde(tag = "dep-kind", content = "value")]
pub enum DependencySource {
    Git(String),
    Local(PathBuf),
    CratesIO,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    source: DependencySource,
    #[serde(default)]
    version: Option<String>,
    features: Vec<String>,
    #[serde(default = "return_true")]
    default_features: bool,
}

impl Dependency {
    pub fn write_dependency_line<W:Write>(&self, mut target: W) -> Result<()>{
        write!(target, "{name}={{version = \"{version}\", default_features = {df}, features = [{features}]", 
            name = self.name,
            version = self.version.as_ref().map(String::as_str).unwrap_or("*"),
            df = if self.default_features { "true" } else { "false" },
            features = self.features.iter().map(|what| format!("\"{}\"", what)).collect::<Vec<_>>().join(","),
        )?;
        match &self.source {
            DependencySource::Git(url) => write!(target, ", git = \"{}\"}}", url),
            DependencySource::Local(path) => write!(target, ", path = \"{}\"}}", path.as_os_str().to_string_lossy()),
            DependencySource::CratesIO => write!(target, "}}"),
        }?;
        writeln!(target, "")
    }
    pub fn create_grass_dep(component: &str, source: &DependencySource) -> Self {
        Self {
            name: component.to_string(),
            source: source.clone(),
            version: None,
            features: vec![],
            default_features: true,
        }
    }
}
