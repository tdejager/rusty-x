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
    let entries = fs::read_dir(&project.folder_name)?;
    for e in entries {
        let dir_ent = e?;
        let path = dir_ent.path();
        let ext_opt = path.extension();

        if let Some(ext) = ext_opt {
            if let Some(s) = ext.to_str(){
                if s == project.ext {
                    res.push(dir_ent);
                }
            }
        }
    }
    Ok(res)
}

/*pub fn load_snippets(dir_entries : Result<Vec<fs::DirEntry>, Error>)*/
//{
    //for entry in dir_entries
    //{
        //let unwrapped_entry = try!(entry);
    //}
/*}*/

//// Start the different operation modes
pub fn start_operation(code: OpCode, options: Vec<String>) -> Result<(), error::Error>{
    let project = project::Project::default_project();
    // Match on operation
    let result = match code {
        OpCode::AddSnippet => { 
            println!("Add a snippet");
            Ok(())
        }

        OpCode::ListSnippets => {
             let snippets = find_snippets(&project)?;
             for snip in snippets {
                 //let path : path::PathBuf =snip.iter().map(|dir_ent| dir_ent.path()).collect();
                 let path : path::PathBuf = snip.path();
                 println!("{:?}", path.to_str());
             }
             Ok(())
        },
        OpCode::SyncSnippets => {
            println!("Sync all snippets");
            Ok(())
        }
    };

    result
}

