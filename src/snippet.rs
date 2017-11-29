
use std::fs::File;
use std::io::{BufReader, BufRead};
use error;

/**
 * The snippet struct that has uses multiple tags, to order the snippets
 */
#[derive(Debug)]
pub struct Snippet {
    name: String,
    tags: Vec<String>
}

impl Snippet {

    pub fn new(name: String, tags: &Vec<String>) -> Snippet {
        Snippet{name, tags: tags.to_owned()}
    }

}

pub fn read_tags(path: &str) -> Result<Vec<String>, error::Error>
{
    let f = File::open(path)?;
    let mut file = BufReader::new(f);

    // Read the first line of the file
    let mut buffer = String::new();
    file.read_line(&mut buffer)?;

    // Read the tags
    let t : Vec<&str> = buffer.as_str().trim().split(',').collect();
    let tags : Vec<String>  = t.iter()
        .map(|s| String::from(s.to_owned())).collect();

    Ok(tags)
}






