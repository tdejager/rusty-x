extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate x;
use x::{start_operation, Error, OpCode};

fn main() {
    let matches = App::new("Rusty X")
                          .version("0.1")
                          .author("Tim de Jager <tdejager89@gmail.com>")
                          .about("Rusty snippet manager")
                          .arg(Arg::with_name("add")
                               .short("a")
                               .long("add")
                               .help("Opens the editor to add a file"))
                          .arg(Arg::with_name("KEYWORDS")
                               .help("Keywords to search for")
                               .required(true)
                               .multiple(true)).get_matches();

    // Pass keywords or options
    let keywords: Vec<String> = matches.values_of("KEYWORDS").unwrap().map(|s| s.to_string()).collect();
    let res = start_operation(OpCode::ListSnippets, keywords);
    if let Err(err) = res {
        println!("{:?}", err);
    }
    
}
