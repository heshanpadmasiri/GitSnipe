use std::{path::PathBuf, env::args};

use git2::{Repository};

enum Error {
    FailedToParseArgs,
    FailedToOpenRepository,
}

struct Config {
    repo_path: PathBuf
}

trait Commit {
    fn hash(&self) -> String;
    fn output(&self) -> String;
}

struct BaseCommit {
    commit_hash : String,
    commit_message : String
}

impl Commit for BaseCommit {
    fn hash(&self) -> String {
        self.commit_hash.to_owned()
    }

    fn output(&self) -> String {
        self.commit_message.to_owned()
    }
}

fn main() {
    match parse_args() {
        Ok(config) => {
            match find_commits(config) {
                Ok(commits) => {
                    for each in commits { print_commit(&each)}
                }
                Err(_) => {
                    eprintln!("Error: Failed to find commits")
                }
            }
        }
        Err(_) => {
            eprintln!("Error: Invalid input. Use git_snipe <path to repository>")
        }
    }
}

fn print_commit(commit: &Box<dyn Commit>) {
    print!("{}: {}", commit.hash(), commit.output())
}

fn find_commits(config: Config) -> Result<Vec<Box<dyn Commit>>, Error> {
    match Repository::open(config.repo_path) {
        Ok(repo) => {
            let mut result : Vec<Box<dyn Commit>>= vec![];
            // TODO: properly return error here
            let mut walk = repo.revwalk().unwrap();
            walk.push_head().unwrap();
            // TODO: make this an iterator
            for oid in walk {
                let oid = oid.unwrap();
                let commit = repo.find_commit(oid).unwrap();
                let commit_hash = oid.to_string();
                let commit_message = commit.message().unwrap_or("NO MESSAGE").to_string();
                result.push(Box::new(BaseCommit {commit_hash, commit_message}))
            }
            Ok(result)
        }
        Err(_) => {
            Err(Error::FailedToOpenRepository)
        }
    }
}

fn parse_args() -> Result<Config, Error> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        return Err(Error::FailedToParseArgs)
    }
    let repo_path = PathBuf::from(&args[1]);
    return Ok(Config { repo_path })
}
