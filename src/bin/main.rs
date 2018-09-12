#[macro_use]
extern crate serde_derive;
extern crate docopt;

extern crate rusty_x;
extern crate syntect;
extern crate skim;
extern crate dirs;


use std::path;
use std::io::BufRead;
use std::default::Default;
use std::io::Cursor;

use docopt::Docopt;

use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightFile;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use skim::{Skim, SkimOptions};

use rusty_x::{start_operation, edit_snippet, Error, OpCode, Project, ProjectOperation};
use rusty_x::Snippet;


const USAGE: &'static str = "\
Usage: x [--add=<filename>] <keywords>...
       x [--edit] <keywords>...

Options:
    -h, --help           Show this message
    --add=<filename>     Add a new snippet with given filename and keywords
    -e, --edit           Edit a existing snippet
\
";

#[derive(Debug, Deserialize)]
struct Args
{
    arg_keywords: Vec<String>,
    flag_add: String,
    flag_edit: bool,
}


/// Display the snippet on the command line
fn display_snippet(full_path: &path::Path) {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = ts.themes.get("base16-eighties.dark");

    let mut highlighter = HighlightFile::new(full_path, &ss, theme.unwrap()).unwrap();
    let mut line = String::new();
    while highlighter.reader.read_line(&mut line).unwrap() > 0 {
        {
            let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line);
            print!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        }

        line.clear();
    }

    // Clear the formatting
    println!("\x1b[0m");
}

/// Use skim to show multiple results, where selections is the files to select
fn show_multiple_results(selections: &Vec<String>) -> Vec<usize> {
    let options: SkimOptions = SkimOptions::default().height("50%").multi(true);

    let joined = selections.iter().fold(String::new(), |acc, s| acc + s + "\n");



    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(joined))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    selected_items.iter().map(|item| item.get_index()).collect()
}


fn main() -> Result<(), Error> {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    let (op_code, filename) = if !args.flag_add.is_empty() {
        (OpCode::AddSnippet, args.flag_add)
    } else if args.flag_edit {
        (OpCode::ListSnippets(true), String::new())
    } else {
        (OpCode::ListSnippets(false), String::new())
    };

    // Try to get the project file
    let project_operation = Project::default_project()?;

    // Create a new project file if it does not exist
    let project = match project_operation {
        ProjectOperation::NotExist(project) =>
            {
                let home = String::from(dirs::home_dir()
                    .expect("Cannot find the home dir")
                    .to_str().unwrap());
                project.write(home.as_ref())?;
                project
            }
        ProjectOperation::Exist(project) => { project }
    };

    // Check if the snippets folder exits and make it if it does not
    for location in &project.locations {
        location.create_if_not_exists()?;
    }

    // Pass keywords or options
    let keywords: Vec<String> = args.arg_keywords;
    let res = start_operation(&op_code, &project, keywords, &filename);

    match res {
        Err(err) =>
            {
                // Return error in case of an error
                Err(err)
            }
        Ok(snippets) =>
            {
                process_snippets(op_code, &snippets)
            }
    }
}

fn process_snippets(op_code: OpCode, snippets: &Vec<Snippet>) -> Result<(), Error> {

    let intermediate: Vec<String> = snippets.iter().map(|s| s.tags.iter().fold(String::new(), |s, val| { (s + "|" + val).to_owned() })).collect();

    // We have more than 1 result
    if intermediate.len() > 1 {
        // Use library to do multiple selection for snippets
        let to_show = show_multiple_results(&intermediate);

        for i in to_show {
            let snip = &snippets[i];
            let full_path = path::Path::new(&snip.name);
            // If we chose to edit the snippet use the edit command
            if let OpCode::ListSnippets(true) = op_code
                {
                    edit_snippet("vim", full_path)?;
                } else {
                // Otherwise display
                display_snippet(&full_path);
            }
        }
    } else if intermediate.len() == 1 {
        // Display a single snippet
        let snip = &snippets[0];
        let full_path = path::Path::new(&snip.name);

        // Same as above
        if let OpCode::ListSnippets(true) = op_code {
            edit_snippet("vim", full_path)?;
        }
        // Display otherwise
        display_snippet(&full_path);
    }
    Ok(())
}
