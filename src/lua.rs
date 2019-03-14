use std::fmt;

/// A trait that needs to be implemented, in order
/// to be fed into LuaBuilder's build script.
pub trait ToLua {
    /// Turns a variable into Lua consumable.
    fn to_lua(&self) -> String;
}

/// A trait that needs to be implemented by any script
/// container. It contains implementation to merge, verify
/// and dump scripts.
pub trait Container: ToLua {
    fn merge(&mut self, other: Box<dyn Container>);
    fn verify(&self, other: Box<dyn Container>);
    fn dump(&mut self) -> String;

    fn push_query(&mut self, query: ScriptQuery);
    fn pop_query(&mut self) -> ScriptQuery;
    fn dump_query(&mut self) -> Vec<ScriptQuery>;
}

/// This enum represents the Lua literals.
pub enum LuaType {
    Number(isize),
    Str(String),
    Boolean(bool),
    Nil,
    UseVar(String),
}

impl ToLua for LuaType {
    /// This function will return a lua consumable representation
    /// of this data type.
    fn to_lua(&self) -> String {
        match self {
            LuaType::Number(n) => n.to_string(),
            LuaType::Str(s) => format!("\"{}\"", s),
            LuaType::Boolean(b) => b.to_string(),
            LuaType::Nil => String::from("nil"),
            LuaType::UseVar(uv) => uv.clone(),
        }
    }
}

/// In case we want to directly parse LuaType into format macro,
/// or something similar.
impl fmt::Display for LuaType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.to_lua().as_str())?;
        Ok(())
    }
}

/// Allows conversion from vector that contains lua type to lua
/// representation.
impl ToLua for Vec<LuaType> {
    fn to_lua(&self) -> String {
        self.iter()
            .map(|lt| lt.to_lua())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

/// The script query. This is used by containers to
/// track scripts that was being added.
pub enum ScriptQuery {
    CreateVariable { is_local: bool, name: String, hardcoded: Option<LuaType> },
    CallFunction { name: String, arguments: Vec<LuaType> },
}

impl ToLua for ScriptQuery {
    /// Convert the queries into lua consumable script stripes.
    fn to_lua(&self) -> String {
        match self {
            ScriptQuery::CreateVariable { is_local, name, hardcoded } => {
                let mut output = String::new();
                if *is_local {
                    output.push_str("local ");
                }
                output.push_str(name.as_str());
                if let Some(hc) = hardcoded {
                    output.push_str(hc.to_lua().as_str());
                }
                output
            },
            ScriptQuery::CallFunction { name, arguments } => {
                format!("{}({})", name, arguments.to_lua())
            },
        }
    }
}

/// The script's base container. This is where all the scripts are
/// kept.
pub struct ScriptContainer {
    query: Vec<ScriptQuery>,
}

impl ToLua for ScriptContainer {
    fn to_lua(&self) -> String {
        let mut output = String::new();
        for q in self.query.iter() {
            output.push_str(format!("{}\n", q.to_lua()).as_str());
        }
        output
    }
}

// TODO: Implement this one.
impl Container for ScriptContainer {
    fn merge(&mut self, other: Box<dyn Container>) {
        panic!("Not yet implemented")
    }

    fn verify(&self, other: Box<dyn Container>) {
        panic!("Not yet implemented")
    }

    fn dump(&mut self) -> String {
        panic!("Not yet implemented")
    }

    fn push_query(&mut self, query: ScriptQuery) {
        panic!("Not yet implemented")
    }

    fn pop_query(&mut self) -> ScriptQuery {
        panic!("Not yet implemented")
    }

    fn dump_query(&mut self) -> Vec<ScriptQuery> {
        panic!("Not yet implemented")
    }
}

impl ScriptContainer {
    /// Verify the query, to check if its valid or not. It will return true
    /// if one or more than one argument variable exist in the container.
    /// Note that, as of now, it only takes `CallFunction` for query.
    fn verify_query(&self, query: &ScriptQuery) -> bool {
        for q in self.query.iter() {
            match q {
                ScriptQuery::CreateVariable { is_local, name, hardcoded } => {
                    if let ScriptQuery::CallFunction { arguments, .. } = query {
                        // return arguments.iter()
                        //     .any(|b| if let LuaType::UseVar(uv) = b {
                        //         uv == name
                        //     } else { false });
                        return arguments.iter()
                            .any(|b| match b {
                                LuaType::UseVar(uv) => uv == name,
                                _ => false,
                            });
                    } else {
                        panic!("ERROR: Container attempted to verify a non-assignable data");
                    }
                },
                _ => continue,
            }
        }
        false
    }
}



// We need to implement the script first. The script struct
// contains all the data that's needed to build a snippet.
// A script container is used to verify and check the script's
// integrity.

// Note: Use logging system to verify them.