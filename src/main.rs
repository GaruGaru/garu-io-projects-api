#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
extern crate reqwest;

mod github;
mod models;

use crate::models::{Repository};
use crate::github::{Github, GithubClient};
use rocket::State;
use rocket_contrib::json::Json;
use std::error::Error;

#[get("/")]
fn index(github: State<GithubClient>) -> Result<Json<Vec<Repository>>, Box<dyn Error>> {
    return match github.projects() {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    };
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .manage(github::new())
        .launch();
}
