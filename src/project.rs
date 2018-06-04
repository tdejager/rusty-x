#[macro_use]
extern crate serde_derive;
extern crate toml;
use std::env;
use std::path;
use std::fs::File;
use error;

/// Location of the snippets
#[derive(Deserialize)]
struct SnippetLocation {
    local: String,
    ext: String,
    git: Option<String>,
}

impl SnippetLocation {

    pub fn default(home: &String) -> SnippetLocation {
        return SnippetLocation{
            local: String::from(home + "/.snippets"),
            ext: "md".to_string(),
            git: None
        }
    }
}

/// Project folder structure
pub struct Project {
    locations: Vec<SnippetLocation>
}

pub enum ProjectOperation {
    Exist(Project),
    NotExist(Project)
}

impl Project {

    /// Write a project
    pub fn write(self, folder : &path::Path) -> Result<(), error::Error> {
        let to_write = toml::to_string(self)?;
        let full_path = path::Path::new(to_write).join(".rusty-x.toml");
        let mut f = File::create(&full_path)?;
        f.write_all(to_write.as_bytes())?;
        Ok(())
    }
    /// Get the default project location
    pub fn default_project() -> Result<ProjectOperation, error::Error> {
        let home = String::from(env::home_dir()
                                .expect("Cannot find the home dir")
                                .to_str().unwrap());

        let path = path::Path::new(format!("{}/.rusty-x.toml", home));

        // If exists than deserialize toml
        if path.exists() {

            // Read the file
            let mut f = File::open(path)?;
            let mut buffer = String::new();
            f.read_to_string(&mut buffer);

            // Deserialize the toml
            let project: Project = toml::from_str(&buffer);
            Ok(ProjectOperation::Exist(project))
        }

        Ok(ProjectOperation::NotExist(Project{ locations: vec![SnippetLocation::default(&home)]}))
    }
}
