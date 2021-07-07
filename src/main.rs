extern crate reqwest;
#[macro_use]
extern crate rocket;

use std::error::Error;
use std::sync::{Arc, Mutex};

use rocket::{Rocket, State};
use rocket::serde::{json::Json, Serialize};

use crate::github::{Github, GithubClient};
use crate::models::Repository;
use pocketcache::cache::Cache;
use pocketcache::time::Expiration;

mod github;
mod models;

struct InMemoryCache {
    cache: Cache<Vec<Repository>>
}

impl InMemoryCache {
    fn put(&mut self, key: &str, val: Vec<Repository>) {
        self.cache.set(key, val);
    }

    fn get(&mut self, key: &str) -> Option<Vec<Repository>> {
        match self.cache.get(key) {
            Some(res) => Some(res.to_vec()),
            None => None
        }
    }
}

#[derive(Responder)]
//#[response(bound = "T: Serialize")]
enum Projects {
    Ok(Json<Vec<Repository>>),
    #[response(status = 500)]
    Error(Json<String>),
}


#[get("/healthz")]
fn health() -> &'static str {
    return "so far so good";
}

#[get("/projects")]
fn projects(cache: &State<Arc<Mutex<InMemoryCache>>>, github: &State<GithubClient>) -> Projects {
    let mut result = cache.lock().unwrap();
    return match result.get("projects") {
        Some(res) => Projects::Ok(Json(res.to_vec())),
        None => {
            match github.projects() {
                Ok(resp) => {
                    result.put("projects", resp.clone());
                    return Projects::Ok(Json(resp.clone()));
                }
                Err(e) => Projects::Error(Json(String::from("e")))
            }
        }
    };
}
//
//#[launch]
//fn rocket()  -> _  {
//    let cache = ;
//    let shared_cache: Arc<Mutex<InMemoryCache>> = Arc::new(Mutex::new(cache));
//
//    return rocket::build()
//        .mount("/", routes![projects, health])
//        .manage(github::new())
//        .manage(shared_cache);
//}
//


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let cache = InMemoryCache { cache: Cache::<Vec<Repository>>::new( Expiration::Minute(30)) };
    let shared_cache = Arc::new(Mutex::new(cache));
    rocket::build()
        .mount("/", routes![projects, health])
        .manage(github::new())
        .manage(shared_cache)
}
