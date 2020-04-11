#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;

#[macro_use]
extern crate rocket;

use rocket_contrib::json::{Json};
use serde::Serialize;

#[derive(Serialize)]
struct Repository {
    name: String,
    description: String,
}

#[get("/")]
fn index() -> Json<Vec<Repository>> {
    Json(vec![
        Repository {
            name: "test".to_string(),
            description: "this is a really nice project".to_string(),
        },
        Repository
        {
            name: "test".to_string(),
            description: "this is a really nice project".to_string(),
        }
    ])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
