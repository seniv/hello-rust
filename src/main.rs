extern crate reqwest;

fn main() -> Result<(), Box<std::error::Error>> {
    let resp: String = reqwest::get("https://ifconfig.me/ip")?
        .text()?;

    println!("Good day, sir! \nYour IP address: {}", resp);
    Ok(())
}
