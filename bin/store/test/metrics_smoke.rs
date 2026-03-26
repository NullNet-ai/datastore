use metrics_exporter_prometheus::PrometheusBuilder;

#[test]
fn test_metrics_endpoint_serves() {
    let _ = PrometheusBuilder::new().install();
    std::thread::sleep(std::time::Duration::from_millis(200));
    let resp = reqwest::blocking::get("http://127.0.0.1:9000/metrics")
        .expect("failed to GET metrics endpoint");
    assert!(resp.status().is_success());
}
