use git2::{Error, Object, Repository};
use std::env::current_dir;
use std::path::PathBuf;

mod git_actions;
mod inputs;
mod utils;

use git_actions::*;
// use inputs::*;
// use utils::*;

fn main() {
    let cwd: PathBuf = current_dir().unwrap();
    let is_repo: bool = Repository::discover(cwd.clone()).is_ok();

    if is_repo {
        let repo: Repository = Repository::open(cwd.clone()).unwrap();
        let rev_parse_single: Result<Object<'_>, Error> = repo.revparse_single("HEAD");

        if rev_parse_single.is_err() {
            println!("No commits yet");
            make_initial_commit();
        } else {
            print!("There are commits");
        }
    } else {
        println!("Not a git repository");
    }
}
