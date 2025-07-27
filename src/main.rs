// src/main.rs

#![no_std]
#![no_main]

use esp_idf_sys as _; // link ESP‑IDF native libraries
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration as HttpConfig, EspHttpServer},
    log::EspLogger,
    nvs::EspDefaultNvsPartition,
};
use log::*;
use anyhow::Result;
use chrono::Utc;
use core::panic::PanicInfo;
use std::thread;
use std::time::Duration;

mod wifi;
mod modbus;
mod logger;
mod alert;

use wifi::WifiClient;
use modbus::ModbusClient;
use logger::{FileLogger, register_logger_routes};
use alert::SmtpAlert;

/// required for `#![no_std]`
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    EspLogger::println(format_args!("Panic: {:?}", info)).unwrap();
    loop {}
}

#[entry]
fn main() -> Result<()> {
    // Initialize ESP‑IDF logger
    EspLogger::initialize_default();
    info!("⏱️  ESP32 Industrial Modbus Logger starting up…");

    // Take system event loop for Wi‑Fi and peripherals
    let sysloop = EspSystemEventLoop::take()?;

    // 1. Connect to Wi‑Fi
    let mut wifi = WifiClient::new(sysloop.clone(), EspDefaultNvsPartition::take()?)?;
    wifi.connect("YOUR_SSID", "YOUR_PASSWORD")?;
    let ip_info = wifi.get_ip_info()?;
    info!("✅ Wi‑Fi connected, IP = {}", ip_info.ip);

    // 2. Start HTTP server for dashboard
    let server = EspHttpServer::new(&HttpConfig::default())?;
    register_logger_routes(&server, "/sdcard/log.csv")?;
    info!("🌐 HTTP dashboard available at http://{}/logs", ip_info.ip);

    // 3. Initialize file logger (e.g. SPIFFS or SD card)
    let mut file_logger = FileLogger::new("/sdcard/log.csv")?;
    file_logger.write_header("timestamp,register,value")?;
    info!("💾 File logger initialized");

    // 4. Initialize SMTP alert system
    let mut smtp = SmtpAlert::new(
        "smtp.gmail.com",  // SMTP server
        587,               // port
        "user@gmail.com",  // username
        "password",        // password
        "from@gmail.com",  // from
        "alert@gmail.com", // to
    )?;
    info!("📧 SMTP alert system ready");

    // 5. Initialize Modbus client (RTU or TCP)
    // RTU over UART/RS485:
    let mut mb = ModbusClient::new_rtu("/dev/ttyUSB0", 9600, 1)?;
    // Or for Modbus TCP:
    // let mut mb = ModbusClient::new_tcp("192.168.1.100", 502)?;
    info!("🔌 Modbus client initialized");

    // Threshold for alert
    const ALERT_THRESHOLD: f32 = 75.0;

    // 6. Main polling loop
    loop {
        let regs = mb.read_input_registers(100, 1)?;
        let raw = regs[0];
        let value = raw as f32 * 0.1; // apply scale if needed

        info!("🔄 Modbus read: addr=100 → value={:.2}", value);

        let timestamp = Utc::now().to_rfc3339();
        file_logger.log(&timestamp, 100, value)?;
        info!("💾 Logged to file");

        if value > ALERT_THRESHOLD {
            info!("⚠️  Threshold exceeded ({:.2}), sending alert", value);
            smtp.send_alert(&format!("Modbus alarm: register 100 = {:.2}", value))?;
            info!("✉️  Alert email sent");
        }

        thread::sleep(Duration::from_secs(10));
    }
}