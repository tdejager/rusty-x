#[macro_use]
extern crate serde_derive;
extern crate docopt;

extern crate dirs;
extern crate rusty_x;
extern crate skim;
extern crate syntect;

extern crate ansi_term;

use ansi_term::Colour::Yellow;
use ansi_term::{ANSIString, ANSIStrings};

use std::default::Default;
use std::io::BufRead;
use std::io::Cursor;
use std::path;

use docopt::Docopt;

use skim::{Skim, SkimOptions};
use syntect::easy::HighlightFile;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

use rusty_x::Snippet;
use rusty_x::{edit_snippet, start_operation, Error, OpCode, Project, ProjectOperation};

const USAGE: &'static str = "\
<<<<<<< HEAD
Usage: x
       x [--add=<filename>] <keywords>...
=======
Usage: x [--add=<filename>] <keywords>...
       x --new
>>>>>>> 05622e3a57b8d0eb727bbcbadfacbce9172d624e
       x [--edit] <keywords>...
       x --pull
       x --save

Options:
    -h, --help           Show this message
    --new                Add a new snippet without a given name and you need to fill in the keywords
    --add=<filename>     Add a new snippet with given filename and keywords
    -e, --edit           Edit a existing snippet
    --all                Show all snippets
    --pull               Sync snippet repo (git pull)
    --save               Save snippet repo (git add, git commit, git push)
\
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_keywords: Vec<String>,
    flag_add: String,
    flag_new: bool,
    flag_edit: bool,
    flag_pull: bool,
    flag_save: bool,
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
            let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line, &ss);
            print!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        }

        line.clear();
    }

    // Clear the formatting
    println!("\x1b[0m");
}

/// Use skim to show multiple results, where selections is the files to select
fn show_multiple_results(selections: &Vec<String>) -> Vec<usize> {
    let options: SkimOptions = SkimOptions::default().ansi(true).height("50%").multi(true);

    let joined = selections
        .iter()
        .fold(String::new(), |acc, s| acc + s + "\n");

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(joined))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    selected_items.iter().map(|item| item.get_index()).collect()
}

fn main() -> Result<(), Error> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Try to get the project file
    let project_operation = Project::default_project()?;

    // Create a new project file if it does not exist
    let project = match project_operation {
        ProjectOperation::NotExist(project) => project,
        ProjectOperation::Exist(project) => project,
    };

    // TODO: Find a cleaner way, without writing all the time
    // Write anyway to be sure changes are merged
    let home = String::from(
        dirs::home_dir()
            .expect("Cannot find the home dir")
            .to_str()
            .unwrap(),
    );
    project.write(home.as_ref())?;

    // Check if the snippets folder exits and make it if it does not
    for location in &project.locations {
        location.create_if_not_exists()?;
    }

    // Get mode of operation
    let op_code = if !args.flag_add.is_empty() || args.flag_new {
        // Convert to strings
        let results = project.locations.iter().map(|l| { l.local.clone() }).collect();
        // Only use the fist choice
        let choice = show_multiple_results(&results);
        // Return if no choice has been made
        if choice.len() == 0 {
            return Ok(());
        }
        if !args.flag_add.is_empty() {
            (OpCode::AddSnippet(args.flag_add, &project.locations[choice[0]]))
        } else {
            (OpCode::NewSnippet(&project.locations[choice[0]]))
        }
    } else if args.flag_edit {
        (OpCode::ListSnippets(true))
    } else if args.flag_pull {
        (OpCode::PullSnippets)
    } else if args.flag_save {
        (OpCode::SaveSnippets)
    } else {
        (OpCode::ListSnippets(false))
    };


    // Pass keywords or options
    let keywords: Vec<String> = args.arg_keywords;

    // Start processing with given arguments
    start_operation(&op_code, &project, keywords)
        .and_then(|snippets| { process_snippets(op_code, &snippets) })?;

    check_modified_files(&project)?;

    Ok(())
}


/// Check if we have unsaved changes if so display
fn check_modified_files(project: &Project) -> Result<(), Error> {
    for location in project.locations.iter().filter(|l| l.git == Some(true)) {
        // If this is a git location
        match rusty_x::determine_git_modified_status(location) {
            Ok(rusty_x::GitStatus::Modified) => {
                let strings: &[ANSIString] = &[
                    Yellow.bold().paint(&location.local),
                    Yellow.paint(" has modified files")
                ];
                println!("{}", ANSIStrings(strings));
                Ok(())
            }
            // Don't need to show anything
            Ok(_) => { Ok(()) }
            // Return the error
            Err(e) => Err(e)
        }?
    };

    Ok(())
}

fn process_snippets(op_code: OpCode, snippets: &Vec<Snippet>) -> Result<(), Error> {
    let intermediate: Vec<String> = snippets
        .iter()
        .map(|s| {
            s.tags
                .iter()
                .fold(String::new(), |s, val| (s + ", " + &format!("{}", ansi_term::Style::new().bold().paint(val.trim())).to_owned()))
                .replacen(",", "", 1)
        })
        .collect();

    // We have more than 1 result
    if intermediate.len() > 1 {
        // Use library to do multiple selection for snippets
        let to_show = show_multiple_results(&intermediate);

        for i in to_show {
            let snip = &snippets[i];
            let full_path = path::Path::new(&snip.name);
            // If we chose to edit the snippet use the edit command
            // TODO add x editor command
            if let OpCode::ListSnippets(true) = op_code {
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
