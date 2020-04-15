use crate::models::{Repository, Root};

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
    fn projects(&self) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
        let host = "https://api.github.com";
        let user = "garugaru";
        let url = format!("{host}/users/{user}/repos?visibility=public&sort=created&affiliation=owner&direction=desc", host = host, user = user);

        let response: Vec<Root> = self
            .client
            .get(&url.to_string())
            .header("User-Agent", "GaruGaru")
            .send()
            .unwrap()
            .json::<Vec<Root>>()
            .unwrap();

        let repos = response
            .iter()
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
            })
            .collect::<Vec<Repository>>();

        return Ok(repos);
    }
}
