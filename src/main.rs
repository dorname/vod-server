use actix_files as fs;
use actix_web::{web, App, HttpServer};
use config::Config;
use error::Result;

mod handlers;
mod services;
mod config;
mod error;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new();
    
    // 创建必要的目录
    std::fs::create_dir_all(&config.upload_dir)
        .map_err(|e| error::AppError::Io(e))?;
    std::fs::create_dir_all(&config.static_dir)
        .map_err(|e| error::AppError::Io(e))?;

    // 在闭包外获取必要的值
    let host = config.host.clone();
    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(fs::Files::new("/static", &config.static_dir).show_files_listing())
            .service(fs::Files::new("/videos", &config.upload_dir).show_files_listing())
            .configure(handlers::upload::config)
            .configure(handlers::play::config)
    })
    .bind((host, port))
    .map_err(|e| error::AppError::Io(e.into()))?
    .run()
    .await
    .map_err(|e| error::AppError::Io(e.into()))
}