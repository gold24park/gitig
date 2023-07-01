mod cli;
mod git;
mod http;

use std::{fs, io};

use anyhow::{anyhow, Ok};
use cli::CmdArgs;
use http::ReqwestClient;
use structopt::StructOpt;

use crate::git::{FlattenGitTree, GitFileDownloader};

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::from_args();

    let client = ReqwestClient::new();

    let git_tree = FlattenGitTree::init(&client).ok_or(anyhow!("Failed to init project list."))?;

    if args.list {
        git_tree
            .iter()
            .filter(|e| {
                e.project
                    .to_lowercase()
                    .starts_with(&args.project.to_lowercase())
            })
            .for_each(|e| println!("- {}", e.project))
    } else {
        match git_tree.get(&args.project) {
            Some(element) => {
                let content = element
                    .download(&client)
                    .ok_or(anyhow!("Failed to download gitignore."))?;

                let file_path = args.path.as_path().join(".gitignore");

                if file_path.exists() {
                    // ask user to overwrite
                    println!("File already exists. Overwrite? [y/n]");

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if input.to_lowercase().trim() != "y" {
                        return Ok(());
                    }
                }

                fs::write(file_path, content)
                    .map_err(|e| anyhow!("Failed to write file: {}", e))?;

                println!("Successfully created .gitignore for {}", args.project);
            }
            None => {
                let suggest_keywords = git_tree.suggest_keywords(&args.project);
                println!(
                    "No project found for \"{}\". Did you mean one of these?",
                    args.project
                );

                for keyword in suggest_keywords {
                    println!("- {}", keyword);
                }
            }
        }
    }
    Ok(())
}
