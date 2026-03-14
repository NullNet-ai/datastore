#[cfg(test)]
mod tests {
    use crate::controllers::store_controller::create_function;
    use crate::controllers::store_controller::create_trigger;
    use crate::controllers::store_controller::{create_materialized_view, create_procedure};
    use crate::structs::core::Auth;
    use actix_web::{test::TestRequest, web, HttpMessage, HttpRequest, Responder};
    use serde_json::json;

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

    #[tokio::test]
    async fn function_missing_unsafe_query_returns_bad_request() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({ "returns": "integer" }));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_invalid_name_returns_bad_request() {
        let req = make_root_request();
        let name = web::Path::from("invalid-name".to_string());
        let body = web::Json(json!({"unsafe_query": "SELECT 1 LIMIT 1", "returns": "integer"}));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_rejects_destructive_statements() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body =
            web::Json(json!({"unsafe_query": "DELETE FROM x WHERE id = 1;", "returns": "void"}));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_requires_select_limit() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({"unsafe_query": "SELECT * FROM x;", "returns": "void"}));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_update_requires_where() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({"unsafe_query": "UPDATE x SET a = 1;", "returns": "void"}));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_rejects_destructive_execute_payload() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body =
            web::Json(json!({"unsafe_query": "EXECUTE $$ TRUNCATE x $$;", "returns": "void"}));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_arguments_invalid_string_spec() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({
            "arguments": ["arg1 integer;"],
            "unsafe_query": "SELECT 1 LIMIT 1;",
            "returns": "integer"
        }));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_arguments_invalid_object_spec() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({
            "arguments": [{"name": "arg1", "type": "integer;"}],
            "unsafe_query": "SELECT 1 LIMIT 1;",
            "returns": "integer"
        }));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_arguments_must_be_array() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({
            "arguments": "not-an-array",
            "unsafe_query": "SELECT 1 LIMIT 1;",
            "returns": "integer"
        }));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_invalid_returns_type() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({
            "unsafe_query": "SELECT 1 LIMIT 1;",
            "returns": "integer;"
        }));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn function_valid_input_attempts_execution() {
        let req = make_root_request();
        let name = web::Path::from("calc_sum".to_string());
        let body = web::Json(json!({
            "arguments": ["a integer", "b integer"],
            "unsafe_query": "SELECT 1 LIMIT 1;",
            "returns": "integer"
        }));
        let resp = create_function(req, name, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        let status = resp.respond_to(&assert_req).status().as_u16();
        assert_ne!(status, 400);
    }

    // ===== Trigger tests =====
    #[tokio::test]
    async fn trigger_missing_both_trigger_and_unsafe_query_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({}));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_invalid_timing_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "name": "trg_contacts_touch",
                "timing": "AFTERX",
                "event": ["UPDATE"],
                "level": "ROW"
            },
            "unsafe_query": "RETURN NEW;"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_invalid_event_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "timing": "AFTER",
                "event": ["BAD"],
                "level": "STATEMENT"
            },
            "unsafe_query": "RETURN NULL;"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_invalid_level_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "timing": "BEFORE",
                "event": ["UPDATE"],
                "level": "ROWX"
            },
            "unsafe_query": "RETURN NEW;"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_invalid_name_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "name": "invalid-name",
                "timing": "BEFORE",
                "event": ["UPDATE"],
                "level": "ROW"
            },
            "unsafe_query": "RETURN NEW;"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_invalid_referenced_table_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "timing": "AFTER",
                "event": ["INSERT"],
                "level": "STATEMENT",
                "referenced_table": "invalid-table"
            },
            "unsafe_query": "RETURN NULL;"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_invalid_transition_alias_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "timing": "AFTER",
                "event": ["INSERT"],
                "level": "STATEMENT",
                "transition_relations": { "old_table": "old-alias" }
            },
            "unsafe_query": "RETURN NULL;"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_structured_missing_unsafe_query_returns_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "timing": "AFTER",
                "event": ["INSERT"],
                "level": "ROW"
            }
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        assert_eq!(resp.respond_to(&assert_req).status().as_u16(), 400);
    }

    #[tokio::test]
    async fn trigger_raw_sql_executes_or_errors_but_not_bad_request() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "unsafe_query": "CREATE TRIGGER trg_tmp AFTER INSERT ON contacts FOR EACH ROW EXECUTE FUNCTION fn_tmp()"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        let status = resp.respond_to(&assert_req).status().as_u16();
        assert_ne!(status, 400);
    }

    #[tokio::test]
    async fn trigger_structured_valid_input_attempts_execution() {
        let req = make_root_request();
        let table = web::Path::from("contacts".to_string());
        let body = web::Json(json!({
            "trigger": {
                "timing": "BEFORE",
                "event": ["UPDATE"],
                "level": "ROW"
            },
            "unsafe_query": "RETURN NEW;"
        }));
        let resp = create_trigger(req, table, body).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        let status = resp.respond_to(&assert_req).status().as_u16();
        assert_ne!(status, 400);
    }

    // ===== Idempotency tests =====
    #[tokio::test]
    async fn matview_idempotent_creation_does_not_return_bad_request() {
        let req1 = make_root_request();
        let table1 = web::Path::from("active_contacts_view".to_string());
        let body1 = web::Json(json!({
            "unsafe_query": "SELECT id FROM contacts WHERE status = 'Active' LIMIT 1"
        }));
        let resp1 = create_materialized_view(req1, table1, body1).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        let status1 = resp1.respond_to(&assert_req).status().as_u16();
        assert_ne!(status1, 400);

        let req2 = make_root_request();
        let table2 = web::Path::from("active_contacts_view".to_string());
        let body2 = web::Json(json!({
            "unsafe_query": "SELECT id FROM contacts WHERE status = 'Active' LIMIT 1"
        }));
        let resp2 = create_materialized_view(req2, table2, body2).await;
        let status2 = resp2.respond_to(&assert_req).status().as_u16();
        assert_ne!(status2, 400);
    }

    #[tokio::test]
    async fn procedure_idempotent_creation_does_not_return_bad_request() {
        let req1 = make_root_request();
        let name1 = web::Path::from("udp_idem_proc".to_string());
        let body1 = web::Json(json!({
            "unsafe_query": "SELECT 1 LIMIT 1;"
        }));
        let resp1 = create_procedure(req1, name1, body1).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        let status1 = resp1.respond_to(&assert_req).status().as_u16();
        assert_ne!(status1, 400);

        let req2 = make_root_request();
        let name2 = web::Path::from("udp_idem_proc".to_string());
        let body2 = web::Json(json!({
            "unsafe_query": "SELECT 1 LIMIT 1;"
        }));
        let resp2 = create_procedure(req2, name2, body2).await;
        let status2 = resp2.respond_to(&assert_req).status().as_u16();
        assert_ne!(status2, 400);
    }

    #[tokio::test]
    async fn function_idempotent_creation_does_not_return_bad_request() {
        let req1 = make_root_request();
        let name1 = web::Path::from("calc_sum_idem".to_string());
        let body1 = web::Json(json!({
            "arguments": ["a integer", "b integer"],
            "unsafe_query": "RETURN a + b;",
            "returns": "integer"
        }));
        let resp1 = create_function(req1, name1, body1).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        let status1 = resp1.respond_to(&assert_req).status().as_u16();
        assert_ne!(status1, 400);

        let req2 = make_root_request();
        let name2 = web::Path::from("calc_sum_idem".to_string());
        let body2 = web::Json(json!({
            "arguments": ["a integer", "b integer"],
            "unsafe_query": "RETURN a + b;",
            "returns": "integer"
        }));
        let resp2 = create_function(req2, name2, body2).await;
        let status2 = resp2.respond_to(&assert_req).status().as_u16();
        assert_ne!(status2, 400);
    }

    #[tokio::test]
    async fn trigger_idempotent_structured_does_not_return_bad_request() {
        let req1 = make_root_request();
        let table1 = web::Path::from("contacts".to_string());
        let body1 = web::Json(json!({
            "trigger": {
                "name": "trg_contacts_touch_idem",
                "timing": "BEFORE",
                "event": ["UPDATE"],
                "level": "ROW"
            },
            "unsafe_query": "RETURN NEW;"
        }));
        let resp1 = create_trigger(req1, table1, body1).await;
        let assert_req = actix_web::test::TestRequest::default().to_http_request();
        let status1 = resp1.respond_to(&assert_req).status().as_u16();
        assert_ne!(status1, 400);

        let req2 = make_root_request();
        let table2 = web::Path::from("contacts".to_string());
        let body2 = web::Json(json!({
            "trigger": {
                "name": "trg_contacts_touch_idem",
                "timing": "BEFORE",
                "event": ["UPDATE"],
                "level": "ROW"
            },
            "unsafe_query": "RETURN NEW;"
        }));
        let resp2 = create_trigger(req2, table2, body2).await;
        let status2 = resp2.respond_to(&assert_req).status().as_u16();
        assert_ne!(status2, 400);
    }
}
