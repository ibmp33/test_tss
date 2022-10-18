use std::fs::File;
use std::io::Read;
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};

// fn load_cert() -> reqwest::Result<reqwest::Certificate> {
//     let mut buf = Vec::new();
//     File::open("private/ca_cert.pem")
//         .unwrap()
//         .read_to_end(&mut buf)
//         .unwrap();
//     reqwest::Certificate::from_pem(&buf)
// }

// fn pkcs12() -> reqwest::Result<reqwest::Identity>{
//     let mut buf = Vec::new();
//     File::open("private/client_rsa_sha256.p12").unwrap().read_to_end(&mut buf).unwrap();
//     reqwest::Identity::from_pkcs12_der(&buf,"rocket")
// }
fn load_cert() -> reqwest::Result<reqwest::Certificate> {
    let mut buf = Vec::new();
    File::open("private/ca_cert.der")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    reqwest::Certificate::from_der(&buf)
}
fn pem() -> reqwest::Result<reqwest::tls::Identity>{
    let mut buf = Vec::new();
    File::open("private/client.pem")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    reqwest::tls::Identity::from_pem(&buf)
}

fn new_client_with_headers() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("Content-Type:application/json; charset=utf-8"),
    );
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    let cert = load_cert().unwrap();
    let identity = pem().unwrap();
    Client::builder()
        .use_rustls_tls()
        .default_headers(headers)
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .identity(identity)
        .build()
        .unwrap()
}

pub async fn postb<T>(client: &Client, addr: &str, path: &str, body: T) -> Option<String>
where
    T: serde::ser::Serialize,
{
    let url = format!("{}/{}", addr, path);
    let retries = 3;
    for _i in 1..retries {
        let res = client
            .post(url.clone())
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&body)
            .send()
            .await;

        if let Ok(res) = res {
            return Some(res.text().await.unwrap());
        }
    }
    None
}




#[tokio::main]
async fn main() {
    let client = new_client_with_headers();
    let addr = "https://127.0.0.1:8000/set";
    let entry = "";
    let res_body = postb(&client, addr, "/", entry).await;
}


















