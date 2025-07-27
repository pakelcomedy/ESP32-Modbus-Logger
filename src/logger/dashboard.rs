// src/logger/dashboard.rs

use anyhow::{Context, Result};
use esp_idf_svc::http::server::EspHttpServer;
use embedded_svc::http::Method;
use std::fs;

/// Register HTTP endpoint `/logs` to serve the CSV log file.
/// 
/// # Arguments
/// * `server` — your EspHttpServer instance  
/// * `log_path` — path to the CSV log file  
pub fn register_logger_routes(server: &EspHttpServer, log_path: &str) -> Result<()> {
    let lp = log_path.to_string();

    server.fn_handler("/logs", Method::Get, move |_req| {
        // Read entire log file
        let content = fs::read_to_string(&lp)
            .with_context(|| format!("Failed to read log file {}", lp))?;
        let mut resp = esp_idf_svc::http::server::Response::new(200);
        resp.set_header("Content-Type", "text/csv; charset=utf-8");
        resp.send_str(&content);
        Ok(resp)
    })?;

    Ok(())
}
