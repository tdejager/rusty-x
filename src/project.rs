use std::env;
// Project folder structure
pub struct Project {
    pub folder_name: String,
    pub ext: String
}

impl Project {


    /// Get the default project location
    pub fn default_project() -> Project {
        let home = String::from(env::home_dir()
                                .expect("Cannot find the home dir")
                                .to_str().unwrap());

        Project{folder_name: String::from(home + "/.snippets"), ext: String::from("md")}
    }
}
