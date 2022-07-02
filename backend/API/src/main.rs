use crate::server_config::Config;
use actix_web::{
    delete, get, middleware::Logger, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use color_eyre::Result;
use tracing::info;
use web3::transports::Http;
mod contract_handlers;
mod server_config;
use actix_cors::Cors;
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
        let cors = Cors::default().allow_any_origin().allow_any_method();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(total_supply)
            .service(balances)
            .service(stakes)
            .service(get_contract_info)
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
    let balances =
        contract_handlers::OurContract::balances(Address::from(address.to_fixed_bytes())).await;

    HttpResponse::Ok().json(balances)
}

#[get("/contract")]
async fn get_contract_info() -> impl Responder {
    let info: contract_handlers::OurContract =
        contract_handlers::OurContract::get_contract_info().await;

    HttpResponse::Ok().json(info)
}

/*#[post("/stake")]
async fn stake(user_info: web::Json<contract_handlers::StakeRequest> )-> impl Responder{
    let stake = contract_handlers::OurContract::stake(user_info.private_key.clone(),  user_info.amount_to_stake).await;

    HttpResponse::Ok()
}*/

#[get("/stakes/{address}")]
async fn stakes(address: web::Path<Address>) -> impl Responder {
    let stakes =
        contract_handlers::OurContract::get_stakes(Address::from(address.to_fixed_bytes())).await;

    HttpResponse::Ok().json(stakes)
}
