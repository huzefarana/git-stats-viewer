extern crate git2;
use git2::Repository;
use std::collections::HashMap;
use std::io;

fn main() {
    // Prompt the user to enter the repository path
    println!("Enter path to your local repo:");
    let mut repo_path = String::new();
    io::stdin()
        .read_line(&mut repo_path)
        .expect("Failed to read input");

    // Trim any leading or trailing whitespaces or newline characters
    let repo_path = repo_path.trim();

    // Open the repository
    let repo = Repository::open(repo_path).expect("Couldn't open repository");
    // Count Commits
    let commit_count = count_commits(&repo);
    println!("Total Commits: {}", commit_count);

    // List Contributors
    let contributors = list_contributors(&repo);
    println!("Contributors:");
    for (contributor, commit_count) in contributors {
        println!("{}: {}", contributor, commit_count);
    }

    // List Most Changed Files
    let changed_files = most_changed_files(&repo, 5); // Display top 5 files
    println!("Most Changed Files:");
    for (file, change_count) in changed_files {
        println!("{}: {}", file, change_count);
    }

    // Display Commit History
    // println!("Commit History:");
    // display_commit_history(&repo, 10);
}
fn count_commits(repo: &Repository) -> usize {
    // Implementation to count commits
    let mut count = 0;
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    for _ in revwalk {
        count += 1;
    }
    count
}

fn list_contributors(repo: &Repository) -> Vec<(String, usize)> {
    // Implementation to list contributors and their commit counts
    let mut contributors = HashMap::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    for oid in revwalk {
        let commit = repo.find_commit(oid.unwrap()).unwrap();
        let author = commit.author();
        let name = author.name().unwrap().to_string();
        let count = contributors.entry(name).or_insert(0);
        *count += 1;
    }
    let mut contributors: Vec<_> = contributors.into_iter().collect();
    contributors.sort_by(|a, b| b.1.cmp(&a.1));
    contributors
}

fn most_changed_files(repo: &Repository, num_files: usize) -> Vec<(String, usize)> {
    // Implementation to identify and list most changed files
    let mut files = HashMap::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    for oid in revwalk {
        let commit = repo.find_commit(oid.unwrap()).unwrap();
        let tree = commit.tree().unwrap();
        for entry in tree.iter() {
            let path = entry.name().unwrap().to_string();
            let count = files.entry(path).or_insert(0);
            *count += 1;
        }
    }
    let mut files: Vec<_> = files.into_iter().collect();
    files.sort_by(|a, b| b.1.cmp(&a.1));
    files.truncate(num_files);
    files
}

// fn display_commit_history(repo: &Repository, num_commits: usize) {
//     // Implementation to display commit history
//     let mut revwalk = repo.revwalk().unwrap();
//     revwalk.push_head().unwrap();
//     for oid in revwalk.take(num_commits) {
//         let commit = repo.find_commit(oid.unwrap()).unwrap();
//         let author = commit.author();
//         let name = author.name().unwrap();
//         let email = author.email().unwrap();
//         let time = commit.time().seconds();
//         let message = commit.message().unwrap();
//         println!("{} <{}> {}", name, email, time);
//         println!("{}", message);
//         println!("");
//     }
// }

