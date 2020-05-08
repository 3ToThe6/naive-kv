use url::Url;

pub mod req;

pub fn server_host_and_port() -> impl AsRef<str> {
    "127.0.0.1:80"
}

pub fn server_url() -> Url {
    "http://127.0.0.1:80/".parse().unwrap()
}
