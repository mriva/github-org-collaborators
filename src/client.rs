use reqwest::{ blocking::Client, Method };
use anyhow::Result;

pub fn get(url: &str) -> Result<String> {
    make_request(Method::GET, url)
}

fn make_request(method: Method, url: &str) -> Result<String> {
    let client = Client::new();

    let token = std::env::var("GITHUB_TOKEN")?;
    
    let res = client.request(method, url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-github-client")
        .send()?;

    Ok(res.text()?)
}
