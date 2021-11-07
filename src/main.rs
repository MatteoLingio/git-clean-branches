use std::env;

fn main() -> std::io::Result<()>{
    //Step 1 get the current path and print it to screen
    let path = env::current_dir()?;
    println!("This is the current directory {:?}", path);
    Ok(())
}
