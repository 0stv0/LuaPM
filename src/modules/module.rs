use mlua::{Lua, Table};
use anyhow::Result;
use crate::modules::fs_module::FsModule;
use crate::modules::io_module::IoModule;
use crate::modules::json_module::JSONModule;

pub trait Module {
    fn name(&self) -> &'static str;
    fn register(&self, lua: &Lua) -> Result<Table>;
}

pub fn register_all(lua: &Lua) -> anyhow::Result<()> {
    let modules: Vec<Box<dyn Module>> = vec![
        Box::new(IoModule),
        Box::new(FsModule),
        Box::new(JSONModule)
    ];
    let globals = lua.globals();
    for module in modules {
        let table = module.register(lua)?;
        globals.set(module.name(), table)?;
    }
    Ok(())
}