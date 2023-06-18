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

use crate::git::FlattenGitTree;

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::from_args();
    println!("{:?}", args);

    let client: Box<dyn HttpClient> = Box::new(ReqwestClient);

    let git_tree =
        FlattenGitTree::init(client.as_ref()).ok_or(anyhow!("Failed to init project list."))?;

    let element = git_tree
        .get(&args.project)
        .ok_or(anyhow!("Cannot find project: {}", args.project)); // TODO: suggest similar projects

    dbg!(element);

    Ok(())
}

async fn download_blob(project: &str) {
    let uri = format!(
        "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
        project,
    );
    let mut resp = reqwest::get(uri).await.expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let mut out = File::create("test").expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("faield to copy content");

    let mut b64 = String::from("IyBMb2dzCmxvZ3MKKi5sb2cKbnBtLWRlYnVnLmxvZyoKeWFybi1kZWJ1Zy5s\nb2cqCnlhcm4tZXJyb3IubG9nKgpsZXJuYS1kZWJ1Zy5sb2cqCi5wbnBtLWRl\nYnVnLmxvZyoKCiMgRGlhZ25vc3RpYyByZXBvcnRzIChodHRwczovL25vZGVq\ncy5vcmcvYXBpL3JlcG9ydC5odG1sKQpyZXBvcnQuWzAtOV0qLlswLTldKi5b\nMC05XSouWzAtOV0qLmpzb24KCiMgUnVudGltZSBkYXRhCnBpZHMKKi5waWQK\nKi5zZWVkCioucGlkLmxvY2sKCiMgRGlyZWN0b3J5IGZvciBpbnN0cnVtZW50\nZWQgbGlicyBnZW5lcmF0ZWQgYnkganNjb3ZlcmFnZS9KU0NvdmVyCmxpYi1j\nb3YKCiMgQ292ZXJhZ2UgZGlyZWN0b3J5IHVzZWQgYnkgdG9vbHMgbGlrZSBp\nc3RhbmJ1bApjb3ZlcmFnZQoqLmxjb3YKCiMgbnljIHRlc3QgY292ZXJhZ2UK\nLm55Y19vdXRwdXQKCiMgR3J1bnQgaW50ZXJtZWRpYXRlIHN0b3JhZ2UgKGh0\ndHBzOi8vZ3J1bnRqcy5jb20vY3JlYXRpbmctcGx1Z2lucyNzdG9yaW5nLXRh\nc2stZmlsZXMpCi5ncnVudAoKIyBCb3dlciBkZXBlbmRlbmN5IGRpcmVjdG9y\neSAoaHR0cHM6Ly9ib3dlci5pby8pCmJvd2VyX2NvbXBvbmVudHMKCiMgbm9k\nZS13YWYgY29uZmlndXJhdGlvbgoubG9jay13c2NyaXB0CgojIENvbXBpbGVk\nIGJpbmFyeSBhZGRvbnMgKGh0dHBzOi8vbm9kZWpzLm9yZy9hcGkvYWRkb25z\nLmh0bWwpCmJ1aWxkL1JlbGVhc2UKCiMgRGVwZW5kZW5jeSBkaXJlY3Rvcmll\ncwpub2RlX21vZHVsZXMvCmpzcG1fcGFja2FnZXMvCgojIFNub3dwYWNrIGRl\ncGVuZGVuY3kgZGlyZWN0b3J5IChodHRwczovL3Nub3dwYWNrLmRldi8pCndl\nYl9tb2R1bGVzLwoKIyBUeXBlU2NyaXB0IGNhY2hlCioudHNidWlsZGluZm8K\nCiMgT3B0aW9uYWwgbnBtIGNhY2hlIGRpcmVjdG9yeQoubnBtCgojIE9wdGlv\nbmFsIGVzbGludCBjYWNoZQouZXNsaW50Y2FjaGUKCiMgT3B0aW9uYWwgc3R5\nbGVsaW50IGNhY2hlCi5zdHlsZWxpbnRjYWNoZQoKIyBNaWNyb2J1bmRsZSBj\nYWNoZQoucnB0Ml9jYWNoZS8KLnJ0czJfY2FjaGVfY2pzLwoucnRzMl9jYWNo\nZV9lcy8KLnJ0czJfY2FjaGVfdW1kLwoKIyBPcHRpb25hbCBSRVBMIGhpc3Rv\ncnkKLm5vZGVfcmVwbF9oaXN0b3J5CgojIE91dHB1dCBvZiAnbnBtIHBhY2sn\nCioudGd6CgojIFlhcm4gSW50ZWdyaXR5IGZpbGUKLnlhcm4taW50ZWdyaXR5\nCgojIGRvdGVudiBlbnZpcm9ubWVudCB2YXJpYWJsZSBmaWxlcwouZW52Ci5l\nbnYuZGV2ZWxvcG1lbnQubG9jYWwKLmVudi50ZXN0LmxvY2FsCi5lbnYucHJv\nZHVjdGlvbi5sb2NhbAouZW52LmxvY2FsCgojIHBhcmNlbC1idW5kbGVyIGNh\nY2hlIChodHRwczovL3BhcmNlbGpzLm9yZy8pCi5jYWNoZQoucGFyY2VsLWNh\nY2hlCgojIE5leHQuanMgYnVpbGQgb3V0cHV0Ci5uZXh0Cm91dAoKIyBOdXh0\nLmpzIGJ1aWxkIC8gZ2VuZXJhdGUgb3V0cHV0Ci5udXh0CmRpc3QKCiMgR2F0\nc2J5IGZpbGVzCi5jYWNoZS8KIyBDb21tZW50IGluIHRoZSBwdWJsaWMgbGlu\nZSBpbiBpZiB5b3VyIHByb2plY3QgdXNlcyBHYXRzYnkgYW5kIG5vdCBOZXh0\nLmpzCiMgaHR0cHM6Ly9uZXh0anMub3JnL2Jsb2cvbmV4dC05LTEjcHVibGlj\nLWRpcmVjdG9yeS1zdXBwb3J0CiMgcHVibGljCgojIHZ1ZXByZXNzIGJ1aWxk\nIG91dHB1dAoudnVlcHJlc3MvZGlzdAoKIyB2dWVwcmVzcyB2Mi54IHRlbXAg\nYW5kIGNhY2hlIGRpcmVjdG9yeQoudGVtcAouY2FjaGUKCiMgRG9jdXNhdXJ1\ncyBjYWNoZSBhbmQgZ2VuZXJhdGVkIGZpbGVzCi5kb2N1c2F1cnVzCgojIFNl\ncnZlcmxlc3MgZGlyZWN0b3JpZXMKLnNlcnZlcmxlc3MvCgojIEZ1c2VCb3gg\nY2FjaGUKLmZ1c2Vib3gvCgojIER5bmFtb0RCIExvY2FsIGZpbGVzCi5keW5h\nbW9kYi8KCiMgVGVybkpTIHBvcnQgZmlsZQoudGVybi1wb3J0CgojIFN0b3Jl\ncyBWU0NvZGUgdmVyc2lvbnMgdXNlZCBmb3IgdGVzdGluZyBWU0NvZGUgZXh0\nZW5zaW9ucwoudnNjb2RlLXRlc3QKCiMgeWFybiB2MgoueWFybi9jYWNoZQou\neWFybi91bnBsdWdnZWQKLnlhcm4vYnVpbGQtc3RhdGUueW1sCi55YXJuL2lu\nc3RhbGwtc3RhdGUuZ3oKLnBucC4qCg==\n");
    b64 = b64.replace("\n", "");
    let bytes = base64::decode(b64).unwrap();
    let str = String::from_utf8(bytes).unwrap();
    println!("{}", str);
}
