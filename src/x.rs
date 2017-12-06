use project;
use snippet;
use error;

use std::fs;
use std::path;


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
pub fn find_snippets(project: &project::Project) -> Result<Vec<fs::DirEntry>, error::Error> {
    println!("Finding snippets in {},", &project.folder_name.as_str());

    // Crawl through directory that is set as project root
    let mut res : Vec<fs::DirEntry> = Vec::new();

    // Read the entries in the folder
    let entries = fs::read_dir(&project.folder_name)?;
    // For each of the entries
    for e in entries {
        let dir_ent = e?;

        // Get the path
        let path = dir_ent.path();
        // Get the extension
        let ext_opt = path.extension();
        if let Some(ext) = ext_opt {
            if let Some(s) = ext.to_str(){
                // Add to list if files match extension
                if s == project.ext {
                    res.push(dir_ent);
                }
            }
        }
    }
    Ok(res)
}

/// Load snippets from the dir entries
pub fn load_snippets(dir_entries : &Vec<fs::DirEntry>, keywords: &Vec<String>) -> Result<Vec<snippet::Snippet>, error::Error>
{
    let mut result : Vec<snippet::Snippet> = Vec::new();
    let keyword_slice = keywords.as_slice();

    // Return snippets
    for entry in dir_entries {
        // Read the file name
        let filename = entry.file_name();
        // Read the tags
        let tags = snippet::read_tags(entry.path().to_str().unwrap())?;
        
        // If tag is in the snippet, or no tags are given
        if keyword_slice.len() == 0 || tags.iter().fold(false, | res, tag| (res || keyword_slice.contains(&tag))) {
            result.push(snippet::Snippet::new(filename.to_str().unwrap().into(), &tags));
        }
    }
    Ok(result)
}

//// Start the different operation modes
pub fn start_operation(code: OpCode, keywords: Vec<String>) -> Result<(), error::Error>{
    let project = project::Project::default_project();
    // Match on operation
    match code {
        
        OpCode::AddSnippet => { 
            println!("Add a snippet");
            Ok(())
        }

        OpCode::ListSnippets => {
             let files = find_snippets(&project)?;
             let snippets = load_snippets(&files , &keywords)?;

             println!("{}", snippets.len());
             for snip in snippets {
                 //let path : path::PathBuf = snip.iter().map(|dir_ent| dir_ent.path()).collect();
                 println!("{:?}", snip);
             }
             Ok(())
        },

        OpCode::SyncSnippets => {
            println!("Sync all snippets");
            Ok(())
        }
    }
}

