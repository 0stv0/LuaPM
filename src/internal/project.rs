use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,

    #[serde(default)]
    pub entry: Option<String>,

    #[serde(default)]
    pub scripts: HashMap<String, String>,

    #[serde(skip)]
    pub root: PathBuf,

    #[serde(skip)]
    pub contents: HashMap<PathBuf, String>
}

pub fn project_created(root: &PathBuf) -> anyhow::Result<bool> {
    Ok(root.join("luapm.json").exists())
}
fn scan_dir(dir: &Path) -> anyhow::Result<HashMap<PathBuf, String>> {
    let mut contents = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path  = entry.path();

        if path.is_dir() {
            let nested = scan_dir(&path)?;
            contents.extend(nested);
        } else if path.extension().map_or(false, |ext| ext == "lua") {
            let content = fs::read_to_string(&path)?;
            contents.insert(path, content);
        }
    }

    Ok(contents)
}

impl Project {
    pub async fn new(name: impl Into<String>, root: PathBuf) -> anyhow::Result<Self> {
        let inst = Self {
            name: name.into(),
            version: "0.1.0".to_string(),
            entry: Some("main.lua".to_string()),
            scripts: HashMap::new(),
            contents: scan_dir(&root)?,
            root
        };
        let manifest = &inst.root.join("luapm.json");
        let json     = serde_json::to_string_pretty(&inst)?;
        tokio::fs::write(&manifest, json).await?;
        Ok(inst)
    }
    pub async fn load(root: &Path) -> anyhow::Result<Self> {
        let manifest = root.join("luapm.json");
        let raw = tokio::fs::read_to_string(&manifest)
            .await
            .map_err(|e| anyhow::anyhow!("failed to read {}: {}", manifest.display(), e))?;
        let mut project: Project = serde_json::from_str(&raw)
            .map_err(|e| anyhow::anyhow!("failed to parse {}: {}", manifest.display(), e))?;

        project.root     = root.to_path_buf();
        project.contents = scan_dir(root)?;
        Ok(project)
    }
}