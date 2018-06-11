extern crate toml;
extern crate serde;
use std::env;
use std::path;
use std::fs::File;
use std::io::{Write, Read};
use error;

/// Location of the snippets
#[derive(Serialize, Deserialize)]
pub struct SnippetLocation {
    pub local: String,
    pub ext: String,
    pub git: Option<String>,
}

impl SnippetLocation {

    pub fn default(home: &String) -> SnippetLocation {
        return SnippetLocation{
            local: String::from(home.to_owned() + "/.snippets"),
            ext: "md".to_string(),
            git: None
        }
    }
}

/// Project folder structure
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub locations: Vec<SnippetLocation>
}

pub enum ProjectOperation {
    Exist(Project),
    NotExist(Project)
}

impl Project {

    /// Write a project
    pub fn write(&self, folder : &path::Path) -> Result<(), error::Error> {
        let to_write = toml::to_string(self).expect("Cannot serialize project");
        let full_path = path::Path::new(&folder).join(".rusty-x.toml");
        println!("Writing to {:?}", full_path);
        let mut f = File::create(&full_path).expect("Cannot create project file");
        f.write_all(to_write.as_bytes()).expect("Cannot write to project file");
        Ok(())
    }
    /// Get the default project location
    pub fn default_project() -> Result<ProjectOperation, error::Error> {
        let home = String::from(env::home_dir()
                                .expect("Cannot find the home dir")
                                .to_str().unwrap());

        let format = format!("{}/.rusty-x.toml", &home);
        let path = path::Path::new(&format);

        // If exists than deserialize toml
        if path.exists() {

            // Read the file
            let mut f = File::open(path).expect("Found project file but can't read it.");
            let mut buffer = String::new();
            f.read_to_string(&mut buffer)?;

            // Deserialize the toml
            let project: Project = toml::from_str(&buffer).expect("Cannot deserialize project file");
            return Ok(ProjectOperation::Exist(project));
        }

        Ok(ProjectOperation::NotExist(Project{ locations: vec![SnippetLocation::default(&home)]}))
    }
}
