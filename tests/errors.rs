mod common;

use common::*;
use rocket::http::Status;

#[test]
fn not_found() {
    let client = test_client().lock().unwrap();
    let req = client.get("/page_that_doesnt_exist");
    let response = req.dispatch();
    let expected_content = "Page not found";
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.into_string().unwrap().contains(expected_content),
        true
    );
}
