// src/modbus/mod.rs

use anyhow::Result;

mod rtu;
mod tcp;

pub use rtu::ModbusRtu;
pub use tcp::ModbusTcp;

/// Unified Modbus client: RTU or TCP
pub enum ModbusClient {
    Rtu(ModbusRtu),
    Tcp(ModbusTcp),
}

impl ModbusClient {
    /// Create an RTU client over a serial port (RS485)
    pub fn new_rtu(path: &str, baud: u32, slave: u8) -> Result<Self> {
        let client = ModbusRtu::new(path, baud, slave)?;
        Ok(ModbusClient::Rtu(client))
    }

    /// Create a TCP client (Modbus/TCP)
    pub fn new_tcp(addr: &str, port: u16) -> Result<Self> {
        let client = ModbusTcp::new(addr, port)?;
        Ok(ModbusClient::Tcp(client))
    }

    /// Read `count` input registers starting at `addr`
    pub fn read_input_registers(&mut self, addr: u16, count: u16) -> Result<Vec<u16>> {
        match self {
            ModbusClient::Rtu(r) => r.read_input_registers(addr, count),
            ModbusClient::Tcp(t) => t.read_input_registers(addr, count),
        }
    }
}
