use crate::server_config::Config;
use r2d2_mysql::MysqlConnectionManager;
use mysql::{Opts, OptsBuilder};
use r2d2::{self, PooledConnection};
use std::sync::Arc;



pub fn get_connection() -> PooledConnection<MysqlConnectionManager>{
    let config = Config::from_env().expect("loading server configurations");
    let opts = Opts::from_url(&config.db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(10).build(manager).unwrap());

    let pooled = pool.clone();
    let con = pooled.get().unwrap();

    return con
}