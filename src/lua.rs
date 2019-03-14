use std::fmt;

pub struct Script(String);

impl Script {
    pub fn new(scr: &str) -> Script {
        Script(String::from(scr))
    }

    pub fn get(&self) -> String {
        self.0.clone()
    }
}

pub struct ScriptContainer {
    scripts: Vec<Script>,
}

impl ScriptContainer {
    pub fn new() -> ScriptContainer {
        ScriptContainer {
            scripts: Vec::new(),
        }
    }

    pub fn push_script(&mut self, scr: Script) {
        self.scripts.push(scr);
    }

    pub fn pop_script(&mut self) -> Script {
        self.scripts.pop()
            .unwrap_or_else(|| panic!("Error attempting to pop a script out of an empty container."))
    }

    pub fn clear(&mut self) {
        self.scripts.clear();
    }
}

impl fmt::Display for ScriptContainer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.scripts.iter()
            .map(|s| s.get())
            .collect::<Vec<String>>()
            .join("\n").as_str())?;
        Ok(())
    }
}