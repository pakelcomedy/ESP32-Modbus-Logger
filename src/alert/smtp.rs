// src/alert/smtp.rs

use anyhow::{Context, Result};
use lettre::{
    message::Mailbox,
    transport::smtp::{
        authentication::Credentials,
        SmtpTransportBuilder,
    },
    Message, SmtpTransport, Transport,
};

/// Simple SMTP‐based alert sender
pub struct SmtpAlert {
    mailer: SmtpTransport,
    from: Mailbox,
    to: Mailbox,
}

impl SmtpAlert {
    /// Create a new SmtpAlert.
    ///
    /// # Arguments
    /// * `server` – SMTP server hostname (e.g. "smtp.example.com")
    /// * `port` – SMTP port (e.g. 587)
    /// * `username` – SMTP user name
    /// * `password` – SMTP password
    /// * `from` – RFC‐822 email address to send from
    /// * `to` – RFC‐822 email address to send alerts to
    pub fn new(
        server: &str,
        port: u16,
        username: &str,
        password: &str,
        from: &str,
        to: &str,
    ) -> Result<SmtpAlert> {
        // Build credentials
        let creds = Credentials::new(username.into(), password.into());

        // Build the SMTP transport
        let mailer = SmtpTransport::relay(server)
            .context("Failed to configure SMTP relay")?
            .port(port)
            .credentials(creds)
            .build();

        // Parse addresses
        let from_addr: Mailbox = from
            .parse()
            .context("Invalid 'from' email address")?;
        let to_addr: Mailbox = to
            .parse()
            .context("Invalid 'to' email address")?;

        Ok(SmtpAlert {
            mailer,
            from: from_addr,
            to: to_addr,
        })
    }

    /// Send a plain‐text alert with the given body.
    pub fn send_alert(&self, body: &str) -> Result<()> {
        // Build the email message
        let email = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .subject("🚨 ESP32 Modbus Alert")
            .body(body.to_string())
            .context("Failed to build email message")?;

        // Send it
        self.mailer
            .send(&email)
            .context("Failed to send alert email")?;

        Ok(())
    }
}