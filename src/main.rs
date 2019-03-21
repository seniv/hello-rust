extern crate actix_web;
extern crate clap;

use actix_web::{server, App, fs};
use clap::{Arg};

fn main() {
    let matches = clap::App::new("Static HTTP server")
        .author("Ivan Seniv <senivq@gmail.com>")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Sets port of the server")
            .takes_value(true))
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .help("Sets address of the server")
            .takes_value(true))
        .arg(Arg::with_name("path")
            .help("Sets path")
            .index(1))
        .get_matches();

    let address = matches.value_of("address").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("3000");
    let path = String::from(matches.value_of("path").unwrap_or("."));

    let host = format!("{}:{}", address, port);

    println!("Starting the server at http://{} in folder \"{}\"", host, path);

    server::new(move || App::new()
        .handler(
            "/",
            fs::StaticFiles::new(&path)
                .unwrap()
                .show_files_listing()
        ))
        .bind(host)
        .unwrap()
        .run();
}
