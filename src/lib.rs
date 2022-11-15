use std::sync::atomic::AtomicUsize;

use actix_web::{
    get,
    web::{self},
    HttpResponse,
};
use core::sync::atomic::Ordering;
use serde::{Deserialize, Serialize};
use serde_json::json;

static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize)]
pub struct GreetingMessage {
    id: usize,
    name: String,
    message: String,
}

#[get("/v1/greeting/{n}")]
pub async fn get_greeting_message(n: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(GreetingMessage {
        id: COUNTER.fetch_add(1, Ordering::Relaxed),
        name: n.clone(),
        message: format!("Servus, {}! Well met!", n),
    })
}

#[get("/v1/version")]
pub async fn get_version() -> HttpResponse {
    HttpResponse::Ok().json(json! { {"version":""}})
}

#[get("/start")]
pub async fn get_startup() -> HttpResponse {
    HttpResponse::Ok().json(json! { {"ok":true}})
}

#[get("/ready")]
pub async fn get_readiness() -> HttpResponse {
    HttpResponse::Ok().json(json! { {"ok":true}})
}

#[get("/live")]
pub async fn get_liveness() -> HttpResponse {
    HttpResponse::Ok().json(json! { {"ok":true}})
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
}
