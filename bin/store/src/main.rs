#![recursion_limit = "2056"]
use actix_web::{web, App, HttpServer};
use batch_sync::background_sync;
use dotenv::dotenv;
use message_stream::gateway::{create_socket_io, set_streaming_service};
use middlewares::auth_middleware::Authentication;
use std::env;
use templates::grpc_controller::grpc_controller_generator;
use templates::proto_generator;
use templates::table_enum::table_enum_generator;
mod auth;
mod batch_sync;
mod cache;

mod controllers;
mod db;
mod generated;
mod initializers;
mod message_stream;
mod middlewares;
mod models;
mod organizations;
mod permissions;
mod providers;
mod schema;
mod shutdown_handler;
mod structs;
mod sync;
mod table_enum;
mod templates;

mod utils;
use crate::batch_sync::BatchSyncService;
use crate::cache::cache_factory::CacheType;
use crate::cache::{cache, CacheConfig}; // Add the cache function import
use crate::controllers::store_controller::get_by_id;
use crate::initializers::init::initialize;
use crate::initializers::structs::EInitializer;
use crate::message_stream::pg_listener_service::PgListenerService;
use crate::message_stream::streaming_service::MessageStreamingService;
use crate::middlewares::session_middleware::SessionMiddleware;
use crate::middlewares::shutdown_middleware::ShutdownGuard;
use crate::organizations::organization_controller::OrganizationsController;
use crate::schema::database_setup::DatabaseSetupFlags;
use crate::sync::controllers::sync_endpoints_controller;
// use crate::sync::merkles::merkle_manager::MerkleManager;
use crate::sync::message_manager::{create_message_channel, SENDER};
use crate::sync::sync_service::bg_sync;
use crate::sync::transactions::queue_service::QueueService;
use crate::sync::transactions::transaction_service::TransactionService;

use controllers::grpc_controller::GrpcController;
use controllers::pg_functions::pg_listener_controller::{
    create_pg_function, pg_listener_delete, pg_listener_get, test_pg_function_syntax,
};
use controllers::store_controller::{
    batch_delete_records, batch_insert_records, batch_update_records, create_record, delete_record,
    get_by_filter, aggregation_filter, update_record, upsert,
};
use controllers::root_controller::{
    root_aggregation_filter, root_batch_delete_records, root_batch_insert_records,
    root_batch_update_records, root_create_record, root_delete_record, root_get_by_filter,
    root_get_by_id, root_update_record, root_upsert,
};
use env_logger::Env;
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal::unix::{signal, SignalKind};

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
    dotenv().ok();
    let generate_proto =
        env::var("GENERATE_PROTO").unwrap_or_else(|_| "false".to_string()) == "true";
    let generate_grpc = env::var("GENERATE_GRPC").unwrap_or_else(|_| "false".to_string()) == "true";
    let generate_table_enum =
        env::var("GENERATE_TABLE_ENUM").unwrap_or_else(|_| "false".to_string()) == "true";
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_module("tokio_postgres", log::LevelFilter::Info)
        .init();
    let cache_type_str = env::var("CACHE_TYPE").unwrap_or_else(|_| "inmemory".to_string());
    let cache_type = CacheType::from_str(&cache_type_str).unwrap_or(CacheType::InMemory);
    let redis_connection = env::var("REDIS_CONNECTION").ok();
    let ttl = env::var("CACHE_TTL")
        .ok()
        .and_then(|ttl_str| ttl_str.parse::<u64>().ok())
        .map(Duration::from_secs);
    let args: Vec<String> = env::args().collect();

    CacheConfig::init(cache_type, redis_connection, ttl);
    log::info!(
        "Initialized cache with type: {:?}, TTL: {:?}",
        cache_type,
        ttl
    );

    let _ = cache.cache_type();

    // Set boolean flags based on command-line arguments
    let cleanup = args.contains(&"--cleanup".to_string());
    let init_db = args.contains(&"--init-db".to_string());
    if cleanup {
        println!("Running cleanup operation only...");
        match schema::database_setup::setup_database(DatabaseSetupFlags {
            run_cleanup: true,
            run_migrations: true,
            initialize_services: false,
            run_init_sql: false,
        })
        .await
        {
            Ok(_) => {
                println!("Database cleanup completed successfully!");
            }
            Err(e) => {
                eprintln!("Error during database cleanup: {}", e);
            }
        }
    }
    if let Err(e) = initialize(EInitializer::BACKGROUND_SERVICES_CONFIG, None).await {
        log::error!("Failed to initialize background services: {}", e);
    } else {
        log::info!("Background services initialized successfully");
    }

    TransactionService::initialize().await;

    if generate_proto || generate_grpc || generate_table_enum {
        println!("Starting code generation...");

        // Proto generation
        if generate_proto {
            println!("Generating proto files");
            proto_generator::generate_protos("src/schema/schema.rs", "src/proto");

            if let Err(e) = run_build_script() {
                eprintln!("Failed to run build script: {}", e);
            }
        }

        // gRPC controller generation
        if generate_grpc {
            println!("Generating gRPC controllers");
            if let Err(e) = grpc_controller_generator::run_generator() {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }

        // Table enum generation
        if generate_table_enum {
            println!("Generating table enums");
            if let Err(e) = table_enum_generator::run_generator() {
                eprintln!("Failed to generate table enum: {}", e);
            }
        }

        println!("Code generation completed successfully!");
    }

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

    //Pg listener service
    println!("Starting PgListenerService...");
    if let Err(e) = PgListenerService::initialize().await {
        log::error!("Failed to initialize PgListenerService: {}", e);
    } else {
        log::info!("PgListenerService initialized successfully");
    }

    // Initialize the message sender
    let sender = create_message_channel();
    let arc_sender = Arc::new(sender);
    SENDER.set(arc_sender).expect("Failed to initialize sender");

    let pool = db::establish_async_pool();
    println!("Database connected successfully.");

    // init batch sync
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

    if init_db {
        println!("Running cleanup operation only...");
        match schema::database_setup::setup_database(DatabaseSetupFlags {
            run_cleanup: false,
            run_migrations: false,
            initialize_services: true,
            run_init_sql: true,
        })
        .await
        {
            Ok(_) => {
                println!("Database cleanup completed successfully!");
            }
            Err(e) => {
                eprintln!("Error during database cleanup: {}", e);
            }
        }
    }

    //GRPC config

    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let grpc_port = env::var("GRPC_PORT").unwrap_or_else(|_| "6000".to_string());
    let grpc_url = env::var("GRPC_URL").unwrap_or_else(|_| "127.0.0.1".to_string());
    let grpc_addr = format!("{}:{}", grpc_url, grpc_port);

    tokio::spawn(async move {
        match GrpcController::init(&grpc_addr).await {
            Ok(_) => println!("gRPC server started successfully"),
            Err(e) => eprintln!("Failed to start gRPC server: {}", e),
        }
    });

    //HTTPS config

    let server_url = format!("0.0.0.0:{}", port);
    println!("Store is running on {}", server_url);
    tokio::spawn(async {
        if let Err(e) = bg_sync().await {
            log::error!("Error starting background sync: {}", e);
        }
    });

    //Socket server config

    tokio::spawn(async move {
        use axum::Router;

        // Use your gateway function that includes all the handlers
        let (layer, io) = create_socket_io();

        // Initialize the MessageStreamingService
         let streaming_service = MessageStreamingService::new(io);
         
         // Set the streaming service reference in gateway
         set_streaming_service(streaming_service.clone());
         
         // Initialize the streaming service (starts broker and routing)
         if let Err(e) = streaming_service.initialize().await {
             log::error!("Failed to initialize MessageStreamingService: {}", e);
         } else {
             log::info!("MessageStreamingService initialized successfully");
         }



         // Note: Message processing is handled by the routing task in initialize()
         // No need for additional message processing loop



        let app = Router::new().layer(layer);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
            .await
            .expect("Failed to bind Socket.IO server to port 3001");

        println!("Socket.IO server running on http://0.0.0.0:3001");

        axum::serve(listener, app)
            .await
            .expect("Socket.IO server failed");
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(sync_endpoints_controller::configure)
            .service(
                web::scope("/api/organizations")
                    .wrap(SessionMiddleware)
                    .route(
                        "/register",
                        web::post().to(OrganizationsController::register),
                    )
                    .route(
                        "/register/{id}",
                        web::put().to(OrganizationsController::reregister_existing_account),
                    )
                    .route("/auth", web::post().to(OrganizationsController::auth))
                    .route("/logout", web::post().to(OrganizationsController::logout)),
            )
            .service(
                web::scope("/api/store")
                    .wrap(ShutdownGuard)
                    .wrap(Authentication)
                    .wrap(SessionMiddleware)
                    .route("/aggregate", web::post().to(aggregation_filter))
                    .route("/{table}", web::post().to(create_record))
                    .route("/upsert/{table}", web::post().to(upsert))
                    .route("/batch/{table}", web::patch().to(batch_update_records))
                    .route("/batch/{table}", web::delete().to(batch_delete_records))
                    .route("/{table}/filter", web::post().to(get_by_filter))
                    .route("/{table}/{id}", web::get().to(get_by_id))
                    .route("/{table}/{id}", web::patch().to(update_record))
                    .route("/{table}/{id}", web::delete().to(delete_record))
                    .route("/batch/{table}", web::post().to(batch_insert_records)),
            )
            .service(
                web::scope("/api/store/{type}")
                    .wrap(ShutdownGuard)
                    .wrap(Authentication)
                    .wrap(SessionMiddleware)
                    .route("/aggregate", web::post().to(root_aggregation_filter))
                    .route("/{table}", web::post().to(root_create_record))
                    .route("/upsert/{table}", web::post().to(root_upsert))
                    .route("/batch/{table}", web::patch().to(root_batch_update_records))
                    .route("/batch/{table}", web::delete().to(root_batch_delete_records))
                    .route("/{table}/filter", web::post().to(root_get_by_filter))
                    .route("/{table}/{id}", web::get().to(root_get_by_id))
                    .route("/{table}/{id}", web::patch().to(root_update_record))
                    .route("/{table}/{id}", web::delete().to(root_delete_record))
                    .route("/batch/{table}", web::post().to(root_batch_insert_records)),
            )
            .service(
                web::scope("/api/listener")
                    .wrap(Authentication)
                    .wrap(SessionMiddleware)
                    .route("", web::get().to(pg_listener_get))
                    .route("/function", web::post().to(create_pg_function))
                    .route("/test", web::post().to(test_pg_function_syntax))
                    .route("/{function_name}", web::delete().to(pg_listener_delete)),
            )
    })
    .disable_signals()
    .bind(server_url)?
    .run();

    let mut sigint = signal(SignalKind::interrupt())?;

    tokio::select! {
        _ = server => {},
        _ = sigint.recv() => {
            println!("SIGINT received, running custom shutdown...");

            // Set the shutdown flag
            shutdown_handler::request_shutdown();

            // Perform your async cleanup operations
            if let Err(e) = shutdown_handler::save_data_before_shutdown().await {
                log::error!("Error during shutdown process: {}", e);
            } else {
                log::info!("Successfully saved all data before shutdown");
            }

            // Wait for 5 seconds before proceeding with shutdown
            println!("Waiting 5 seconds before final shutdown...");
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            println!("Shutdown delay complete, exiting now");
        },
    }

    Ok(())
}
