// src/modbus/tcp.rs

use anyhow::{Context, Result};
use modbus::{client::sync::tcp, Client};
use std::net::TcpStream;
use std::time::Duration;

/// Modbus TCP client
pub struct ModbusTcp {
    ctx: tcp::Context,
}

impl ModbusTcp {
    /// Connect to `addr:port`, e.g. ("192.168.1.100", 502)
    pub fn new(addr: &str, port: u16) -> Result<Self> {
        let socket_addr = format!("{}:{}", addr, port);
        let stream = TcpStream::connect(&socket_addr)
            .with_context(|| format!("Failed to connect to Modbus TCP at {}", socket_addr))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .ok();
        stream
            .set_write_timeout(Some(Duration::from_secs(1)))
            .ok();

        let ctx = tcp::Context::new(stream);
        Ok(ModbusTcp { ctx })
    }

    /// Read `count` input registers starting at `addr`
    pub fn read_input_registers(&mut self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let regs = self
            .ctx
            .read_input_registers(1, addr, count) // unit ID = 1 by default
            .context("TCP read_input_registers failed")?;
        Ok(regs)
    }
}
