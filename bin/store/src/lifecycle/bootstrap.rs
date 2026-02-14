use crate::utils::helpers::parse_command_args;
use crate::{
    controllers::{
        organization_controller::OrganizationsController,
        pg_functions::pg_listener_controller::{
            create_pg_function, pg_listener_delete, pg_listener_get, test_pg_function_syntax,
        },
        root_controller::{
            root_aggregation_filter, root_batch_delete_records, root_batch_insert_records,
            root_batch_update_records, root_count_by_filter, root_create_record,
            root_delete_record, root_get_by_filter, root_get_by_id, root_search_suggestions,
            root_switch_account, root_update_record, root_upsert,
        },
        store_controller::{
            aggregation_filter, batch_delete_records, batch_insert_records, batch_update_records,
            count_by_filter, create_record, delete_record, download_file_by_id, get_by_filter,
            get_by_id, get_file_by_id, search_suggestions, switch_account, update_record,
            upload_file, upsert,
        },
    },
    database::{
        db,
        schema::{self, database_setup::DatabaseSetupFlags},
    },
    generated::grpc_controller::GrpcController,
    initializers::system_initialization::{init::initialize, structs::EInitializer},
    middlewares::{
        auth_middleware::Authentication, session_middleware::SessionMiddleware,
        shutdown_middleware::ShutdownGuard,
    },
    providers::{
        self,
        operations::{
            batch_sync::{background_sync, batch_sync::BatchSyncService},
            message_stream::{
                gateway::{create_socket_io, set_streaming_service},
                pg_listener_service::PgListenerService,
                streaming_service::MessageStreamingService,
            },
            sync::{
                message_manager::{create_message_channel, SENDER},
                sync_service::bg_sync,
                transactions::{
                    queue_service::QueueService, transaction_service::TransactionService,
                },
            },
        },
        storage::{
            cache::{cache, cache_factory::CacheType, CacheConfig},
            AppState,
        },
    },
    routers::sync_router::configure_sync_routes,
};
use actix_web::{mime, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::{env, sync::Arc, time::Duration};
use tokio::signal::unix::{signal, SignalKind};

pub async fn exec() -> std::io::Result<()> {
    dotenv().ok();

    let (s3_client, bucket_name) = match providers::storage::initialize().await {
        Ok((client, bucket)) => (client, bucket),
        Err(e) => {
            log::error!("Failed to initialize S3 client: {}", e);
            std::process::exit(1);
        }
    };

    let command_args = parse_command_args();
    let generate_proto = command_args.generate_proto;
    let generate_grpc = command_args.generate_grpc;
    let generate_table_enum = command_args.generate_table_enum;
    let create_schema = command_args.create_schema;

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_module("tokio_postgres", log::LevelFilter::Info)
        .init();
    let cache_type_str = env::var("CACHE_TYPE").unwrap_or_else(|_| "inmemory".to_string());
    let cache_type = match cache_type_str.as_str() {
        "redis" => CacheType::Redis,
        _ => CacheType::InMemory,
    };
    let redis_connection = env::var("REDIS_CONNECTION").ok();
    let ttl = env::var("CACHE_TTL")
        .ok()
        .and_then(|ttl_str| ttl_str.parse::<u64>().ok())
        .map(Duration::from_secs);

    CacheConfig::init(cache_type, redis_connection, ttl);
    log::info!(
        "Initialized cache with type: {:?}, TTL: {:?}",
        cache_type,
        ttl
    );

    let _ = cache.cache_type();

    let cleanup = command_args.cleanup;
    let init_db = command_args.init_db;
    if cleanup {
        info!("Running cleanup operation only...");
        match schema::database_setup::setup_database(DatabaseSetupFlags {
            run_cleanup: true,
            run_migrations: true,
            initialize_services: false,
            run_init_sql: false,
        })
        .await
        {
            Ok(_) => {
                info!("Database cleanup completed successfully!");
            }
            Err(e) => {
                error!("Error during database cleanup: {}", e);
            }
        }
    }
    if let Err(e) = initialize(EInitializer::BACKGROUND_SERVICES_CONFIG, None).await {
        log::error!("Failed to initialize background services: {}", e);
    } else {
        log::info!("Background services initialized successfully");
    }

    TransactionService::initialize().await;

    if generate_proto || generate_grpc || generate_table_enum || create_schema {
        crate::lifecycle::code_generation::handle_code_generation(&command_args).await;
        unreachable!("handle_code_generation exits on success");
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
    info!("Starting PgListenerService...");
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
    info!("Database connected successfully.");

    // init batch sync
    if let Err(e) = BatchSyncService::init().await {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        log::info!("Queue initialized successfully");
    }

    if let Err(e) = QueueService::init().await {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        info!("Queue initialized successfully");
    }

    if init_db {
        info!("Running cleanup operation only...");
        match schema::database_setup::setup_database(DatabaseSetupFlags {
            run_cleanup: false,
            run_migrations: false,
            initialize_services: true,
            run_init_sql: true,
        })
        .await
        {
            Ok(_) => {
                info!("Database cleanup completed successfully!");
            }
            Err(e) => {
                error!("Error during database cleanup: {}", e);
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
            Ok(_) => info!("gRPC server started successfully"),
            Err(e) => error!("Failed to start gRPC server: {}", e),
        }
    });

    //HTTPS config

    let server_url = format!("0.0.0.0:{}", port);
    info!("Store is running on {}", server_url);
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

        info!("Socket.IO server running on http://0.0.0.0:3001");

        axum::serve(listener, app)
            .await
            .expect("Socket.IO server failed");
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_sync_routes)
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
            .service(web::scope("/api/token").wrap(SessionMiddleware).route(
                "/verify",
                web::post().to(OrganizationsController::verify_token),
            ))
            .service(web::scope("/api/password").wrap(SessionMiddleware).route(
                "/verify",
                web::post().to(OrganizationsController::verify_password),
            ))
            .service(
                web::scope("/api/store/root")
                    .wrap(ShutdownGuard)
                    .wrap(Authentication)
                    .wrap(SessionMiddleware)
                    .route("/aggregate", web::post().to(root_aggregation_filter))
                    .route("/{table}", web::post().to(root_create_record))
                    .route("/upsert/{table}", web::post().to(root_upsert))
                    .route("/batch/{table}", web::patch().to(root_batch_update_records))
                    .route(
                        "/batch/{table}",
                        web::delete().to(root_batch_delete_records),
                    )
                    .route("/{table}/filter", web::post().to(root_get_by_filter))
                    .route("/{table}/count", web::post().to(root_count_by_filter))
                    .route("/{table}/{id}", web::get().to(root_get_by_id))
                    .route("/{table}/{id}", web::patch().to(root_update_record))
                    .route("/{table}/{id}", web::delete().to(root_delete_record))
                    .route("/batch/{table}", web::post().to(root_batch_insert_records))
                    .route("/switch_account", web::post().to(root_switch_account))
                    .route(
                        "/{table}/filter/suggestions",
                        web::post().to(root_search_suggestions),
                    ),
            )
            .service(
                web::scope("/api/store")
                    .app_data(web::Data::new(AppState {
                        s3_client: s3_client.clone(),
                        bucket_name: bucket_name.clone(),
                    }))
                    .wrap(ShutdownGuard)
                    .wrap(Authentication)
                    .wrap(SessionMiddleware)
                    .route("/aggregate", web::post().to(aggregation_filter))
                    .route("/{table}", web::post().to(create_record))
                    .route("/upsert/{table}", web::post().to(upsert))
                    .route("/batch/{table}", web::patch().to(batch_update_records))
                    .route("/batch/{table}", web::delete().to(batch_delete_records))
                    .route("/{table}/filter", web::post().to(get_by_filter))
                    .route("/{table}/count", web::post().to(count_by_filter))
                    .route("/{table}/{id}", web::get().to(get_by_id))
                    .route("/{table}/{id}", web::patch().to(update_record))
                    .route("/{table}/{id}", web::delete().to(delete_record))
                    .route("/batch/{table}", web::post().to(batch_insert_records))
                    .route("/switch_account", web::post().to(switch_account))
                    .route(
                        "/{table}/filter/suggestions",
                        web::post().to(search_suggestions),
                    ),
            )
            .service(
                web::scope("/api/listener")
                    .wrap(ShutdownGuard)
                    .wrap(Authentication)
                    .wrap(SessionMiddleware)
                    .route("", web::get().to(pg_listener_get))
                    .route("/function", web::post().to(create_pg_function))
                    .route("/test", web::post().to(test_pg_function_syntax))
                    .route("/{function_name}", web::delete().to(pg_listener_delete)),
            )
            .service(
                web::scope("/api/file")
                    .app_data(web::Data::new(providers::storage::AppState {
                        s3_client: s3_client.clone(),
                        bucket_name: bucket_name.clone(),
                    }))
                    .wrap(ShutdownGuard)
                    .wrap(Authentication)
                    .wrap(SessionMiddleware)
                    .route("/{id}", web::get().to(get_file_by_id))
                    .route("/{id}/download", web::get().to(download_file_by_id))
                    .app_data(
                        web::JsonConfig::default()
                            .limit(1024 * 1024 * 10) // 10MB JSON payload limit
                            .content_type(|mime| mime == mime::APPLICATION_JSON),
                    )
                    .app_data(
                        web::FormConfig::default().limit(1024 * 1024 * 100), // 100MB form payload limit
                    )
                    .route("/upload", web::post().to(upload_file)),
            )
    })
    .disable_signals()
    .bind(server_url)?
    .run();

    let mut sigint = signal(SignalKind::interrupt())?;

    tokio::select! {
        _ = server => {},
        _ = sigint.recv() => {
            info!("SIGINT received, running custom shutdown...");

            // // Set the shutdown flag
            // request_shutdown();

            // // Perform your async cleanup operations
            // if let Err(e) = shutdown_handler::save_data_before_shutdown().await {
            //     log::error!("Error during shutdown process: {}", e);
            // } else {
            //     log::info!("Successfully saved all data before shutdown");
            // }

            // Wait for 5 seconds before proceeding with shutdown
            info!("Waiting 5 seconds before final shutdown...");
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            info!("Shutdown delay complete, exiting now");
        },
    }

    Ok(())
}
