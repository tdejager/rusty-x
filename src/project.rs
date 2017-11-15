
// Project folder structure
pub struct Project {
    pub folder_name: String,
    pub ext: String
}

impl Project {
    

    /// Get the default project location
    pub fn default_project() -> Project{
        Project{folder_name: String::from("~/.snippets"), ext: String::from(".md")}
    }
}
