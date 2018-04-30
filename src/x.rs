use project;
use snippet;
use error::Error;
use error::Error::{InternalError};

use std::fs;
use std::path;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::Command;


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
pub fn find_snippets(project: &project::Project) -> Result<Vec<fs::DirEntry>, Error> {
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
pub fn load_snippets(dir_entries : &Vec<fs::DirEntry>, keywords: &Vec<String>) -> Result<Vec<snippet::Snippet>, Error>
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
pub fn start_operation(code: OpCode, keywords: Vec<String>, optional_filename: &str) -> Result<(), Error>{
    let project = project::Project::default_project();
    println!("Opcode {:?}", code);

    // Match on operation
    let result = match code {

        OpCode::AddSnippet => {
            // Create the full path
            let full_path = path::Path::new(&project.folder_name).join(optional_filename);
            // Create the file
            if full_path.exists() {
                return Err(InternalError("Snippet already exists".to_string()))
            }
            let mut file = File::create(&full_path)?;

            // Write the keywords to the file
            for keyword in keywords {
                file.write(keyword.as_bytes())?;
                file.write(b",")?;
            }
            file.write(b"\n==============\n")?;
            // Open vim on location
            let _output = Command::new("vim").
                arg(full_path).spawn()?.wait_with_output()?;
            Ok(())
        }

        // List snippets
        OpCode::ListSnippets => {
            println!("List snippets");
             let files = find_snippets(&project)?;
             let snippets = load_snippets(&files , &keywords)?;

             for snip in snippets {
                 println!("{:?}", snip.name);
                 let full_path = path::Path::new(&project.folder_name).join(snip.name);
                 let mut file = File::open(full_path)?;

                 let mut contents = String::new();
                 file.read_to_string(&mut contents)?;
                 println!("{:?}", contents);
             }
             Ok(())
        },

        // Sync snippets
        OpCode::SyncSnippets => {
            println!("Sync all snippets");
            Ok(())
        }
    };
    result
}

