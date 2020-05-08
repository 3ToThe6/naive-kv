use std::str::FromStr;

use reqwest::{blocking::Client, Url};

use common::*;

fn main() {
    let mut url = Url::from_str(&format!("http://{}/", server_address().as_ref())).unwrap();
    url.set_path("/stop");
    Client::new().post(url).send().unwrap();
}
