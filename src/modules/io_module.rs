use std::io::{stdin, stdout, Write};
use crate::modules::module::Module;
use mlua::{Lua, Table, Value};
use crate::utils::time_util::current_time;

pub struct IoModule;

enum LogLevel {
    Info,
    Warn,
    Error,
    Debug
}

impl LogLevel {
    fn color(&self) -> &'static str {
        match self {
            LogLevel::Info => "\x1b[36m",
            LogLevel::Warn => "\x1b[33m",
            LogLevel::Error => "\x1b[91m",
            LogLevel::Debug => "\x1b[38;5;208m",
        }
    }
    fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Info => "[{TIME}] [INFO]",
            LogLevel::Warn => "[{TIME}] [WARN]",
            LogLevel::Error => "[{TIME}] [ERROR]",
            LogLevel::Debug => "[{TIME}] [DEBUG]",
        }
    }
}

fn log(value: Value, level: LogLevel) -> anyhow::Result<()> {
    let prefix = format!("{}{}\x1b[0m", level.color(), level.prefix()).replace("{TIME}", &current_time());
    println!("{} {}", prefix, value.to_string()?);
    Ok(())
}

fn read_line(prompt: Option<String>) -> anyhow::Result<String> {
    if let Some(p) = prompt {
        print!("{}", p);
        stdout().flush().ok();
    }

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .map_err(mlua::Error::external)?;

    Ok(input.trim_end().to_string())
}

impl Module for IoModule {
    fn name(&self) -> &'static str {
        "IO"
    }
    fn register(&self, lua: &Lua) -> anyhow::Result<Table> {
        let table = lua.create_table()?;

        // Output
        table.set(
            "info",
            lua.create_function(|_, value: Value| {
                log(value, LogLevel::Info).map_err(mlua::Error::external)?;
                Ok(())
            })?,
        )?;
        table.set(
            "warn",
            lua.create_function(|_, value: Value| {
                log(value, LogLevel::Warn).map_err(mlua::Error::external)?;
                Ok(())
            })?,
        )?;
        table.set(
            "error",
            lua.create_function(|_, value: Value| {
                log(value, LogLevel::Error).map_err(mlua::Error::external)?;
                Ok(())
            })?,
        )?;
        table.set(
            "debug",
            lua.create_function(|_, value: Value| {
                log(value, LogLevel::Debug).map_err(mlua::Error::external)?;
                Ok(())
            })?,
        )?;

        // Input
        table.set(
            "readLine",
            lua.create_function(|_, prompt: Option<String>| {
                let result = read_line(prompt).map_err(mlua::Error::external)?;
                Ok(result)
            })?
        )?;

        Ok(table)
    }
}