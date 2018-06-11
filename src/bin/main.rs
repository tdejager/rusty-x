extern crate clap;

use clap::{Arg, App, SubCommand};

extern crate x;

use x::{start_operation, Error, OpCode, Project, ProjectOperation};

use std::path;
use std::io::BufRead;
use std::env;

extern crate syntect;

use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightFile;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;

extern crate skim;

use skim::{Skim, SkimOptions};
use std::default::Default;
use std::io::Cursor;

fn display_snippet(full_path: &path::Path) {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = ts.themes.get("base16-ocean.dark");

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

fn show_multiple_results(selections: &Vec<String>) -> Vec<usize> {

    let options: SkimOptions = SkimOptions::default().height("50%").multi(true);

    let joined = selections.iter().fold(String::new(), |acc, s| acc + s + "\n");

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(joined))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    selected_items.iter().map(|item| item.get_index()).collect()
}

fn main() -> Result<(), Error> {
    let matches = App::new("Rusty X")
        .version("0.1")
        .author("Tim de Jager <tdejager89@gmail.com>")
        .about("Rusty snippet manager")
        .arg(Arg::with_name("KEYWORDS")
            .help("Keywords to search for")
            .required(true)
            .multiple(true))
        .subcommand(SubCommand::with_name("add")
            .about("Add a snippet")
            .arg(Arg::with_name("filename")
                .help("Snippet file name")
                .required(true)))
        .get_matches();

    // Should add
    let add = matches.subcommand_matches("add");

    let (op_code, filename) = if let Some(matches) = add {
        (OpCode::AddSnippet, matches.value_of("filename").unwrap())
    } else {
        (OpCode::ListSnippets, "")
    };

    // Try to get the project file
    let project_operation = Project::default_project()?;

    // Create a new project file if it does not exist
    let project = match project_operation {
        ProjectOperation::NotExist(project) =>
            {
                let home = String::from(env::home_dir()
                    .expect("Cannot find the home dir")
                    .to_str().unwrap());
                project.write(home.as_ref())?;
                project
            }
        ProjectOperation::Exist(project) => { project }
    };

    // Pass keywords or options
    let keywords: Vec<String> = matches.values_of("KEYWORDS").unwrap().map(|s| s.to_string()).collect();
    let res = start_operation(op_code, &project, keywords, filename);

    match res {
        Err(err) =>
            {
                Err(err)
            }
        Ok(snippets) =>
            {
                // Retrieve names to show in multiple selection
                let intermediate : Vec<String> = snippets.iter().map(|s| s.name.to_owned()).collect();
                if intermediate.len() > 1 {
                    // Use library to do multiple selection for snippets
                    let to_show = show_multiple_results(&intermediate);

                    for i in to_show {
                        let snip = &snippets[i];
                        let full_path = path::Path::new(&snip.name);
                        display_snippet(&full_path);
                    }
                } else if intermediate.len() == 1 {
                    let snip = &snippets[0];
                    let full_path = path::Path::new(&snip.name);
                    display_snippet(&full_path);
                }
                Ok(())
            }
    }
}
