extern crate git2;
use git2::{Repository, BranchType};
use std::path::Path;
use std::io::{self, Read};

fn main() {
    let repo = Repository::open(&Path::new(".")).unwrap();
    match repo.branches(Some(BranchType::Local)) {
        Ok(branches) => {
            for mut m in branches {
                if let Ok((ref mut branch, BranchType::Local)) = m {
                    'branch_questions: loop {
                        println!("Delete? (Y/n) {}", branch.name().unwrap().unwrap());
                        let mut buffer = String::new();
                        io::stdin().read_line(&mut buffer).unwrap();
                        match buffer.trim().to_lowercase().as_str() {
                            "yes" | "y" => {
                                match branch.delete() {
                                    Ok(_) => println!("Deleted"),
                                    Err(e) => println!("Error deleting: {}", e),
                                };
                                break 'branch_questions;
                            },
                            "no" | "n" | "" => {
                                println!("Skipped");
                                break 'branch_questions;
                            },
                            other => println!("Did not recognize {}", other),
                        }
                    }
                }
            }
        },
        Err(_) => {
            println!("Failed to find branches for some reason");
            std::process::exit(1);
        }
    }
    repo;
}
