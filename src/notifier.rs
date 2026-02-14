// src/notifier.rs
use anyhow::{Context, Result};
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use std::env;

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub recipients: Vec<String>,
    pub subject: String,
    pub plain_body: String,
    pub html_body: Option<String>,
}

pub async fn send_notification(message: &EmailMessage) -> Result<()> {
    let smtp_host = env::var("SMTP_HOST").unwrap_or("smtp.gmail.com".to_string());
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or("587".into())
        .parse()
        .context("Invalid SMTP_PORT")?;

    let username = env::var("SMTP_USERNAME").context("SMTP_USERNAME missing")?;
    let password = env::var("SMTP_PASSWORD").context("SMTP_PASSWORD missing")?;
    let from_email = env::var("FROM_EMAIL").context("FROM_EMAIL missing")?;
    let from_name = env::var("FROM_NAME").unwrap_or("Trading Bot".to_string());

    // Build the email message
    let recipients: Vec<(&str, &str)> = message.recipients.iter()
        .map(|r| (r.as_str(), r.as_str()))
        .collect();

    let builder = MessageBuilder::new()
        .from((from_name.as_str(), from_email.as_str()))
        .to(recipients)
        .subject(&message.subject);

    // Add body content
    let builder = if let Some(html) = &message.html_body {
        builder.text_body(&message.plain_body).html_body(html)
    } else {
        builder.text_body(&message.plain_body)
    };

    // Build the SMTP client and connect
    let mut client = SmtpClientBuilder::new(smtp_host, smtp_port)
        .implicit_tls(false)
        .credentials((username, password))
        .connect()
        .await
        .context("Failed to connect to SMTP server")?;

    // Send the email
    client.send(builder).await.context("Failed to send email")?;

    Ok(())
}


/*
//===============================================================
//example for how to use in bot code:
//===============================================================
// src/main.rs

mod notifier;  // ← This line includes src/notifier.rs as a module
               // Now you can use notifier::EmailMessage and notifier::send_notification

use anyhow::Result;
use notifier::{EmailMessage, send_notification};

// If using .env file:
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();  // loads .env if present — very convenient

    // Example: create and send an alert
    let alert = EmailMessage {
        recipients: vec!["alan@example.com".to_string()],
        subject: "Test Alert from Bot".to_string(),
        plain_body: "This is a test notification.".to_string(),
        html_body: Some("<h2>Test Alert</h2><p>Works great!</p>".to_string()),
    };

    send_notification(&alert).await?;

    // ... rest of your bot logic (market polling, decisions, etc.)

    Ok(())
}
*/