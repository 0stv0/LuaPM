use mlua::{Lua, Table, Value};
use crate::modules::module::Module;
use serde_json::Value as JsonValue;

pub struct JSONModule;

fn lua_to_json(value: &Value) -> JsonValue {
    match value {
        Value::Nil => JsonValue::Null,
        Value::Boolean(b) => JsonValue::Bool(*b),
        Value::Integer(i) => JsonValue::Number((*i).into()),
        Value::Number(n) => serde_json::Number::from_f64(*n)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        Value::String(s) => JsonValue::String(s.to_string_lossy().to_string()),
        Value::Table(t) => {
            let mut is_array  = true;
            let mut max_index = 0;

            for pair in t.clone().pairs::<Value, Value>() {
                if let Ok((key, _)) = pair {
                    match key {
                        Value::Integer(i) if i > 0 => {
                            max_index = max_index.max(i as usize);
                        }
                        _ => {
                            is_array = false;
                            break;
                        }
                    }
                }
            }

            if is_array {
                let mut arr = Vec::new();
                for i in 1..=max_index {
                    let v: Value = t.get(i as i64).unwrap_or(Value::Nil);
                    arr.push(lua_to_json(&v));
                }
                JsonValue::Array(arr)
            } else {
                let mut map = serde_json::Map::new();
                for pair in t.clone().pairs::<String, Value>() {
                    if let Ok((key, val)) = pair {
                        map.insert(key, lua_to_json(&val));
                    }
                }
                JsonValue::Object(map)
            }
        },
        _ => JsonValue::Null
    }
}

fn json_to_lua(lua: &Lua, value: &JsonValue) -> mlua::Result<Value> {
    match value {
        JsonValue::Null => Ok(Value::Nil),
        JsonValue::Bool(b) => Ok(Value::Boolean(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else {
                Ok(Value::Number(n.as_f64().unwrap_or(0.0)))
            }
        },
        JsonValue::String(s) => Ok(Value::String(lua.create_string(s)?)),
        JsonValue::Array(arr) => {
            let table = lua.create_table()?;
            for (i, item) in arr.iter().enumerate() {
                let lua_val = json_to_lua(lua, item)?;
                table.set(i + 1, lua_val)?;
            }
            Ok(Value::Table(table))
        },
        JsonValue::Object(map) => {
            let table = lua.create_table()?;
            for (key, val) in map {
                let lua_val = json_to_lua(lua, val)?;
                table.set(key.clone(), lua_val)?;
            }
            Ok(Value::Table(table))
        },
        _ => Ok(Value::Nil)
    }
}

impl Module for JSONModule {
    fn name(&self) -> &'static str {
        "JSON"
    }
    fn register(&self, lua: &Lua) -> anyhow::Result<Table> {
        let table = lua.create_table()?;

        table.set(
            "encode",
            lua.create_function(|_, value: Value| {
                let json_value = lua_to_json(&value);
                serde_json::to_string(&json_value).map_err(mlua::Error::external)
            })?,
        )?;
        table.set(
            "decode",
            lua.create_function(|lua, text: String| {
                let json_value: JsonValue = serde_json::from_str(&text)
                    .map_err(mlua::Error::external)?;
                json_to_lua(lua, &json_value)
            })?,
        )?;

        Ok(table)
    }
}