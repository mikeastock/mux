use std::process::Command;
use yaml_rust::Yaml;

pub struct Project {
    pub name: String
}

impl Project {
    pub fn new(config: Yaml) -> Project {
        Project {
            name: config["name"].as_str().unwrap().to_owned()
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

#[cfg(test)]
mod tests {
    use super::Project;
    use yaml_rust::YamlLoader;

    #[test]
    fn test_new() {
        let yaml =
"
name: rails
";

        let config = YamlLoader::load_from_str(yaml).unwrap()[0].to_owned();
        let project = Project::new(config);
        assert_eq!("rails", project.name);
    }
}
