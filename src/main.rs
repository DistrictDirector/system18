use anyhow::Result;
use dotenvy::dotenv;
use std::env;

mod notifier;
use notifier::{EmailMessage, send_notification};
//===============================================================
//Testing the email sender – this is just a quick test to verify the SMTP setup works. You can remove or comment out this code after confirming it works.
//===============================================================
#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file (do this early)
    dotenv().ok();

    // Quick debug: print the env vars we're using (without showing the password)
    println!("Testing email sender...");
    println!("SMTP_HOST:     {}", env::var("SMTP_HOST").unwrap_or("not set".to_string()));
    println!("SMTP_PORT:     {}", env::var("SMTP_PORT").unwrap_or("not set".to_string()));
    println!("SMTP_USERNAME: {}", env::var("SMTP_USERNAME").unwrap_or("not set".to_string()));
    println!("FROM_EMAIL:    {}", env::var("FROM_EMAIL").unwrap_or("not set".to_string()));
    println!("FROM_NAME:     {}", env::var("FROM_NAME").unwrap_or("Trading Bot".to_string()));

    // ────────────────────────────────────────────────
    //  Test email – change these values!
    // ────────────────────────────────────────────────
    let test_email = EmailMessage {
        recipients: vec!["upatheartist@gmail.com".to_string()],  // ← put YOUR email here to receive it
        subject:     "Test Email from Trading Bot".to_string(),
        plain_body:  "Hello!\n\nThis is a test message sent from your Rust trading bot.\n\nIf you see this, everything is working! 🚀\n\nSent at: 2026-02-13".to_string(),
        html_body:   Some(
            r#"
            <h2 style="color: #2e7d32;">Test Notification</h2>
            <p>Hello from your Rust trading bot!</p>
            <p><strong>This is just a test.</strong> If you're reading this, the SMTP setup is working correctly.</p>
            <p style="color: #555; font-size: 0.9em;">
                Sent on February 13, 2026 from Farragut, TN
            </p>
            "#.to_string()
        ),
    };

    println!("\nSending test email to: {}", test_email.recipients.join(", "));

    match send_notification(&test_email).await {
        Ok(()) => {
            println!("\n✅ Email sent successfully!");
        }
        Err(e) => {
            eprintln!("\n❌ Failed to send email: {:#}", e);
        }
    }

    // Your normal bot logic would go here...
    println!("\nTest complete. You can now remove or comment out the test code.");

    Ok(())
}
//End of Test code