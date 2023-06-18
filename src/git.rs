use crate::http::HttpClient;
use serde::Deserialize;

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

pub trait ProjectMatcher {
    fn matches(&self, project: &str) -> bool;
}

impl ProjectMatcher for Element {
    fn matches(&self, project: &str) -> bool {
        self.project.to_lowercase() == project.to_lowercase()
    }
}

impl FlattenGitTree {
    pub fn init(client: &dyn HttpClient) -> Option<FlattenGitTree> {
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