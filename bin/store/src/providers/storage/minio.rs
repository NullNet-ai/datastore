use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::{Client, Config};
#[derive(Clone)]
pub struct AppState {
    pub s3_client: Client,
    pub bucket_name: String, // Store the default bucket name here
}

pub async fn initialize() -> std::io::Result<(Client, String)> {
    // Check if storage is disabled
    let disable_storage = std::env::var("DISABLE_STORAGE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    if disable_storage {
        println!("🚫 Storage is DISABLED via DISABLE_STORAGE environment variable");
        println!("   Creating mock S3 client for testing/development purposes");

        // Create a minimal mock client configuration for disabled storage
        let mock_credentials =
            Credentials::new("disabled", "disabled", None, None, "mock-provider");

        let mock_config = Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new("us-east-1"))
            .endpoint_url("http://localhost:9000")
            .credentials_provider(mock_credentials)
            .force_path_style(true)
            .build();

        let mock_client = Client::from_conf(mock_config);
        let mock_bucket = "disabled-storage".to_string();

        println!("⚠️  WARNING: All storage operations will be no-ops!");
        return Ok((mock_client, mock_bucket));
    }

    println!("Loading AWS config from environment");

    // Get MinIO/S3 configuration from environment
    let endpoint_url =
        std::env::var("STORAGE_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string());
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let secret_key =
        std::env::var("STORAGE_SECRET_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let region = std::env::var("STORAGE_REGION").unwrap_or_else(|_| "us-east-1".to_string());

    // Option to disable SSL verification (for development/testing)
    let disable_ssl_verification = std::env::var("STORAGE_DISABLE_SSL_VERIFICATION")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    println!("🔧 Configuring S3 client for endpoint: {}", endpoint_url);
    if disable_ssl_verification {
        println!("⚠️  SSL certificate verification is DISABLED - use only for development!");
    }

    // Create credentials
    let credentials = Credentials::new(access_key, secret_key, None, None, "custom-provider");

    // Build S3 client configuration with MinIO-specific settings
    let s3_config = Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(region.clone()))
        .endpoint_url(endpoint_url.clone()) // MinIO endpoint URL
        .credentials_provider(credentials)
        // since MinIO uses path-style addressing, we need to force it
        .force_path_style(true) // Required for MinIO compatibility - uses path-style addressing
        .build();

    let client = Client::from_conf(s3_config);
    println!(
        "✅ S3 client configured successfully for endpoint: {}",
        endpoint_url
    );

    // Print client configuration details including region
    print_client_config(&client);

    // Demonstrate getting region from client
    if let Some(client_region) = get_client_region(&client) {
        println!("\n📍 Region retrieved from client: {}", client_region);
    } else {
        println!("\n⚠️  No region configured in client");
    }

    println!("Reading bucket name from environment");

    let base_bucket_name = std::env::var("STORAGE_BUCKET_NAME").unwrap_or_else(|_| {
        log::info!("STORAGE_BUCKET_NAME not set, using default: store");
        "store".to_string()
    });

    let bucket_name = base_bucket_name.clone();

    println!(
        "S3 client initialization complete with bucket: {}",
        bucket_name
    );

    // Test basic connectivity by listing buckets first
    println!("\nTesting connectivity to MinIO endpoint...");
    let buckets_result = client.list_buckets().send().await;
    let existing_buckets = match buckets_result {
        Ok(output) => {
            println!("✅ Successfully connected to MinIO endpoint");
            let buckets = output.buckets();
            println!("Found {} existing buckets", buckets.len());
            let mut bucket_names = Vec::new();
            for bucket in buckets {
                if let Some(name) = bucket.name() {
                    println!("  - {}", name);
                    bucket_names.push(name.to_string());
                }
            }
            bucket_names
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to MinIO endpoint: {}", endpoint_url);
            eprintln!("Connection error: {}", e);

            if let Some(service_error) = e.as_service_error() {
                eprintln!("Service error code: {:?}", service_error.code());
                eprintln!("Service error message: {:?}", service_error.message());
            }

            eprintln!("💡 Troubleshooting steps:");
            eprintln!("   1. Verify MinIO server is running at: {}", endpoint_url);
            eprintln!(
                "   2. Check network connectivity (try curl {})",
                endpoint_url
            );
            eprintln!(
                "   3. Verify credentials: ACCESS_KEY={}, SECRET_KEY=***",
                std::env::var("STORAGE_ACCESS_KEY").unwrap_or_else(|_| "<not_set>".to_string())
            );
            eprintln!("   4. Check firewall/proxy settings");

            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                format!("Cannot connect to MinIO at {}: {}", endpoint_url, e),
            ));
        }
    };

    // Check if bucket already exists in the list (more reliable than head_bucket for MinIO)
    if existing_buckets.contains(&bucket_name) {
        println!(
            "✅ Bucket '{}' already exists (found in bucket list)",
            bucket_name
        );
        return Ok((client, bucket_name));
    }

    // Check if bucket exists first
    println!("\nChecking if bucket '{}' exists...", bucket_name);
    match client.head_bucket().bucket(&bucket_name).send().await {
        Ok(_) => {
            println!("✅ Bucket '{}' already exists", bucket_name);
            return Ok((client, bucket_name));
        }
        Err(e) => {
            println!(
                "Bucket '{}' does not exist, will attempt to create...",
                bucket_name
            );
            println!("Head bucket error details: {}", e);

            // Log detailed error information for debugging
            if let Some(service_error) = e.as_service_error() {
                println!("Head bucket service error code: {:?}", service_error.code());
                println!(
                    "Head bucket service error message: {:?}",
                    service_error.message()
                );
            }

            // Check for dispatch/connectivity issues
            let error_string = e.to_string();
            if error_string.contains("dispatch failure") || error_string.contains("connection") {
                println!("⚠️  Network connectivity issue detected during bucket check");
                println!("   Endpoint: {}", endpoint_url);
                println!("   This may indicate MinIO server is unreachable");
            }
        }
    }

    // Create bucket using AWS SDK pattern
    println!("   Region: {}", region);

    let create_result = if region == "us-east-1" {
        // us-east-1 doesn't need location constraint
        client.create_bucket().bucket(&bucket_name).send().await
    } else {
        // Other regions need location constraint
        let constraint = aws_sdk_s3::types::BucketLocationConstraint::from(region.as_str());
        let cfg = aws_sdk_s3::types::CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(&bucket_name)
            .send()
            .await
    };

    println!("🪣 Creating bucket '{}'", bucket_name);

    // Handle bucket creation result following AWS SDK pattern
    match create_result {
        Ok(_) => {
            println!("✅ Bucket '{}' created successfully", bucket_name);
        }
        Err(e) => {
            eprintln!("❌ Failed to create bucket '{}'", bucket_name);
            eprintln!("Error details: {}", e);

            // Check if it's a specific AWS error
            if let Some(service_error) = e.as_service_error() {
                eprintln!("Service error code: {:?}", service_error.code());
                eprintln!("Service error message: {:?}", service_error.message());

                // Handle specific error codes following AWS SDK pattern
                if service_error.is_bucket_already_exists()
                    || service_error.is_bucket_already_owned_by_you()
                {
                    eprintln!("💡 Bucket already exists - proceeding with existing bucket");
                    return Ok((client, bucket_name));
                }

                if let Some(code) = service_error.code() {
                    match code {
                        "AccessDenied" => {
                            eprintln!("💡 Access denied - check MinIO user permissions for bucket creation");
                        }
                        "InvalidBucketName" => {
                            eprintln!(
                                "💡 Invalid bucket name '{}' - check naming rules",
                                bucket_name
                            );
                        }
                        _ => {
                            eprintln!("💡 Service error: {}", code);
                        }
                    }
                } else {
                    // No error code - might be a MinIO permission issue
                    eprintln!("⚠️  No specific error code returned - this often indicates:");
                    eprintln!("   1. MinIO user lacks bucket creation permissions");
                    eprintln!("   2. Bucket policy restrictions");
                    eprintln!("   3. MinIO server configuration issues");

                    // Try to verify if bucket actually exists by re-listing
                    eprintln!("\n🔍 Re-checking bucket existence...");
                    if let Ok(list_output) = client.list_buckets().send().await {
                        let current_buckets: Vec<String> = list_output
                            .buckets()
                            .iter()
                            .filter_map(|b| b.name().map(|n| n.to_string()))
                            .collect();

                        if current_buckets.contains(&bucket_name) {
                            eprintln!("✅ Bucket '{}' actually exists! Proceeding...", bucket_name);
                            return Ok((client, bucket_name));
                        }
                    }
                }
            }

            // Additional debugging for MinIO-specific issues
            eprintln!("\n🔍 MinIO Debugging Information:");
            eprintln!(
                "   - Access Key: {}",
                std::env::var("STORAGE_ACCESS_KEY").unwrap_or_else(|_| "<not_set>".to_string())
            );
            eprintln!(
                "   - Secret Key: {}",
                std::env::var("STORAGE_SECRET_KEY").unwrap_or_else(|_| "<not_set>".to_string())
            );
            eprintln!("   - Connectivity: ✅ (list_buckets worked)");
            eprintln!("   - Target bucket: {}", bucket_name);
            eprintln!("   - Region: {}", region);
            eprintln!("   - Endpoint: {}", endpoint_url);

            eprintln!("\n💡 Possible solutions:");
            eprintln!("   1. Use an existing bucket from the list above");
            eprintln!("   2. Check MinIO admin console for user permissions");
            eprintln!("   3. Verify bucket naming conventions (lowercase, no underscores)");
            eprintln!("   4. Check if bucket creation is restricted in MinIO policy");
            eprintln!("   5. Contact MinIO administrator to grant bucket creation permissions");

            // Check for common issues
            let error_string = e.to_string();
            if error_string.contains("dispatch failure") {
                eprintln!("\n⚠️  Network connectivity issue detected");
                eprintln!("   - Check network connectivity to: {}", endpoint_url);
                eprintln!("   - Verify STORAGE_ACCESS_KEY and STORAGE_SECRET_KEY are correct");
                eprintln!(
                    "   - For HTTPS endpoints, try setting STORAGE_DISABLE_SSL_VERIFICATION=true"
                );
                eprintln!("   - Ensure MinIO server is running and accessible");
            }

            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ));
        }
    }

    Ok((client, bucket_name))
}

/// Get the region configured for the S3 client
pub fn get_client_region(client: &Client) -> Option<String> {
    // Access the client's configuration to get the region
    client
        .config()
        .region()
        .map(|region| region.as_ref().to_string())
}

/// Print the current client configuration details
pub fn print_client_config(client: &Client) {
    println!("\n🔧 S3 Client Configuration:");

    // Get region from client config
    if let Some(region) = client.config().region() {
        println!("   Region: {}", region.as_ref());
    } else {
        println!("   Region: Not configured");
    }

    // Note: endpoint_url is not directly accessible from client.config()
    // The endpoint is configured during client creation but not exposed via public API
    println!("   Endpoint: Configured during client creation");

    println!("   Service: Amazon S3");
}

/// Check if storage is disabled via environment variable
pub fn is_storage_disabled() -> bool {
    std::env::var("DISABLE_STORAGE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false)
}

/// Generate a valid bucket name from bucket name and optional organization ID
/// This function creates S3-compatible bucket names following AWS naming conventions
pub fn get_valid_bucket_name(bucket_name: &str, org_id: Option<&str>) -> String {
    // Process organization ID if provided
    let org_pattern = if let Some(org_id) = org_id {
        if org_id.is_empty() {
            String::new()
        } else {
            let org_len = org_id.len();
            let mut pattern = String::new();

            // Get first 2 characters
            if org_len >= 2 {
                pattern.push_str(&org_id[0..2]);
            } else {
                pattern.push_str(org_id);
            }

            // Get middle characters (floor(length/2) - 1 to floor(length/2) + 1)
            if org_len >= 3 {
                let mid_start = (org_len / 2).saturating_sub(1);
                let mid_end = std::cmp::min(mid_start + 2, org_len);
                pattern.push_str(&org_id[mid_start..mid_end]);
            }

            // Get last 2 characters
            if org_len >= 2 {
                pattern.push_str(&org_id[org_len.saturating_sub(2)..]);
            }

            // Remove non-alphabetic characters and convert to lowercase
            pattern
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<String>()
                .to_lowercase()
        }
    } else {
        String::new()
    };

    // Process bucket name
    let bucket_name_processed = bucket_name
        .trim()
        .split_whitespace()
        .filter_map(|word| word.chars().next()) // Get first character of each word
        .collect::<String>()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_lowercase() || *c == '-') // Keep only lowercase letters and hyphens
        .take(20) // Limit to 20 characters
        .collect::<String>();

    // Combine prefix, processed bucket name, and org pattern
    format!("bckt{}{}", bucket_name_processed, org_pattern)
}
