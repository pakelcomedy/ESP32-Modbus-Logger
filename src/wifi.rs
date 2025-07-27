// src/wifi.rs

use anyhow::Result;
use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
};
use log::info;

/// A simple Wi‑Fi client wrapper around EspWifi
pub struct WifiClient {
    wifi: EspWifi,
}

impl WifiClient {
    /// Create a new WifiClient.
    ///
    /// # Arguments
    /// * `sysloop` – shared ESP‑IDF event loop
    /// * `nvs` – NVS partition for Wi‑Fi credentials
    pub fn new(
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self> {
        let wifi = EspWifi::new_default(sysloop, Some(nvs))?;
        Ok(WifiClient { wifi })
    }

    /// Connect to the specified SSID/password, blocking until an IP is obtained.
    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<()> {
        info!("🔌 Configuring Wi‑Fi SSID “{}” …", ssid);
        let client_conf = ClientConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            ..Default::default()
        };
        self.wifi.set_configuration(&Configuration::Client(client_conf))?;
        self.wifi.start()?;
        self.wifi.connect()?;

        info!("⏳ Waiting for Wi‑Fi connection...");
        while !self.wifi.is_connected()? {
            // spin until connected
        }
        info!("✅ Connected to Wi‑Fi");
        Ok(())
    }

    /// Retrieve the station interface's IP info (IP, gateway, netmask).
    pub fn get_ip_info(
        &self,
    ) -> Result<esp_idf_svc::netif::EspNetifIpInfo<'_>> {
        let ip_info = self.wifi.sta_netif().get_ip_info()?;
        info!(
            "📶 IP acquired: {} (gw: {}, mask: {})",
            ip_info.ip, ip_info.gateway, ip_info.netmask
        );
        Ok(ip_info)
    }
}
