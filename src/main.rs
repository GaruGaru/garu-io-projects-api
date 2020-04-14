#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate lru_time_cache;

mod github;
mod models;

use crate::models::Repository;
use crate::github::{Github, GithubClient};
use rocket::State;
use rocket_contrib::json::Json;
use std::error::Error;
use lru_time_cache::LruCache;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};


struct SuperCache {
    cache: LruCache<String, Vec<Repository>>
}

impl SuperCache {
    fn put(&mut self, key: &str, val: Vec<Repository>) {
        self.cache.insert(key.to_string(), val);
    }

    fn get(&mut self, key: &str) -> Option<Vec<Repository>> {
        match self.cache.get(key) {
            Some(res) => Some(res.to_vec()),
            None => None
        }
    }
}

#[get("/")]
fn projects(cache: State<Arc<Mutex<SuperCache>>>, github: State<GithubClient>) -> Result<Json<Vec<Repository>>, Box<dyn Error>> {
    let mut result = cache.lock().unwrap();
    return match result.get("projects") {
        Some(res) => Ok(Json(res.to_vec())),
        None => {
            match github.projects() {
                Ok(resp) => {
                    result.put("projects", resp.clone());
                    Ok(Json(resp.clone()))
                }
                Err(e) => Err(e),
            }
        }
    };
}

fn main() {
    let cache = SuperCache { cache: LruCache::<String, Vec<Repository>>::with_expiry_duration(::std::time::Duration::from_secs(3600)) };

    let shared_cache: Arc<Mutex<SuperCache>> = Arc::new(Mutex::new(cache));

    rocket::ignite()
        .mount("/projects", routes![projects])
        .manage(github::new())
        .manage(shared_cache)
        .launch();
}
