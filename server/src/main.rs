#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;
// use rocket::config::{Config, TlsConfig, MutualTls};
// use rocket::mtls::Certificate;

// #[get("/")]
// fn mutual(cert: Certificate<'_>) -> String {
//     format!("Hello! Here's what we know: [{}] {}", cert.serial(), cert.subject())
// }

#[get("/", rank = 2)]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/set")]
fn set() {
    let entry=0;
}

#[launch]
fn rocket() -> _ {
    // See `Rocket.toml` and `Cargo.toml` for TLS configuration.
    // Run `./private/gen_certs.sh` to generate a CA and key pairs.
    // let tls_config = TlsConfig::from_paths("/Users/password/Rocket/examples/tls/private/rsa_sha256_cert.pem", "/Users/password/Rocket/examples/tls/private/rsa_sha256_key.pem")
    // .with_mutual(MutualTls::from_path("/Users/password/Rocket/examples/tls/private/ca_cert.pem"));
    // let config = Config {
    //     tls: Some(tls_config),
    //      ..Default::default()};
    // rocket::custom(config);



    rocket::build().mount("/", routes![hello, set])
}
