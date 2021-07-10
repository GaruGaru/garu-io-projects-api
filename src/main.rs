extern crate reqwest;
#[macro_use]
extern crate rocket;

use std::convert::Infallible;
use std::error::Error;
use std::sync::{Arc, Mutex};

use http::header::{HeaderName, HOST, InvalidHeaderValue};
use http::HeaderValue;
use opentelemetry::{Context, global, KeyValue};
use opentelemetry::global::BoxedTracer;
use opentelemetry::trace::{Span, TraceError, Tracer};
use opentelemetry_http::HeaderExtractor;
use pocketcache::cache::Cache;
use pocketcache::time::Expiration;
use rocket::{Request, request, Rocket, State};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{json::Json, Serialize};

use crate::github::{Github, GithubClient};
use crate::models::Repository;
use opentelemetry_jaeger::PipelineBuilder;

mod github;
mod models;

struct InMemoryCache {
    cache: Cache<Vec<Repository>>,
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
fn projects(cache: &State<Arc<Mutex<InMemoryCache>>>, github: &State<GithubClient>, traced: TracedHeaders) -> Projects {
    let mut result = cache.lock().unwrap();

    let mut span = global::tracer("api").start_with_context("/projects", traced.parent_ctx);

    return match result.get("projects") {
        Some(res) => {
            span.set_attribute(KeyValue::new("cached", true));
            Projects::Ok(Json(res.to_vec()))
        },
        None => {
            span.set_attribute(KeyValue::new("cached", false));
            match github.projects() {
                Ok(resp) => {
                    result.put("projects", resp.clone());
                    return Projects::Ok(Json(resp.clone()));
                }
                Err(e) => Projects::Error(Json(String::from(e.description())))
            }
        }
    };
}

#[launch]
fn rocket() -> _ {
    let cache = InMemoryCache { cache: Cache::<Vec<Repository>>::new(Expiration::Minute(30)) };
    let shared_cache = Arc::new(Mutex::new(cache));

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("garu-io-projects-api")
        .install_simple()
        .unwrap();


    rocket::build()
        .mount("/", routes![projects, health])
        .manage(github::new())
        .manage(shared_cache)
        .manage(tracer)
}


struct TracedHeaders {
    headers: http::HeaderMap,
    parent_ctx: Context,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TracedHeaders {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let headers: &rocket::http::HeaderMap = req.headers();

        let mut requestHeaders: http::HeaderMap = http::HeaderMap::new();

        for header in headers.iter() {
            let name = HeaderName::from_lowercase(header.name.as_str().as_bytes()).unwrap();
            let value = HeaderValue::from_bytes(header.value.as_bytes()).unwrap();
            requestHeaders.insert(name, value);
        }


        let parent_cx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor(&requestHeaders.clone()))
        });

        return Outcome::Success(TracedHeaders {
            headers: requestHeaders.clone(),
            parent_ctx: parent_cx,
        });
    }
}

