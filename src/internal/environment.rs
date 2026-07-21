use std::collections::HashMap;
use mlua::Lua;
use std::path::{Path, PathBuf};
use anyhow::Context;
use crate::internal::preload::preload_code;
use crate::modules::module::register_all;

pub struct Environment {
    lua: Lua
}

fn resolve_imports(path: &PathBuf, root: &PathBuf, contents: &HashMap<PathBuf, String>) -> anyhow::Result<String> {
    let raw = contents
        .get(path)
        .ok_or_else(|| anyhow::anyhow!("file {} not found in the project.", path.display()))?;

    let mut lines: Vec<String> = raw.lines().map(|l| l.to_string()).collect();
    let mut imported_code      = String::new();
    for i in 0..lines.len() {
        let line = lines[i].clone();

        if !line.starts_with("@import") {
            continue;
        }
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() < 2 {
            continue;
        }

        let mut req_name = parts[1].replace(";", "");
        if req_name.is_empty() {
            continue;
        }

        if !req_name.ends_with(".lua") {
            req_name = format!("{}.lua", req_name);
        }

        let req_path = root.join(PathBuf::from(req_name));
        lines[i]     = format!("-- {}", line);

        let nested    = resolve_imports(&req_path, root, contents)?;
        imported_code = format!("{}\n\n{}", imported_code, nested);
    }

    let combined = format!("{}\n\n{}", imported_code, lines.join("\n"));
    Ok(combined)
}

impl Environment {
    pub fn new() -> anyhow::Result<Self> {
        let lua = Lua::new();
        register_all(&lua)?;
        Ok(Self { lua })
    }
    pub async fn run_file(&self, path: &Path) -> anyhow::Result<()> {
        let source = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("failed to read {}", path.display()))?;
        let fixed = preload_code(&source);
        self.lua
            .load(&fixed)
            .exec_async()
            .await
            .with_context(|| format!("error executing {}", path.display()))?;

        Ok(())
    }
    pub async fn run_many(&self, root: &PathBuf, entry: &Option<PathBuf>, contents: &HashMap<PathBuf, String>) -> anyhow::Result<()> {
        let mut fixed: HashMap<PathBuf, String> = HashMap::new();
        for path in contents.keys() {
            let resolved  = resolve_imports(path, root, contents)?;
            let processed = preload_code(&resolved);
            fixed.insert(path.clone(), processed);
        }

        if let Some(entry_path) = entry {
            if let Some(source) = fixed.get(entry_path) {
                self.exec(source, &entry_path.display().to_string()).await?;
            }
        }
        for (path, source) in &fixed {
            if Some(path) == entry.as_ref() {
                continue;
            }
            self.exec(source, &path.display().to_string()).await?;
        }

        Ok(())
    }
    async fn exec(&self, source: &str, label: &str) -> anyhow::Result<()> {
        self.lua
            .load(source)
            .exec_async()
            .await
            .with_context(|| format!("error executing {}", label))?;
        Ok(())
    }
}