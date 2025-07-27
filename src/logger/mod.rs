// src/logger/mod.rs

pub mod file_logger;
pub mod dashboard;

pub use file_logger::FileLogger;
pub use dashboard::register_logger_routes;
