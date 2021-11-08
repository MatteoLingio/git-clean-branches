use std::{env};
use git2::{BranchType, Oid, Repository};
use chrono::{NaiveDateTime, Duration};
fn main() -> std::io::Result<()>{
    let path_buf = env::current_dir()?;
    let path = path_buf.as_path(); 
    println!("This is the current directory {:?}", &path);
    //Try to open the repository
    let repo = match Repository::open(path)  {
       Ok(repo) => repo,
       Err(e) => panic!("Failed to open the repo in directory {} for error {}", &path.display(), e)
    };

    let branches = get_branches(&repo).unwrap();

    for b in branches {
        println!("branch {:?} - last commit {} - at {}", b.name, b.last_commit_id, b.time);
    }
    Ok(())
}

//creates a Vector of Branch with the corerct informations with main/master filtered out
//The vector is sorted by oldest commit
fn get_branches(repo: &Repository) -> Result<Vec<Branch>, git2::Error> {

    let mut branches = repo
        .branches(Some(BranchType::Local))?
        .map(|branch| {
            let (branch, _) = branch?;
            let name = String::from_utf8(branch.name_bytes()?.to_vec()).unwrap();
            let commit = branch.get().peel_to_commit()?;
            let last_commit_id = commit.id();
            let offset = Duration::minutes(i64::from(commit.time().offset_minutes()));
            let time = NaiveDateTime::from_timestamp(commit.time().seconds(), 0) + offset;
            Ok(Branch {
                name,
                branch,
                time,
                last_commit_id
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
    time: NaiveDateTime,
    last_commit_id: Oid
}