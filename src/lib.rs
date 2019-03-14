mod lua;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use lua::{Container};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub struct LuaBuilder {
    containers: Vec<Box<dyn Container>>,
}

impl LuaBuilder {
    pub fn new() -> LuaBuilder {
        LuaBuilder {
            containers: Vec::new(),
        }
    }

    fn build_script(&self) -> String {
        let mut output = String::new();
        for container in &self.containers {
            output.push_str(format!("{}\n", container.to_lua()).as_str());
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