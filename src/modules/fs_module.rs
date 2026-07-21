use mlua::{Lua, Table, Value};
use std::fs;
use crate::modules::module::Module;

pub struct FsModule;

fn read_file(path: &str) -> mlua::Result<String> {
    let content = fs::read_to_string(path)
        .map_err(mlua::Error::external)?;
    Ok(content)
}

fn exists(path: &str) -> mlua::Result<bool> {
    let exists = fs::exists(path)
        .map_err(mlua::Error::external)?;
    Ok(exists)
}

fn write_file(path: &str, content: &str) -> mlua::Result<()> {
    fs::write(path, content)
        .map_err(mlua::Error::external)?;
    Ok(())
}

fn delete_file(path: &str) -> mlua::Result<()> {
    fs::remove_file(path)
        .map_err(mlua::Error::external)?;
    Ok(())
}

fn create_dir(path: &str) -> mlua::Result<()> {
    fs::create_dir_all(path)
        .map_err(mlua::Error::external)?;
    Ok(())
}

impl Module for FsModule {
    fn name(&self) -> &'static str {
        "FS"
    }
    fn register(&self, lua: &Lua) -> anyhow::Result<Table> {
        let table = lua.create_table()?;

        // Reading
        table.set(
            "readFile",
            lua.create_function(|_, path: String| {
                read_file(&path)
            })?,
        )?;
        table.set(
            "exists",
            lua.create_function(|_, path: String| {
                exists(&path)
            })?,
        )?;

        // Writing
        table.set(
            "writeFile",
            lua.create_function(|_, (path, content): (String, String)| {
                write_file(&path, &content)
            })?,
        )?;
        table.set(
            "deleteFile",
            lua.create_function(|_, path: String| {
                delete_file(&path)
            })?,
        )?;
        table.set(
            "createDir",
            lua.create_function(|_, path: String| {
                create_dir(&path)
            })?,
        )?;

        // Scanners

        Ok(table)
    }
}