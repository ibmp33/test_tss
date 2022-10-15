use rocket::local::blocking::Client;
use rocket::mtls::Certificate;
use rocket::config::MutualTls;

use std::fs::File;
use std::io::Read;

// let tls_config = MutualTls::from_path("")

// let cert = {

// }




#[test]
fn hello_world() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}
