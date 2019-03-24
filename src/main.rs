extern crate actix_web;
extern crate clap;

use actix_web::{server, App, fs};
use clap::{Arg};

fn select_address() -> String {
    let addrs = get_if_addrs::get_if_addrs().unwrap();

    println!("Please, enter index of address you want to bind");
    let mut iter_index = 0;
    addrs.iter().for_each(|addr| {
        println!("[{}]: {}", iter_index, addr.ip());
        iter_index += 1;
    });

    let mut index = String::new();
    std::io::stdin().read_line(&mut index).unwrap();
    let index: usize = index.trim().parse().unwrap();

    addrs[index].ip().to_string()
}

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
        .arg(Arg::with_name("select")
            .short("s")
            .long("select")
            .help("Select which address you want to bind"))
        .arg(Arg::with_name("path")
            .help("Sets path")
            .index(1))
        .get_matches();

    let display_select = matches.is_present("select");
    let address = if display_select {
        select_address()
    } else {
        matches.value_of("address").unwrap_or("127.0.0.1").to_string()
    };
    let port = matches.value_of("port").unwrap_or("3000");
    let path = matches.value_of("path").unwrap_or(".").to_string();

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
