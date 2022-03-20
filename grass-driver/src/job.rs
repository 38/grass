
use std::{path::{PathBuf, Path}, process::{Command, Child}, io::Result, fs::File, io::{Write, Error}, collections::HashMap};


use grass_ir::GrassIR;
use serde::Deserialize;
use sha::{sha256::Sha256, utils::DigestExt};
use tempfile::TempDir;

use crate::dependency::{Dependency, DependencySource};

use crate::return_true;

#[derive(Deserialize)]
pub enum BuildFlavor {
    Debug,
    Release
}

impl Default for BuildFlavor {
    fn default() -> Self {
        Self::Release
    }
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct JobDefinition {
    ir: Vec<GrassIR>,
    #[serde(default)]
    deps: Vec<Dependency>,
    working_dir: PathBuf,
    #[serde(default)]
    cmdline_args: Vec<String>,
    #[serde(default)]
    env_vars: HashMap<String, String>,
    #[serde(default = "default_runtime")]
    runtime_package: String,
    #[serde(default = "default_macro")]
    macro_package: String,
    runtime_source: DependencySource,
    macro_source: DependencySource,
    #[serde(default = "default_version")]
    version: String,
    #[serde(default)]
    build_flavor: BuildFlavor,
    #[serde(default)]
    temp_dir: Option<PathBuf>,
    #[serde(default = "return_true")]
    use_cache: bool,
    #[serde(default = "return_true")]
    update_cache: bool,
    #[serde(default = "default_cache_root")]
    cache_root: PathBuf,

    // ################ Runtime values ####################
    #[serde(default)]
    tool_chain_path: Option<PathBuf>,
    #[serde(skip)]
    compilation_dir: Option<TempDir>,
    #[serde(skip)]
    ir_hash: String,
    #[serde(skip)]
    artifact_path: Option<PathBuf>,
}

fn default_version() -> String {
    "0.0.1".to_string()
}

fn default_runtime() -> String {
    "grass-runtime".to_string()
}

fn default_macro() -> String {
    "grass-macro".to_string()
}

fn default_cache_root() -> PathBuf {
    let mut ret:PathBuf = std::env::var("HOME").unwrap_or_else(|_| "/".to_string()).into();
    ret.push(".grass-ql");
    ret.push("cache");
    ret
}

impl JobDefinition {
    pub fn get_stderr_log(&mut self) -> Result<File> {
        File::open(self.get_compilation_dir()?.join("stderr.log"))
    }
    pub fn cargo(&mut self, args: &[&str], capture_stdout: bool) -> Result<Child> {
        let mut cmd = Command::new("cargo");
        
        if let Some(tool_chain_path) = self.tool_chain_path.as_ref() {
            cmd.env("PATH", tool_chain_path);
        }

        cmd.args(args);

        let comp_dir = self.get_compilation_dir()?;
        cmd.current_dir(comp_dir);

        let err_log = File::create(comp_dir.join("stderr.log"))?;
        cmd.stderr(err_log);

        if capture_stdout { 
            let out_log = File::create(comp_dir.join("stdout.txt"))?;
            cmd.stdout(out_log);
        }

        cmd.spawn()
    }
    fn populate_ir_hash(&mut self) -> Result<()> {
        let mut hasher = Sha256::default();
        serde_json::to_writer(&mut hasher, &self.ir)?;

        let mut deps = self.deps.clone();
        deps.sort_by_key(|x| x.name.clone());

        serde_json::to_writer(&mut hasher, &deps)?;
        
        let ir_hash = hasher.to_hex();

        self.ir_hash = ir_hash;
        Ok(())
    }
    fn get_artifact_name(&mut self) -> Result<String> {
        if self.ir_hash.len() == 0 {
            self.populate_ir_hash()?;
        }
        Ok(format!("grass-artifact-{}", self.ir_hash))
    }
    fn get_artifact_path(&mut self) -> Result<PathBuf> {
        let mut ret = self.get_compilation_dir()?.to_path_buf();
        ret.push("target");
        match self.build_flavor {
            BuildFlavor::Debug => ret.push("debug"),
            BuildFlavor::Release => ret.push("release"),
        }
        ret.push(self.get_artifact_name()?);
        Ok(ret)
    }
    fn write_manifest_file(&mut self, root: &Path) -> Result<()> {
        let manifest_path = root.join("Cargo.toml");
        let mut manifest_file = File::create(&manifest_path)?;
        writeln!(&mut manifest_file, "[package]")?;
        writeln!(&mut manifest_file, "name = \"{}\"", self.get_artifact_name()?)?;
        writeln!(&mut manifest_file, "version = \"{}\"", self.version)?;
        writeln!(&mut manifest_file, "edition = \"2021\"")?;
        writeln!(&mut manifest_file, "[dependencies]")?;
        for dep in self.deps.iter().chain([
            &Dependency::create_grass_dep(&self.runtime_package, &self.runtime_source),
            &Dependency::create_grass_dep(&self.macro_package, &self.macro_source)
        ]) {
            dep.write_dependency_line(&mut manifest_file)?;
        }
        Ok(())
    }
    fn write_source_code(&self, root: &Path) -> Result<()> {
        let source_dir = root.join("src");
        std::fs::create_dir(&source_dir)?;
        
        let source_path = source_dir.join("main.rs");
        let mut source_file = File::create(source_path)?;

        for (id, ir) in self.ir.iter().enumerate() {
            let ir_path = source_dir.as_path().join(format!("grass_ir_{}.json", id));
            let ir_file = File::create(&ir_path)?;
            serde_json::to_writer(ir_file, ir)?;

            writeln!(&mut source_file, "fn grass_query_{id}() -> Result<(), Box<dyn std::error::Error>> {{", id = id)?;
            writeln!(&mut source_file, "    grass_macro::import_grass_ir_from_file!(\"{ir_file}\");", ir_file = ir_path.as_os_str().to_string_lossy())?;
            writeln!(&mut source_file, "    Ok(())")?;
            writeln!(&mut source_file, "}}")?;
        }

        writeln!(&mut source_file, "fn main() -> Result<(), Box<dyn std::error::Error>> {{")?;
        for id in 0..self.ir.len() {
            writeln!(&mut source_file, "    grass_query_{id}()?;", id = id)?;
        }
        writeln!(&mut source_file, "    Ok(())")?;
        writeln!(&mut source_file, "}}")?;

        Ok(())
    }
    pub fn get_compilation_dir(&mut self) -> Result<&Path> {
        if self.compilation_dir.is_some() {
            Ok(self.compilation_dir.as_ref().unwrap().as_ref())
        } else {
            let mut root_dir = tempfile::Builder::new();

            root_dir.prefix("grass-workspace-");
            root_dir.rand_bytes(5);
            let compilation_dir = if let Some(temp_dir) = &self.temp_dir {
                root_dir.tempdir_in(temp_dir.as_path())?
            } else {
                root_dir.tempdir()?
            };

            self.write_manifest_file(compilation_dir.path())?;
            self.write_source_code(compilation_dir.path())?;


            self.compilation_dir = Some(compilation_dir);
            Ok(self.compilation_dir.as_ref().unwrap().path())
        }
    }
    pub fn get_artifact(&mut self) -> Result<&Path> {
        if self.artifact_path.is_some() {
            return Ok(self.artifact_path.as_ref().unwrap());
        }
        log::info!("Building artifact {}", self.get_artifact_name()?);
        let mut child = match self.build_flavor {
            BuildFlavor::Debug => {
                self.cargo(&["build"], true)
            },
            BuildFlavor::Release => {
                self.cargo(&["build", "--release"], true)
            }
        }?;
        let status = child.wait()?;
        if status.success() {
            self.artifact_path = Some(self.get_artifact_path()?);
            return self.get_artifact();
        }

        Err(Error::new(std::io::ErrorKind::Other, format!("Cargo exited with error code {}", status.code().unwrap_or(0))))
    }

    pub fn execute_artifact(&mut self) -> Result<Child> {
        let artifact_path = self.get_artifact()?;
        log::info!("Launching artifact {}", artifact_path.as_os_str().to_string_lossy());
        Ok(
            Command::new(artifact_path)
                .current_dir(&self.working_dir)
                .envs(&self.env_vars)
                .args(&self.cmdline_args)
                .spawn()?
        )
    }
    pub fn print_expanded_code(&mut self) -> Result<()> {
        self.cargo(&["expand"], false)?.wait()?;
        Ok(())
    }
}

