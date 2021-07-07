use reqwest::blocking::Response;

use crate::models::{Repository, Root};
use std::error::Error;

pub trait Github {
    fn projects(&self) -> Result<Vec<Repository>, Box<dyn std::error::Error>>;
}

pub struct GithubClient {
    client: reqwest::blocking::Client,
}

pub fn new() -> GithubClient {
    GithubClient {
        client: reqwest::blocking::Client::new(),
    }
}

impl Github for GithubClient {
    fn projects(&self) -> Result<Vec<Repository>,  Box<dyn Error>> {
        let host = "https://api.github.com";
        let user = "garugaru";
        let url = format!("{host}/users/{user}/repos?visibility=public&sort=created&affiliation=owner&direction=desc", host = host, user = user);

        let response = match self
            .client
            .get(&url.to_string())
            .header("User-Agent", "GaruGaru")
            .send() {
            Ok(r) => r,
            Err(err) => { return Err(Box::new(err)) }
        };

        let repos_list = match response.json::<Vec<Root>>() {
            Ok(r) => {r}
            Err(err) => { return Err(Box::new(err)) }
        };

        let repos = repos_list
            .iter()
            .filter(|&i| !i.fork)
            .map(|r| Repository {
                name: r.name.to_string(),
                description: match &r.description {
                    Some(val) => val.to_string(),
                    None => "".to_string(),
                },
                language: match &r.language {
                    Some(val) => val.to_string(),
                    None => "".to_string(),
                },
                uri: r.html_url.to_string(),
            })
            .collect::<Vec<Repository>>();

        return Ok(repos);
    }
}
