use std::error;
use xshell::{Shell, cmd};

use chrono::prelude::*;


#[derive(Debug)]
struct MyError;

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dfjdfdf")
    }
}


fn sync_current_repo() -> Result<(), Box<dyn error::Error>> {
    let sh = Shell::new()?;
    let branch = cmd!(sh, "git rev-parse --abbrev-ref HEAD").read()?;
    let dir_path = std::env::current_dir()?;
    let curr_time = Utc::now().round_subsecs(3);
    let dir_name = dir_path.file_name().expect("Cant get dir name").to_str().expect("Cant parse dir name");

    let commit_message = format!("{}: backup at {}", dir_name, curr_time);

    cmd!(sh, "git add .").run()?;
    cmd!(sh, "git commit -m {commit_message}").run()?;
    cmd!(sh, "git push origin {branch}").run()?;

    Ok(())

}

fn main() -> Result<(), Box<dyn error::Error>> {
    sync_current_repo()?;
    Ok(())
}

