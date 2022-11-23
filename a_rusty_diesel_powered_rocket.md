# Off the Rails and into a Rusty Rocket

Having spent the last year or so building digital services for government using Ruby on Rails (and a good chunk of time working on Rails app before that), I thought I'd approach learning Rust by seeing how easily I could put together a similar sort of web application, trying to understand the ecosystem and the current state of web dev in Rust.

The first thing to do it seemed was pick a web server framework. While Rails is clearly the dominant, arguably "default", framework in the Ruby world, Rust has a few options with similar maturity and mind share. Some fairly cursory research (I was keen to get started with _something_) suggested the biggest 3 options were [Actix-Web](https://actix.rs/), [Rocket](https://rocket.rs/) and [Axum](https://github.com/tokio-rs/axum). Of those, it seemed that Rocket and Axum had somewhat nicer syntax/ergonomics (to the extent that I was able to judge that with my limited Rust knowledge), though both seemed to be in the middle of breaking changes still at release candidate change.

Recently Axum seems a little more actively developed than Rocket and with a bigger team behind it, but Rocket had far more documentation available, including a number of blogs, examples, books and a matrix channel, and had the biggest real world use case I could find in [Vaultwarden.rs](https://github.com/dani-garcia/vaultwarden), an alternative Bitwarden server implementation. In the end I chose to start with Rocket.

Time to write GET `/hello-world`.

## 101: Start a server, return a basic response

This is pretty well covered by the Rocket guide docs and involved fairly minimal boilerplate. After creating our new cargo project, we add a single dependency:

```rust
# Cargo.toml

[package]
name = "govuk-prototype-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = "0.5.0-rc.2"

```
a single route and handler, and a "launch" (main) function that mounts our route.

```rust
# src/main.rs

#[macro_use]
extern crate rocket;

#[get("/")]
fn start_page() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![start_page])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn start_page() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/");
        let response = req.dispatch();
        let expected_content = "Hello, world!";
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap().contains(expected_content),
            true
        );
    }
}
```

We can run our test with `cargo test`. We can also have cargo generate the directory structure and dependency manifest for us:

```bash
cargo new <my-project>
cargo add rocket
```

So far, so good. A couple of interesting syntax bits to note: 

## Render some server side templated HTML

The next thing I wanted to do was return some server side rendered (and preferably templated) HTML rather than just the static string we started with. Rocket supports two templating languages [Tera](https://tera.netlify.app/) & [Handlebars](https://handlebarsjs.com/). Ideally there'd be a nunjucks parser since that's what the GovUK Design system mostly uses but both the available options seem equally viable.

Tera seems to support more complex templating so that's what I decided to start with. First we need to add support for Template responses to Rocket via a crate `rocket_dyn_templates`. This along with a couple of crates for database support make up the rocket "contrib library" and are part of Rocket, just nicely modularized.

```bash
cargo add rocket_dyn_templates --features tera
``` 

Then we can modify our request handler to return a template an:

```rust
#[get("/")]
fn start_page() -> Template {
    Template::render("index", context! {text: "Hello, world!"})
}
```


## Add frontend assets from an NPM package (ESBuild)

## Add a database (Diesel)

- Version mismatch
- Connect to test database
- Get database handler in tests
- Blocking on async in tests
- Database transactions/rollback in tests


## Make requests to external service (Notify)

- JWT tokens
- API testing

## Background jobs

## Deploying

- Docker
- Render
