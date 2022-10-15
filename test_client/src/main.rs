use std::fs::File;
use std::io::Read;
use reqwest::{Body, Client, Certificate};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};


// fn load_cert() -> reqwest::Result<reqwest::Certificate> {
//     // let mut buf = Vec::new();
//     // File::open("private/ca_cert.pem")
//     //     .unwrap()
//     //     .read_to_end(&mut buf)
//     //     .unwrap();
//     let cert = std::fs::read("private/ca_cert.pem").unwrap();
//     reqwest::Certificate::from_der(&cert)
// }


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
    let cert = std::fs::read("private/ca_cert.pem").unwrap();
    let cert = reqwest::Certificate::from_pem(&cert).unwrap();
    Client::builder().danger_accept_invalid_certs(true)
        .use_native_tls()
        .add_root_certificate(cert)
        .default_headers(headers)
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
    let addr = "https://127.0.0.1:8000";
    let entry = "";
    let res_body = postb(&client, addr, "/", entry).await;
}
