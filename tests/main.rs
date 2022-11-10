// Pub to avoid dead code warning: https://github.com/rust-lang/rust/issues/46379
pub mod common;

use common::test_client;
use rocket::http::Status;

#[test]
fn health() {
    let client = test_client().lock().unwrap();
    let req = client.get("/health");
    let response = req.dispatch();
    let expected_content = "ok";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap().contains(expected_content),
        true
    );
}

#[test]
fn index() {
    let client = test_client().lock().unwrap();
    let req = client.get("/");
    let response = req.dispatch();
    let expected_content = "Gov UK Design System Prototyping in Rust";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap().contains(expected_content),
        true
    );
}
