#![recursion_limit = "2056"]
use actix_web::{web, App, HttpServer};
use batch_sync::background_sync;
use dotenv::dotenv;
use middlewares::auth_middleware::Authentication;
use std::env;
use templates::grpc_controller::grpc_controller_generator;
use templates::proto_generator;
use templates::table_enum::table_enum_generator;
mod batch_sync;
mod controllers;
mod db;
mod middlewares;
mod models;
mod schema;
mod shutdown_handler;
mod structs;
mod sync;
mod table_enum;
mod templates;
mod utils;
mod generated;
use crate::batch_sync::BatchSyncService;
use crate::middlewares::shutdown_middleware::ShutdownGuard;
use crate::sync::controllers::sync_endpoints_controller;
use crate::sync::merkles::merkle_manager::MerkleManager;
use crate::sync::message_manager::{create_message_channel, SENDER};
use crate::sync::sync_service::bg_sync;
use crate::sync::transactions::queue_service::QueueService;
use crate::sync::transactions::transaction_service::TransactionService;
use controllers::grpc_controller::GrpcController;
use controllers::store_controller::{
    batch_delete_records, batch_insert_records, batch_update_records, create_record, delete_record,
    update_record, upsert,
};
use env_logger::Env;
use std::sync::Arc;
use std::process;

fn run_build_script() -> std::io::Result<()> {
    use std::process::Command;

    println!("Running build script manually...");

    let output = Command::new("cargo").arg("build").arg("--quiet").output()?;

    if output.status.success() {
        println!("Build script executed successfully");
    } else {
        eprintln!(
            "Build script failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let generate_proto = env::var("GENERATE_PROTO").unwrap_or_else(|_| "false".to_string());
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_module("tokio_postgres", log::LevelFilter::Info)
        .init();
    let merkle_manager = MerkleManager::instance();
    shutdown_handler::setup_shutdown_handler().await;
    // if (generate_proto == "true") {
    println!("Generating proto files");
    // proto_generator::generate_protos("src/schema/schema.rs", "src/proto");
    // run_build_script()?;
    // // Run the generator
    if let Err(e) = grpc_controller_generator::run_generator() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    // if let Err(e) = table_enum_generator::run_generator() {
    //     eprintln!("Failed to generate table enum: {}", e);
    // }
    println!("gRPC controller generation completed successfully!");

    let background_sync_service = match background_sync::BackgroundSyncService::new().await {
        Ok(service) => service,
        Err(e) => {
            log::error!("Failed to initialize BackgroundSyncService: {}", e);
            return Ok(());
        }
    };

    // Spawn it in a background task
    tokio::spawn(async move {
        if let Err(e) = background_sync_service.init().await {
            log::error!("Error in background sync service: {}", e);
        }
    });

    // }
    merkle_manager.load_trees_from_db().await;

    // Initialize the message sender
    let sender = create_message_channel();
    let arc_sender = Arc::new(sender);
    SENDER.set(arc_sender).expect("Failed to initialize sender");

    // Start periodic save task (optional)
    // Save to database every 5 minutes (300000 milliseconds)
    let _save_handle = merkle_manager.start_periodic_save(300000);

    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let grpc_port = env::var("GRPC_PORT").unwrap_or_else(|_| "6000".to_string());
    let grpc_url = env::var("GRPC_URL").unwrap_or_else(|_| "127.0.0.1".to_string());
    let pool = db::establish_async_pool();
    println!("Database connected successfully.");
    TransactionService::initialize().await;

    let grpc_addr = format!("{}:{}", grpc_url, grpc_port);

    //init batch sync
    if let Err(e) = BatchSyncService::init().await {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        log::info!("Queue initialized successfully");
    }

    if let Err(e) = QueueService::init().await {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        println!("Queue initialized successfully");
    }

    let server_url = format!("0.0.0.0:{}", port);
    println!("Server is running on {}", server_url);
    // ! not using async generator
    tokio::spawn(async {
        if let Err(e) = bg_sync().await {
            log::error!("Error starting background sync: {}", e);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(sync_endpoints_controller::configure)
            .service(
                web::scope("/api/store")
                    .wrap(Authentication)
                    .wrap(ShutdownGuard)
                    .route("/{table}", web::post().to(create_record))
                    .route("/upsert/{table}", web::post().to(upsert))
                    .route("/batch/{table}", web::patch().to(batch_update_records))
                    .route("/batch/{table}", web::delete().to(batch_delete_records))
                    .route("/{table}/{id}", web::patch().to(update_record))
                    .route("/{table}/{id}", web::delete().to(delete_record))
                    .route("/batch/{table}", web::post().to(batch_insert_records)),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
