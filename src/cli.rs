use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gitig",
    about = "Creates a gitignore boilerplate for the configured project.",
    author = "gold24park"
)]
pub struct CmdArgs {
    /// The project for which to create a .gitignore.
    #[structopt(default_value = "")]
    pub project: String,

    /// The path to create the .gitignore.
    #[structopt(parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    //// List available projects
    #[structopt(short, long)]
    pub list: bool,
}
