mod lua;

pub mod standard;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub use lua::{ScriptContainer, Script};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub struct LuaBuilder {
    containers: Vec<ScriptContainer>,
}

impl LuaBuilder {
    pub fn new() -> LuaBuilder {
        LuaBuilder {
            containers: Vec::new(),
        }
    }

    pub fn push_container(&mut self, container: ScriptContainer) {
        self.containers.push(container);
    }

    fn build_script(&self) -> String {
        let mut output = String::new();
        for container in &self.containers {
            output.push_str(format!("{}\n", container).as_str());
        }
        output
    }

    pub fn export_to_file(&self, path: &str) {
        let path = Path::new(path);
        let display = path.display();
        
        // Open file in write only mode.
        let mut file = File::create(&path)
            .unwrap_or_else(|err| panic!("Could not create {}: {}", display, err.description()));
        
        // Generate scripts
        let script = self.build_script();

        match file.write_all(script.as_bytes()) {
            Ok(_) => println!("Script successfully created!"),
            Err(err) => panic!("Couldn't write to {}: {}", display, err.description()),
        }
    }
}