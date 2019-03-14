extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn create_app() -> App {
    App::new()
        .resource("/", |r| r.f(index))
}

fn main() {
    println!("Starting the server at port 3000...");

    server::new(create_app)
        .bind("127.0.0.1:3000")
        .unwrap()
        .run();
}
