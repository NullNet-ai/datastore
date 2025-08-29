use crate::constants::paths;
use crate::config::core::EnvConfig;
use log::{info, warn};
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[allow(warnings)]
pub fn run_sql_files(cleanup: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Get database connection info from environment variables
    let config = EnvConfig::default();
    let user = config.postgres_user;
    let password = config.postgres_password;
    let dbname = config.postgres_db;
    let host = config.postgres_host;
    let port = config.postgres_port;

    // Get the project directory
    let current_dir = env::current_dir()?.to_string_lossy().to_string();

    // Only run cleanup if the flag is set
    if cleanup {
        info!("Database cleanup requested...");

        // Prompt for password
        print!("Enter password for database cleanup: ");
        io::stdout().flush()?;

        // Read password securely
        let entered_password = rpassword::read_password()?;

        // Define the expected password (you might want to store this in an environment variable)
        let expected_password =
            config.cleanup_password;

        if entered_password == expected_password {
            info!("Password correct. Running database cleanup script...");

            // Run cleanup.sql
            let cleanup_path = Path::new(&current_dir).join(paths::database::CLEANUP_SQL_FILE);
            let cleanup_status = Command::new("psql")
                .args([
                    "-U",
                    &user,
                    "-h",
                    &host,
                    "-p",
                    &port,
                    "-d",
                    &dbname,
                    "-f",
                    cleanup_path.to_str().unwrap(),
                ])
                .env("PGPASSWORD", &password)
                .status()?;

            if !cleanup_status.success() {
                return Err("Database cleanup failed".into());
            }
        } else {
            warn!("Incorrect password. Skipping database cleanup.");
        }
    }

    info!("Running database initialization script...");

    // Run init.sql to initialize the database
    let init_path = Path::new(&current_dir).join(paths::database::INIT_SQL_FILE);
    let init_status = Command::new("psql")
        .args([
            "-U",
            &user,
            "-h",
            &host,
            "-p",
            &port,
            "-d",
            &dbname,
            "-f",
            init_path.to_str().unwrap(),
        ])
        .env("CLEANUP_PASSWORD", &password)
        .current_dir(&current_dir) // Set working directory for relative paths in init.sql
        .status()?;

    if !init_status.success() {
        return Err("Database initialization failed".into());
    }

    info!("Database initialization completed successfully!");
    Ok(())
}
