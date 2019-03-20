extern crate actix_web;
use actix_web::{server, App, fs};

fn create_app() -> App {
    App::new()
        .handler(
            "/",
            fs::StaticFiles::new(".")
                .unwrap()
                .show_files_listing()
        )
}

fn main() {
    let port = 3000;
    let host = format!("127.0.0.1:{}", port);

    println!("Starting the server at http://{}...", host);

    server::new(create_app)
        .bind(host)
        .unwrap()
        .run();
}
