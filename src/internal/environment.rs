use std::collections::HashMap;
use mlua::Lua;
use std::path::{Path, PathBuf};
use anyhow::Context;
use crate::internal::preload::preload_code;
use crate::modules::module::register_all;

pub struct Environment {
    lua: Lua
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
        for (k, v) in contents {
            // TODO: FIX NESTING IMPORTS
            let mut lines: Vec<String> = v.lines().map(|l| l.to_string()).collect();
            let mut code = String::new();
            for i in 0..lines.len() {
                let line = &lines[i];
                if !line.starts_with("@import") {
                    continue;
                }
                let mut parts: Vec<&str> = line.split(" ").collect();
                if parts.len() < 2 {
                    continue;
                }
                let mut requested = parts[1].to_string();
                if requested.is_empty() {
                    continue;
                }
                if !requested.contains(".lua") {
                    requested = format!("{}.lua", requested).replace(";", "");
                }
                let req_path = root.join(&requested.replace(";", ""));
                lines[i] = format!("-- {}", line);

                let source = tokio::fs::read_to_string(&req_path)
                    .await
                    .with_context(|| format!("failed to read {}", req_path.display()))?;
                code = format!("{}\n\n\n{}", code, source);
            }

            code = format!("{}\n\n\n{}", preload_code(&code), preload_code(&lines.join("\n")));
            fixed.insert(k.to_owned(), code);
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