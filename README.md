```
esp32-modbus-logger/
├── Cargo.toml
├── memory.x
├── .cargo/
│   └── config.toml
├── src/
│   ├── main.rs              # Entry point, init semua komponen
│   ├── modbus/
│   │   ├── mod.rs           # Abstraksi Modbus
│   │   ├── rtu.rs           # Modbus RTU (via UART RS485)
│   │   └── tcp.rs           # Modbus TCP (via Wi-Fi Ethernet)
│   ├── logger/
│   │   ├── mod.rs           # Logging interface
│   │   ├── file_logger.rs   # Simpan log ke file (SPIFFS / SD)
│   │   └── dashboard.rs     # Web dashboard via HTTP
│   ├── alert/
│   │   ├── mod.rs           # Notifikasi sistem
│   │   └── smtp.rs          # Kirim email via SMTP
│   └── wifi.rs              # Setup koneksi Wi-Fi (TCP & SMTP)
├── fs/
│   ├── log.csv              # Contoh file log
│   └── config.json          # Threshold & konfigurasi lainnya
└── website/
    ├── index.html           # Dashboard utama (grafik data)
    ├── style.css
    └── app.js               # Realtime fetch dari ESP32
```
