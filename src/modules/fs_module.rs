use mlua::{Lua, Table};
use crate::modules::module::Module;

pub struct FsModule;

impl Module for FsModule {
    fn name(&self) -> &'static str {
        "FS"
    }
    fn register(&self, lua: &Lua) -> anyhow::Result<Table> {
        let table = lua.create_table()?;
        
        // Reading
        
        // Writing
        
        Ok(table)
    }
}