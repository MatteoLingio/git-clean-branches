use std::env;
use git2::{BranchType, Repository};
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


    let branches = match repo.branches(Some(BranchType::Local)){
        Ok(branches) => branches,
        Err(e) => panic!("Failed to get branches in this repository: {}", e)
    };

    for b in branches {
        let (branch, _) = b.unwrap();
        let commit = branch.get().peel_to_commit().unwrap();
        let sum = commit.summary().unwrap();
        match branch.name() {
            Ok(name) => {
                println!("branch: {} {}",name.unwrap(), &sum);
                if name != Some("develop") && name != Some("master") && name != Some("main"){
                    println!("{}", name.unwrap());
                }
            },
            Err(_) => {},
        };
    }

    Ok(())
}