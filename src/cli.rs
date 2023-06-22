use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gitignore",
    about = "generates gitignore for specified project",
    author = "gold24park"
)]
pub struct CmdArgs {
    /// gitignore를 생성할 project
    pub project: String,

    /// 지정한 경로에 .gitignore 생성
    #[structopt(parse(from_os_str), default_value = ".")]
    pub path: PathBuf,
}
