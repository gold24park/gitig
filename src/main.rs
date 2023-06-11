mod cli;

use cli::CmdArgs;
use std::fs::File;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

fn main() {
    let args = CmdArgs::from_args();
    println!("{:?}", args);

    let path = Path::new(&args.path);
    let _display = path.display();
    let new_path = path.join("a");
    println!("{:?}", new_path);

    let mut path_buf = PathBuf::from("/tmp");
    path_buf.push("file.bk");
}
