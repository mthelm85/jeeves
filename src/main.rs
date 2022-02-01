use std::env;
use std::fs::{ self, File };
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;

mod input;
mod julia_ver;

fn main() -> std::io::Result<()> {
    let args = input::Jeeves::from_args();

    let user_version = julia_ver::julia_ver();

    fs::create_dir_all(env::current_dir()?.join(&args.name).join("src"))?;
    fs::write(Path::new(&args.name).join("src").join(format!("{}.jl", &args.name)), "println(\"Hello, world!\")")?;
    fs::write(
        Path::new(&args.name).join("Manifest.toml"), 
    format!("# This file is machine-generated - editing it directly is not advised \n\n julia_version = \"{}.{}.{}\" \n manifest_format = \"2.0\"",
                    user_version.major,
                    user_version.minor,
                    user_version.patch
                    )
    )?;

    let project_name = format!("name = \"{}\"\n", &args.name);

    let mut project_toml = File::create(Path::new(&args.name).join("Project.toml"))?;

    project_toml.write_all(project_name.as_bytes())?;

    Ok(())
}