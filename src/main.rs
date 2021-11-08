use std::{env};
use git2::{BranchType, Repository};
use chrono::{NaiveDateTime, Duration};
fn main() -> std::io::Result<()>{
    //Step 1 get the current path and print it to screen
    let path_buf = env::current_dir()?;
    let path = path_buf.as_path(); 
    println!("This is the current directory {:?}", &path);
    //Step 2 get show branches of the current directory
    //Try to open the repository

    let repo = match Repository::open(path)  {
       Ok(repo) => repo,
       Err(e) => panic!("Failed to open the repo in directory {} for error {}", &path.display(), e)
    };

    let branches = get_branches(&repo).unwrap();

    for b in branches {
        println!("branch {:?} - last commit time {}", b.name, b.time);
    }
    Ok(())
}

fn get_branches(repo: &Repository) -> Result<Vec<Branch>, git2::Error> {
    let mut branches = repo
        .branches(Some(BranchType::Local))?
        .map(|branch| {
            let (branch, _) = branch?;
            let name = String::from_utf8(branch.name_bytes()?.to_vec()).unwrap();
            let commit = branch.get().peel_to_commit()?;
            let offset = Duration::minutes(i64::from(commit.time().offset_minutes()));
            let time = NaiveDateTime::from_timestamp(commit.time().seconds(), 0) + offset;
            // println!("the time of commit is {:?}", time);
            Ok(Branch {
                name,
                branch,
                time
            })
        })
        .filter(|branch| {
            return match branch {
                Ok(branch) => {
                    branch.name != "master" && branch.name != "main"
                },
                Err(e) => panic!("Error {}", e)
            };
        })
        .collect::<Result<Vec<_>, git2::Error>>()?;

    branches.sort_unstable_by_key(|branch| branch.time);
    
    Ok(branches)
}

struct Branch<'repo> {
    name: String,
    branch: git2::Branch<'repo>,
    time: NaiveDateTime
}