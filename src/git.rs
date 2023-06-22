use std::cmp::min;

use crate::http::HttpClient;
use serde::Deserialize;

#[cfg(test)]
mod git_test {
    use super::*;

    #[test]
    fn match_test() {
        let project = "node";
        let element = Element {
            project: "Node".to_string(),
            path: "Node.gitignore".to_string(),
            sha: "123".to_string(),
        };
        assert!(element.matches(project));
    }

    #[test]
    fn extract_test() {
        let path1 = "Some/Path/Target.gitignore";
        let path2 = "Target.gitignore";
        let path3 = "Some/Target";
        let path4 = "Target";

        assert!(extract_project_name(path1).is_some());
        assert_eq!(extract_project_name(path1), Some("Target"));
        assert_eq!(extract_project_name(path2), Some("Target"));
        assert!(extract_project_name(path3).is_none());
        assert!(extract_project_name(path4).is_none());
    }
}

#[derive(Debug, Deserialize)]
pub struct FlattenGitTree {
    #[serde(rename = "tree")]
    elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
pub struct Element {
    #[serde(skip)]
    project: String,
    path: String,
    sha: String,
}

#[derive(Debug, Deserialize)]
struct GitFile {
    content: String,
}

pub trait GitFileDownloader {
    fn download(&self, client: &impl HttpClient) -> Option<String>;
}

impl GitFileDownloader for Element {
    fn download(&self, client: &impl HttpClient) -> Option<String> {
        let url = format!(
            "https://api.github.com/repos/github/gitignore/git/blobs/{}",
            self.sha
        );

        let body = client.get(url.as_str()).ok()?;

        let git_file: GitFile = serde_json::from_str(&body).ok()?;

        let b64 = git_file.content.replace("\n", "");
        let bytes = base64::decode(b64).ok()?;

        String::from_utf8(bytes).ok()
    }
}

pub trait ProjectMatcher {
    fn matches(&self, project: &str) -> bool;
}

impl ProjectMatcher for Element {
    fn matches(&self, project: &str) -> bool {
        self.project.to_lowercase() == project.to_lowercase()
    }
}

impl FlattenGitTree {
    pub fn init(client: &impl HttpClient) -> Option<FlattenGitTree> {
        let url = "https://api.github.com/repos/github/gitignore/git/trees/main?recursive=0";
        let body = client.get(url).ok()?;

        let mut git_tree: FlattenGitTree = serde_json::from_str(&body).ok()?;
        git_tree.elements = git_tree
            .elements
            .iter()
            .filter_map(|e| match extract_project_name(&e.path) {
                Some(project) => Some(Element {
                    project: project.to_string(),
                    path: e.path.clone(),
                    sha: e.sha.clone(),
                }),
                None => None,
            })
            .collect::<Vec<Element>>();

        Some(git_tree)
    }

    pub fn get(&self, project: &str) -> Option<&Element> {
        self.elements.iter().find(|e| e.matches(project))
    }

    pub fn suggest_keywords(&self, keyword: &str) -> Vec<String> {
        let keywords: Vec<String> = self
            .elements
            .iter()
            .map(|e| e.project.to_lowercase())
            .collect();

        let mut candidates: Vec<(&str, usize)> = vec![];
        let mut min_dist = usize::MAX;

        for candidate in keywords.iter() {
            let mut dist = levenshtein(&keyword, candidate);

            if dist == candidate.len() {
                dist = usize::MAX;
            }

            if candidate.starts_with(&keyword) {
                dist = 0;
            }

            candidates.push((candidate, dist));
            min_dist = min(min_dist, dist)
        }

        candidates.sort_by_key(|&(_, dist)| dist);

        candidates
            .iter()
            .filter_map(|&(v, dist)| match dist <= min_dist + 1 {
                true => Some(v.to_owned()),
                false => None,
            })
            .collect::<Vec<String>>()
    }
}

fn extract_project_name(path: &str) -> Option<&str> {
    let start = if path.contains("/") {
        path.rfind('/')? + 1
    } else {
        0
    };
    let end = path.rfind(".gitignore")?;

    if start <= end {
        Some(&path[start..end])
    } else {
        None
    }
}

fn levenshtein(src: &str, target: &str) -> usize {
    let r = src.len() + 1;
    let c = target.len() + 1;
    let mut dist = vec![vec![0; c]; r];

    for i in 0..r {
        dist[i][0] = i
    }

    for i in 0..c {
        dist[0][i] = i;
    }

    for i in 1..r {
        for j in 1..c {
            let cost = if src[i - 1..i] == target[j - 1..j] {
                0
            } else {
                1
            };

            dist[i][j] = min(
                dist[i - 1][j] + 1,
                min(dist[i][j - 1] + 1, dist[i - 1][j - 1] + cost),
            )
        }
    }

    dist[src.len()][target.len()]
}
