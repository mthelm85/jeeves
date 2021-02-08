use std::env;
use std::fs;

pub fn create_dirs(name: &String) -> Result<(), std::io::Error> {
    match fs::create_dir_all(env::current_dir()?.join(name).join("src")) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}