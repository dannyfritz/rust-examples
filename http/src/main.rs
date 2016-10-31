use std::io::Read;

extern crate serde;
extern crate serde_json;
use serde_json::Map;

extern crate hyper;
use hyper::Client;
use hyper::header::{ Headers, UserAgent };

fn main() {
    let url = "https://api.github.com/users/donaldpipowitch/repos";
    let mut headers = Headers::new();
    headers.set(UserAgent("Mercateo/rust-for-node-developers".to_string()));
    let client = Client::new();
    let mut res = client.get(url)
        .headers(headers)
        .send()
        .expect("Couldn't send request.");
    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Couldn't read response.");
    
    if res.status.is_client_error() {
        panic!("Got client error: {}", res.status);
    }
    if res.status.is_server_error() {
        panic!("Got server error: {}", res.status);
    }
    
    let repositories: Map<String, String> = serde_json::from_str(&buf).expect("Couldn't parse response.");
    println!("Result is:\n{:?}", repositories);
}
