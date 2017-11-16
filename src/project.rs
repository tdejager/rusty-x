use std::env;
// Project folder structure
pub struct Project {
    pub folder_name: String,
    pub ext: String
}

impl Project {
    

    /// Get the default project location
    pub fn default_project() -> Project{
        if let Some(home) = env::home_dir() {
            Project{folder_name: String::from(home.join(".snippets").to_str().unwrap()), 
                ext: String::from(".md")}
        }
        else {
            panic!("Error cannot read user home directory")
        }
    }
}
