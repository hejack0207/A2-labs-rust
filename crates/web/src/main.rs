extern crate uesim;

use axum::{
    routing::get,
    routing::post,
    Router,
    http::StatusCode,
};

use std::mem;

use axum::{
    extract,
};

// use serde::Deserialize;


// #[derive(Deserialize)]
// struct IfInfo {
//     ifindex: u16,
//     ifname: String,
// }

use uesim::{
    ue_sim_api::{
        on_model_init,
    },
    model_config,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let _ = app.clone().route("/model/init", get(web_model_init));
    let _ = app.clone().route("/model/config", get(web_model_config));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn web_model_init(extract::Json(payload): extract::Json<model_init_request >)->Result<String, StatusCode>{
    unsafe {
        on_model_init(payload);
    }
    Ok("".to_string())
}

async fn web_model_config()->Result<String, StatusCode>{
    Ok(model_config())
}


