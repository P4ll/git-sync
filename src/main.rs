use std::error;
use xshell::{Shell, cmd};

fn main() {
    let sh = Shell::new();
    if let Ok(res) = sh {
        let branch = cmd!(res, "git rev-parse --abbrev-ref HEAD").read().expect("dfdf");
        dbg!("{}", branch);
    }
}

