
macro_rules! Tester {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    };
}

mod database;
use database::{AssetHolder};


use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};
//testing
fn main()->rlua::Result<()>{
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        globals.set("string_var", "hello")?;
        globals.set("int_var", 42)?;

        Ok(())
    })?;
    Ok(())
}

