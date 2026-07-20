use mlua::Lua;
use std::path::Path;
use anyhow::Context;
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
        self.lua
            .load(&source)
            .exec_async()
            .await
            .with_context(|| format!("error executing {}", path.display()))?;

        Ok(())
    }
}