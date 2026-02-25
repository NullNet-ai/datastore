//! Integration tests for counter-service. Require Redis at REDIS_URL (default redis://127.0.0.1:6379).
//! Run with: cargo test --test integration -- --ignored

use counter_service::redis_code;
use deadpool_redis::Config;
use std::time::Duration;

async fn make_pool() -> deadpool_redis::Pool {
    let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    let cfg = Config::from_url(url);
    cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).expect("create pool")
}

fn unique_db() -> String {
    format!("test_db_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos())
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration -- --ignored
async fn get_next_code_fails_when_config_missing() {
    let pool = make_pool().await;
    let db = unique_db();
    let err = redis_code::get_next_code(&pool, &db, "nonexistent_entity").await;
    assert!(err.is_err());
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(err_str.contains("ConfigMissing") || err_str.contains("config missing"));
}

#[tokio::test]
#[ignore]
async fn init_counters_then_get_next_code_success() {
    let pool = make_pool().await;
    let db = unique_db();
    let entity = "test_entity";
    redis_code::init_counters(
        &pool,
        &db,
        &[(entity.to_string(), "TE".to_string(), 1000, 4)],
    )
    .await
    .expect("init_counters");
    let code1 = redis_code::get_next_code(&pool, &db, entity).await.expect("get_code 1");
    let code2 = redis_code::get_next_code(&pool, &db, entity).await.expect("get_code 2");
    assert_eq!(code1, "TE1001"); // default_code 1000 + counter 1
    assert_eq!(code2, "TE1002");
}

#[tokio::test]
#[ignore]
async fn init_counters_with_digits_format() {
    let pool = make_pool().await;
    let db = unique_db();
    let entity = "digits_entity";
    redis_code::init_counters(
        &pool,
        &db,
        &[(entity.to_string(), "DV".to_string(), 0, 6)],
    )
    .await
    .expect("init");
    let code = redis_code::get_next_code(&pool, &db, entity).await.expect("get_code");
    assert_eq!(code, "DV000001");
}

#[tokio::test]
#[ignore]
async fn get_code_empty_table_returns_error() {
    use counter_service::generated::code_service_server::CodeService;
    use counter_service::generated::GetCodeRequest;
    use counter_service::server::CodeServiceImpl;
    use tonic::Request;

    let pool = make_pool().await;
    let svc = CodeServiceImpl::new(pool);
    let req = Request::new(GetCodeRequest {
        database: "default".to_string(),
        table: "".to_string(),
    });
    let res = svc.get_code(req).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
#[ignore]
async fn init_counters_empty_list_succeeds() {
    let pool = make_pool().await;
    let db = unique_db();
    redis_code::init_counters(&pool, &db, &[]).await.expect("init empty");
}

#[tokio::test]
#[ignore]
async fn init_counters_multiple_entities() {
    let pool = make_pool().await;
    let db = unique_db();
    let entities: Vec<(String, String, i32, i32)> = vec![
        ("e1".into(), "A".into(), 10, 0),
        ("e2".into(), "B".into(), 20, 2),
    ];
    redis_code::init_counters(&pool, &db, &entities).await.expect("init");
    let c1 = redis_code::get_next_code(&pool, &db, "e1").await.expect("e1");
    let c2 = redis_code::get_next_code(&pool, &db, "e2").await.expect("e2");
    assert_eq!(c1, "A11");
    assert_eq!(c2, "B01");
}
