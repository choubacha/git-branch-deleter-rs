extern crate git2;
use git2::{Repository, BranchType};
use std::path::Path;
use std::io;

fn main() {
    let repo = Repository::open(&Path::new(".")).unwrap();
    let branches = repo.branches(Some(BranchType::Local)).unwrap();
    for mut m in branches {
        if let Ok((ref mut branch, BranchType::Local)) = m {
            if branch.is_head() {
                println!("Skipping branch {} as it is currently HEAD", branch.name().unwrap().unwrap());
                break
            }

            'branch_questions: loop {
                println!("Delete? {}\n(y/n): ", branch.name().unwrap().unwrap());

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
}
