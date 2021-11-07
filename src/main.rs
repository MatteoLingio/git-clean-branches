use std::env;

fn main() -> std::io::Result<()>{
    //Step 1 get the current path and print it to screen
    let path_buf = env::current_dir()?;
    let path = path_buf.as_path(); 
    println!("This is the current directory {:?}", &path);
    //Step 2 get show branches of the current directory
    Ok(())
}
