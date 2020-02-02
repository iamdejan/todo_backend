use actix_web::{App, HttpServer};
use std::io::Result;
use mongodb::{Client, options::ClientOptions};

extern crate serde;

#[macro_use]
extern crate serde_derive;

mod routes;
mod model;

fn initiate_mongodb(db_url: &str) -> Client {
    let mut client_options: ClientOptions = ClientOptions::parse(db_url).unwrap();
    client_options.app_name = Some("todo_backend".to_string());
    let client = Client::with_options(client_options).unwrap();
    return client;
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let bind_address = "127.0.0.1:8080";

    let client = initiate_mongodb("mongodb://localhost:27017");
    HttpServer::new(move || {
        println!("Starting Actix-Web server...");
        return App::new().data(client.clone()).configure(routes::routes::register_routes);
    }).bind(bind_address)
      .expect("Fail to run server!")
      .run()
      .await
}