use error;
use project::{Project, SnippetLocation};
use std::ffi::OsStr;
use std::io::Error;
use std::process::Output;
use std::process::{Command, Stdio};

/// Runt git command for this snippet location
fn run_git_command_for<I, S>(location: &SnippetLocation, commands: I) -> Result<Output, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("git")
        .stdout(Stdio::piped())
        .current_dir(&location.local)
        .args(commands)
        .spawn()?
        .wait_with_output()
}

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
    let output = run_git_command_for(location, &["rev-parse", "--is-inside-work-tree"]);
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

/// Struct that gives the git status of the project
pub enum GitStatus {
    Clean,
    Modified,
}

/// Determine the git file status for the snippet location
pub fn determine_git_modified_status(
    location: &SnippetLocation,
) -> Result<GitStatus, error::Error> {
    let output = run_git_command_for(location, &["status", "--porcelain"]);
    let output_str_result = String::from_utf8(output?.stdout);

    output_str_result.map(|s| {
        if s.eq_ignore_ascii_case("") {
            Ok(GitStatus::Clean)
        } else {
            Ok(GitStatus::Modified)
        }
    })?
}

/// Sync/pull git location with upstream repo
pub fn git_pull(location: &SnippetLocation) -> Result<(), error::Error> {
    let output = run_git_command_for(location, &["pull"]);

    // Return if success
    if output?.status.success() {
        Ok(())
    } else {
        Err(error::Error::InternalError(
            "Failed to execute pull command".to_string(),
        ))
    }
}

/// Git push from snippet location
pub fn git_push(location: &SnippetLocation) -> Result<(), error::Error> {
    let output = run_git_command_for(location, &["push"]);

    // Return if success
    if output?.status.success() {
        Ok(())
    } else {
        Err(error::Error::InternalError(
            "Failed to execute push command".to_string(),
        ))
    }
}

/// Git add from snippet location
pub fn git_add(location: &SnippetLocation) -> Result<(), error::Error> {
    let output = run_git_command_for(location, &["add", "-A"]);

    // Return if success
    if output?.status.success() {
        Ok(())
    } else {
        Err(error::Error::InternalError(
            "Failed to execute `add -A` command".to_string(),
        ))
    }
}

/// Git commit from snippet location
pub fn git_commit(location: &SnippetLocation, msg: String) -> Result<(), error::Error> {
    let output = run_git_command_for(location, &["commit", "-am", &format!("\"{}\"", msg)]);

    // Return if success
    if output?.status.success() {
        Ok(())
    } else {
        Err(error::Error::InternalError(
            "Failed to execute `commit -a` command".to_string(),
        ))
    }
}
