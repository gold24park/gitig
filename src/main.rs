mod cli;
mod git;
mod http;

use anyhow::anyhow;
use base64;
use cli::CmdArgs;
use http::{HttpClient, ReqwestClient};
use std::fs::File;
use std::io;
use structopt::StructOpt;

use crate::git::{FlattenGitTree, GitFileDownloader};

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::from_args();
    println!("{:?}", args);

    let client = ReqwestClient::new();

    let git_tree = FlattenGitTree::init(&client).ok_or(anyhow!("Failed to init project list."))?;

    let element = git_tree
        .get(&args.project)
        .ok_or(anyhow!("Cannot find project: {}", args.project))?;
    // TODO: suggest similar projects

    dbg!(element);

    let content = element.download(&client);

    dbg!(content);

    Ok(())
}
