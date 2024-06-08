#[macro_use] extern crate rocket;

// Import IpAddr
use std::net::{IpAddr, Ipv4Addr};

#[get("/<name>/<age>")]
fn hello_path(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // Configure rocket to listen on specified host and port

    rocket::build()
    .mount("/", routes![index])
    .mount("/hello", routes![hello_path])
    .configure(rocket::config::Config {
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        // Alternatively, you can use the following line to specify the address
        // address: "127.0.0.1".parse().unwrap(),
        port: 8888,
        ..Default::default()
    })
}