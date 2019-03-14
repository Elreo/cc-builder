use super::lua;

pub fn print(text: &str) -> lua::Script {
    lua::Script::new(format!("print(\"{}\")", text).as_str())
}