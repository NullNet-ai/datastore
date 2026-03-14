#[cfg(test)]
mod tests {
    use actix_web::{test::TestRequest, web, HttpRequest, Responder, HttpMessage};
    use serde_json::json;
    use crate::controllers::store_controller::{create_materialized_view, create_procedure};
    use crate::structs::core::Auth;

    fn make_root_request() -> HttpRequest {
        let req = TestRequest::default().to_http_request();
        req.extensions_mut().insert(Some("root".to_string()));
        req.extensions_mut().insert(Auth {
            organization_id: "".to_string(),
            responsible_account: "".to_string(),
            sensitivity_level: 0,
            role_name: "".to_string(),
            account_organization_id: "".to_string(),
            role_id: "".to_string(),
            is_root_account: true,
            account_id: "".to_string(),
        });
        req
    }

    #[tokio::test]
    async fn matview_missing_unsafe_query_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("valid_view".to_string());
        let body = web::Json(json!({}));
        let resp = create_materialized_view(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn matview_invalid_name_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("invalid-name".to_string());
        let body = web::Json(json!({"unsafe_query": "SELECT 1 LIMIT 1"}));
        let resp = create_materialized_view(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_rejects_destructive_statements() {
        let req = make_root_request();
        let name = web::Path::from("udp_safe_proc".to_string());
        let body = web::Json(json!({"unsafe_query": "DELETE FROM x WHERE id = 1;"}));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_requires_select_limit() {
        let req = make_root_request();
        let name = web::Path::from("udp_limit_proc".to_string());
        let body = web::Json(json!({"unsafe_query": "SELECT * FROM x;"}));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_rejects_excessive_limit() {
        let req = make_root_request();
        let name = web::Path::from("udp_limit_proc2".to_string());
        let body = web::Json(json!({"unsafe_query": "SELECT * FROM x LIMIT 20001;"}));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_update_requires_where() {
        let req = make_root_request();
        let name = web::Path::from("udp_update_proc".to_string());
        let body = web::Json(json!({"unsafe_query": "UPDATE x SET a = 1;"}));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_rejects_destructive_execute_payload() {
        let req = make_root_request();
        let name = web::Path::from("udp_exec_proc".to_string());
        let body = web::Json(json!({"unsafe_query": "EXECUTE $$ TRUNCATE x $$;"}));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_execute_update_requires_where() {
        let req = make_root_request();
        let name = web::Path::from("udp_exec_update_proc".to_string());
        let body = web::Json(json!({"unsafe_query": "EXECUTE $$ UPDATE x SET a = 1 $$;"}));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_arguments_invalid_string_spec() {
        let req = make_root_request();
        let name = web::Path::from("udp_args_proc".to_string());
        let body = web::Json(json!({
            "arguments": ["arg1 integer;"],
            "unsafe_query": "SELECT 1 LIMIT 1;"
        }));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_arguments_invalid_object_spec() {
        let req = make_root_request();
        let name = web::Path::from("udp_args_proc2".to_string());
        let body = web::Json(json!({
            "arguments": [{"name": "arg1", "type": "integer;"}],
            "unsafe_query": "SELECT 1 LIMIT 1;"
        }));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn procedure_arguments_must_be_array() {
        let req = make_root_request();
        let name = web::Path::from("udp_args_proc3".to_string());
        let body = web::Json(json!({
            "arguments": "not-an-array",
            "unsafe_query": "SELECT 1 LIMIT 1;"
        }));
        let resp = create_procedure(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }
}
