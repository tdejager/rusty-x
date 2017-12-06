extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate x;
use x::{start_operation, Error, OpCode};

use std::io;
use std::process;

fn main() {
    let matches = App::new("Rusty X")
                          .version("0.1")
                          .author("Tim de Jager <tdejager89@gmail.com>")
                          .about("Rusty snippet manager")
                          .arg(Arg::with_name("tags")
                               .help("Keywords to search for")
                               .required(false)
                               .multiple(true))
                          .subcommand(SubCommand::with_name("add")
                               .about("Add a file")
                               .args_from_usage(
                               "<name> 'Name of the snippet'
                                [tags]... 'Tags to add'
                               "))
                         .get_matches();


    let res = if let Some(add) = matches.subcommand_matches("add") {
        // Parse for tags
        let tags: Vec<String> = match add.values_of("tags") {
            Some(ts) => ts.map(|s| s.to_string()).collect(),
            None => Vec::new()
        };
        println!("Tags : {:?}", tags);
       // Add a snippet
       start_operation(OpCode::AddSnippet, tags)
    } else {
        // Parse for tags
        let tags: Vec<String> = match matches.values_of("tags") {
            Some(ts) => ts.map(|s| s.to_string()).collect(),
            None => Vec::new()
        };
        // List a snippet
        start_operation(OpCode::ListSnippets, tags)
    };

    if let Err(err) = res {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
    
}
