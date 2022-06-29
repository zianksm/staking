use actix_web::{
    delete, get, middleware::Logger, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use color_eyre::Result;
use web3::transports::Http;
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
            .service(balances)
            .service(locked_balance)
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
    let total_supply = contract_handlers::OurContract::total_supply().await;

    HttpResponse::Ok().json(total_supply.to_string())
}

#[get("/balances/{address}")]
async fn balances(address: web::Path<Address>) -> impl Responder {
    let balances = contract_handlers::OurContract::balances(Address::from(address.to_fixed_bytes())).await;

    HttpResponse::Ok().json(balances.to_string())
}

#[get("/locked_balance/{address}")]
async fn locked_balance(address: web::Path<Address>) -> impl Responder {
    let locked_balance = contract_handlers::OurContract::locked_balance(Address::from(address.to_fixed_bytes())).await;

    HttpResponse::Ok().json(locked_balance)
}
