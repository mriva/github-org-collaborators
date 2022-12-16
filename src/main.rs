mod client;

use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct GithubUser {
    login: String,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Collaborator {
    login: String,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    private: bool,
    language: Option<String>,
    visibility: String,
}

fn main() -> Result<()> {
    let url = "https://api.github.com/orgs/soisy/repos?per_page=2";

    let collaborators_map = analyze_org_users(url)?;

    for (login, repos) in &collaborators_map {
        println!("{}({}): {:?}", login, repos.len(), repos);
    }

    Ok(())
}

fn analyze_org_users(org_url: &str) -> Result<HashMap<String, Vec<String>>, anyhow::Error> {
    let repos_body = client::get(org_url)?;
    let repos: Vec<Repository> = serde_json::from_str(&repos_body)?;

    let mut collaborators_map: HashMap<String, Vec<String>> = HashMap::new();

    for repo in repos {
        let collaborators = get_repo_collaborators(&repo.full_name)?;
        for (c, r) in collaborators.iter() {
            collaborators_map.entry(c.to_string())
                .or_insert(Vec::new())
                .push(r.to_string());
        };
    }

    Ok(collaborators_map)
}

fn get_repo_collaborators(repo_name: &str) -> Result<Vec<(String, String)>> {
    let collaborators_body = client::get(&format!("https://api.github.com/repos/{}/collaborators", repo_name))?;
    let collaborators: Vec<Collaborator> = serde_json::from_str(&collaborators_body)?;

    Ok(collaborators
        .iter()
        .map(|c| (c.login.to_owned(), repo_name.to_owned()))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_collaborators() {
        fn get_repo_collaborators() -> Result<Vec<(String, String)>> {
            Ok(vec![(String::from("cippa"), String::from("lippa"))])
        }

        assert_eq!(get_repo_collaborators().unwrap(), vec![(String::from("cippa"), String::from("lippa"))]);
    }
}
