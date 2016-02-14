use std::process::Command;
use yaml_rust::Yaml;

pub struct Project {
    pub name: String
}

impl Project {
    pub fn new(config: Yaml) -> Project {
        Project {
            name: "Mike".to_owned()
        }
    }

    pub fn launch(self) {
        let child = Command::new("tmux").spawn().unwrap();
        let output = child.wait_with_output().unwrap();

        println!("{:?}", output.status);
        println!("{:?}", String::from_utf8(output.stderr));
        println!("{:?}", output.stdout);
    }
}
