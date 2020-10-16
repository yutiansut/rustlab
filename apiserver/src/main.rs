mod config;
mod db;
mod urls;
mod handlers;
mod models;

#[macro_use]
extern crate serde_derive;

extern crate env_logger;
extern crate log;

extern crate failure;
extern crate toml;
use failure::Error;

use std::env;
use std::fs;
use std::io;
use std::process::exit;

use std::cell::Cell;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use actix_web::{middleware, get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::ServiceConfig;
//actix web 使用文档： https://www.rectcircle.cn/posts/rust-actix/

#[actix_web::main]
async fn main() -> io::Result<()> {
    //解析配置文件
    let config_string = fs::read_to_string(config::CONFIG_FILENAME)?;
    let config: config::Config = toml::from_str(&config_string)?;
    println!("{:#?}", config);

    //设置日志
    env::set_var(
        "RUST_LOG",
        "diesel=debug,actix=debug,actix_web=debug,apiserver=info",
    );
    env_logger::init();

    //创建数据库连接池
    let pool = db::create_db_pool(&config.db.host);

    //启动http server
    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);

    let serverConfig = ServiceConfig::new();

    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(url_config)

    })
    .bind(&bind)?
    .run()
    .await
}

