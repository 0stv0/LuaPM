use std::collections::HashMap;
use std::path::PathBuf;
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
    pub root: PathBuf
}
impl Project {
    pub fn new(name: impl Into<String>, root: PathBuf) -> Self {
        Self {
            name: name.into(),
            version: "0.1.0".to_string(),
            entry: Some("main.lua".to_string()),
            scripts: HashMap::new(),
            root
        }
    }
    pub async fn load(root: &std::path::Path) -> anyhow::Result<Self> {
        let manifest = root.join("luapm.json");

        let raw = tokio::fs::read_to_string(&manifest)
            .await
            .map_err(|e| anyhow::anyhow!("failed to read {}: {}", manifest.display(), e))?;
        let mut project: Project = serde_json::from_str(&raw)
            .map_err(|e| anyhow::anyhow!("failed to parse {}: {}", manifest.display(), e))?;

        project.root = root.to_path_buf();
        Ok(project)
    }
    pub async fn load_default() -> anyhow::Result<Self> {
        let cwd = std::env::current_dir()?;
        Self::load(&cwd).await
    }
    pub async fn save(&self) -> anyhow::Result<()> {
        let manifest = self.root.join("luapm.json");
        let json     = serde_json::to_string_pretty(self)?;
        tokio::fs::write(&manifest, json).await?;

        Ok(())
    }
}