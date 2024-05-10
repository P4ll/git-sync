use xshell::{Shell, cmd};
use anyhow;
use chrono::prelude::*;

fn sync_current_repo() -> anyhow::Result<()> {
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

fn main() -> anyhow::Result<()> {
    sync_current_repo()?;
    Ok(())
}

