#[macro_use]
extern crate serde_derive;

mod x;
pub use x::{edit_snippet, start_operation, OpCode};

mod project;
pub use project::{Project, ProjectOperation};

mod snippet;
pub use snippet::Snippet;

mod error;
pub use error::Error;

mod git;
pub use git::*;
