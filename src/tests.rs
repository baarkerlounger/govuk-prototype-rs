mod test {
    use crate::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn health() {
        let client = Client::tracked(rocket()).unwrap();
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
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/");
        let response = req.dispatch();
        let expected_content = "Gov UK Design System Prototyping in Rust";
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap().contains(expected_content),
            true
        );
    }

    #[test]
    fn user_page() {
        let client = Client::tracked(rocket()).unwrap();
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
        let client = Client::tracked(rocket()).unwrap();
        let req = client
            .post("/users")
            .header(ContentType::Form)
            .body("name=john%20doe&email=john.doe@example.com&age=28");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Created);
    }

    #[test]
    fn not_found() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/page_that_doesnt_exist");
        let response = req.dispatch();
        let expected_content = "Page not found";
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.into_string().unwrap().contains(expected_content),
            true
        );
    }
}
