use reqwest::blocking::Client;

use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();
    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd);
    println!("{:?}", response);
    Ok(())
}
