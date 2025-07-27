// src/modbus/rtu.rs

use anyhow::{Context, Result};
use modbus::{client::sync::rtu, Client};
use serialport::prelude::*;
use std::time::Duration;

/// Modbus RTU client over serial/RS485
pub struct ModbusRtu {
    ctx: rtu::Context<Box<dyn SerialPort>>,
    slave: u8,
}

impl ModbusRtu {
    /// Open the serial port at `path` (e.g. "/dev/ttyUSB0") with `baud`
    /// and target slave ID `slave`.
    pub fn new(path: &str, baud: u32, slave: u8) -> Result<Self> {
        let settings = SerialPortSettings {
            baud_rate: baud,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::Even,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(1000),
        };
        let port = serialport::open_with_settings(path, &settings)
            .with_context(|| format!("Failed to open serial port {}", path))?;
        let ctx = rtu::Context::new(port);
        Ok(ModbusRtu { ctx, slave })
    }

    /// Read `count` input registers starting at `addr`
    pub fn read_input_registers(&mut self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let regs = self
            .ctx
            .read_input_registers(self.slave as u8, addr, count)
            .context("RTU read_input_registers failed")?;
        Ok(regs)
    }
}
