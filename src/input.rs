use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jeeves")]
pub struct Jeeves {
    #[structopt(help="The name of your project")]
    pub name: String,

    #[structopt(short, long, help="Include if this will be a Pluto project.")]
    pub pluto: bool
}