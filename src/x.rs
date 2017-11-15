use project;
use std::fs;

#[derive(Debug)]
pub enum OpCode {
    // For the add snippet command
    AddSnippet,
    // For listing snippets
    ListSnippets,
    // For syncing snippets with the server
    SyncSnippets
}

/// Find the snippets associated with the project
pub fn find_snippets(project: &project::Project) {
    println!("Finding snippets in {},", &project.folder_name.as_str());

    // Crawl through directory that is set as project root
    match fs::read_dir(&project.folder_name) {
        Ok(files) => for path in files {
            println!("Name: {}", path.unwrap().path().display());
        },
        Err(why) => println!("{:?}", why)
    }

}
//// Start the different operation modes
pub fn start_operation(code: OpCode, options: Vec<String>){
    let project = project::Project::default_project();
    // Match on operation
    match code {
        OpCode::AddSnippet => println!("Add a snippet"),
        OpCode::ListSnippets => find_snippets(&project),
        OpCode::SyncSnippets => println!("Sync all snippets")
    }
}

