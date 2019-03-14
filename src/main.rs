extern crate reqwest;

use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;

    let iterable = resp["origin"].split(",");
    let collection: Vec<_> = iterable.collect();
    println!("Hello! \nYour IP address: {}", collection[0]);
    Ok(())
}