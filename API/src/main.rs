use actix_web::{
    delete, get, middleware::Logger, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use color_eyre::Result;
use crate::server_config::Config;
use tracing::info;
mod server_config;
mod contract_handlers;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};



#[actix_web::main]
async fn main() -> Result<()> {
    info!("loading config into server...");
    let config = Config::from_env().expect("loading server configurations");
    info!("server config loaded successfully.");

    info!(
        "starting server at  host : {}, port : {}",
        config.host, config.port
    );
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(total_supply)
            .route("/", web::get().to(home))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    Ok(())
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home page".to_string())
}

#[get("/total_supply")]
async fn total_supply() -> impl Responder {
    let total_supply = contract_handlers::OurContract::total_supply();

    HttpResponse::Ok().json(total_supply)
}
