use axum::Router;
use axum::routing::{get, post};
use crate::controller::login_handler;
use crate::controller::get_info_handler;

mod model;
mod controller;


#[tokio::main]
async fn main () {

    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/info", get(get_info_handler));

    let listner = 
    tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Listening on 0.0.0.0:3000");
    
    axum::serve(listner, app)
        .await
        .unwrap();
}