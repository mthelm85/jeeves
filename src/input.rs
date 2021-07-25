use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jeeves")]
pub struct Jeeves {
    #[structopt(help="The name of your project")]
    pub name: String,
}