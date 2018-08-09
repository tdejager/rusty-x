#[macro_use]
extern crate serde_derive;

mod x;
pub use x::{OpCode, start_operation, edit_snippet};

mod project;
pub use project::{Project, ProjectOperation};

mod snippet;
pub use snippet::{Snippet};

mod error;
pub use error::{Error};

