use error;
use project::{Project, SnippetLocation};
use std::process::{Command, Stdio};

/// Determine the git support for a project, modifies the git status
pub fn determine_git_status(project: &mut Project) -> bool {
    for location in &mut project.locations {
        if location.git == None {
            let support = determine_git_support(location)
                .expect("Cannot determine git support for project location");
            if support {
                location.git = Some(true);
            } else {
                location.git = Some(false);
            }
        }
    }
    false
}

/// Determine the git support for a given snippet location
pub fn determine_git_support(location: &mut SnippetLocation) -> Result<bool, error::Error> {
    let output = Command::new("git")
        .stdout(Stdio::piped())
        .current_dir(&location.local)
        .args(&["rev-parse", "--is-inside-work-tree"])
        .spawn()?
        .wait_with_output();
    let output_str_result = String::from_utf8(output?.stdout);
    match output_str_result {
        Ok(s) => {
            if s.eq_ignore_ascii_case("true\n") {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        Err(_) => Ok(false),
    }
}
