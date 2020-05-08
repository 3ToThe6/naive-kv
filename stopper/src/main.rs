use reqwest::blocking::Client;

use common::*;

fn main() {
    let mut url = server_url();
    url.set_path("/stop");
    Client::new().post(url).send().unwrap();
}
