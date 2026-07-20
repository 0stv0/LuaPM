use mlua::{Lua, Table};
use anyhow::Result;
use crate::modules::io_module;

pub trait Module {
    fn name(&self) -> &'static str;
    fn register(&self, lua: &Lua) -> Result<Table>;
}

pub fn register_all(lua: &Lua) -> anyhow::Result<()> {
    let modules: Vec<Box<dyn Module>> = vec![
        Box::new(io_module::IoModule)
    ];
    let globals = lua.globals();
    for module in modules {
        let table = module.register(lua)?;
        globals.set(module.name(), table)?;
    }
    Ok(())
}