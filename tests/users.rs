mod common;

use common::*;
use rocket::http::{ContentType, Status};

#[test]
fn show_user() {
    let client = test_client().lock().unwrap();
    let req = client.get("/users/3e2dd4ae-3c37-40c6-aa64-7061f284ce28");
    let response = req.dispatch();
    let expected_content = "John Doe";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap().contains(expected_content),
        true
    );
}

#[test]
fn create_user() {
    let client = test_client().lock().unwrap();
    let req = client
        .post("/users")
        .header(ContentType::Form)
        .body("name=john%20doe&email=john.doe@example.com&age=28");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::new(303));
}

#[test]
fn api_create_user() {
    let client = test_client().lock().unwrap();
    let req = client
        .post("/api/users")
        .header(ContentType::Form)
        .body("name=john%20doe&email=john.doe@example.com&age=28");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Created);
}

#[test]
fn user_index() {
    let client = test_client().lock().unwrap();
    client
        .post("/users")
        .header(ContentType::Form)
        .body("name=testname23&email=testname23@example.com&age=28")
        .dispatch();
    let req = client.get("/users");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap().contains("testname23"), true);
}

#[test]
fn new_user() {
    let client = test_client().lock().unwrap();
    let req = client.get("/users/new");
    let response = req.dispatch();
    let expected_content = "Create a new user";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap().contains(expected_content),
        true
    );
}
