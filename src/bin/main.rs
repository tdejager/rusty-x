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

    // Pass keywords or options
    let keywords: Vec<String> = matches.values_of("KEYWORDS").unwrap().map(|s| s.to_string()).collect();
    let res = start_operation(op_code, keywords, filename);

    if let Err(err) = res {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
    
}
