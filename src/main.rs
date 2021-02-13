use std::env;
use std::fs::{ self, File };
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;
use uuid::Uuid;

mod input;

fn main() -> std::io::Result<()> {
    let args = input::Jeeves::from_args();

    if args.pluto {
        fs::create_dir_all(env::current_dir()?.join(&args.name))?;
        fs::write(Path::new(&args.name).join(format!("{}.jl", &args.name)), "using Pluto\n\nPluto.run()")?;
    } else {
        fs::create_dir_all(env::current_dir()?.join(&args.name).join("src"))?;
        fs::write(Path::new(&args.name).join("src").join(format!("{}.jl", &args.name)), "println(\"Hello, world!\")")?;
    }

    fs::write(Path::new(&args.name).join("Manifest.toml"), "# This file is machine-generated - editing it directly is not advised")?;

    let project_name = format!("name = \"{}\"\n", &args.name);
    let project_uuid = format!("uuid = \"{}\"", Uuid::new_v4());

    let mut project_toml = File::create(Path::new(&args.name).join("Project.toml"))?;

    project_toml.write_all(project_name.as_bytes())?;
    project_toml.write_all(project_uuid.as_bytes())?;

    Ok(())
}