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
        .arg(Arg::with_name("path")
            .help("Sets path")
            .index(1))
        .get_matches();

    let port = matches.value_of("port").unwrap_or("3000");
    let path = matches.value_of("path").unwrap_or(".").to_string();
    let addrs = get_if_addrs::get_if_addrs().unwrap();

    println!("Starting the server in folder \"{}\" at addresses", path);

    let mut srv = server::new(move || App::new()
        .handler(
            "/",
            fs::StaticFiles::new(&path)
                .unwrap()
                .show_files_listing()
        ));

    for addr in addrs {
        let host = format!("{}:{}", addr.ip(), port);
        println!("http://{}", host);
        srv = srv.bind(host).unwrap();
    }
    srv.run();
}
