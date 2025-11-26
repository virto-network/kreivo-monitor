use anyhow::Result;
use clap::Parser;
use log::{error, info};
use reqwest::Client;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

use kreivo_monitor::config::{Args, Config};
use kreivo_monitor::poller::Poller;
use kreivo_monitor::remote_write::RemoteWriteClient;
use kreivo_monitor::scraper::Scraper;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    info!("Loading config from {}", args.config);
    let config = Config::build(args)?;

    let client = Client::new();

    // Shared state for instance name (updated by scraper, used by poller)
    // Initially None, will be populated once scraper finds it.
    let instance_name = Arc::new(Mutex::new(None::<String>));

    // Initialize components
    let scraper = Scraper::new(client.clone(), config.scrape_url.clone());
    let remote_write = RemoteWriteClient::new(client.clone(), config.remote_write.clone());
    let poller = Poller::new(
        client.clone(),
        config.alertmanager_url.clone(),
        config.actions.clone(),
    );

    // Scraper Task
    let instance_name_scraper = instance_name.clone();
    let scrape_interval = config.scrape_interval_seconds;
    tokio::spawn(async move {
        loop {
            match scraper.scrape().await {
                Ok((ts, inst)) => {
                    if let Some(name) = inst {
                        let mut lock = instance_name_scraper.lock().unwrap();
                        if lock.is_none() || lock.as_ref().unwrap() != &name {
                            info!("Detected instance name: {}", name);
                            *lock = Some(name);
                        }
                    }
                    remote_write.send(ts).await;
                }
                Err(e) => error!("Scrape error: {}", e),
            }
            time::sleep(Duration::from_secs(scrape_interval)).await;
        }
    });

    // Poller Task
    let instance_name_poller = instance_name.clone();
    let poll_interval = config.poll_interval_seconds;
    tokio::spawn(async move {
        loop {
            let current_instance = {
                let lock = instance_name_poller.lock().unwrap();
                lock.clone()
            };

            if let Some(inst) = current_instance {
                if let Err(e) = poller.poll_and_remediate(&inst).await {
                    error!("Poller error: {}", e);
                }
            } else {
                info!("Waiting for instance name detection...");
            }
            time::sleep(Duration::from_secs(poll_interval)).await;
        }
    });

    // Keep alive
    loop {
        time::sleep(Duration::from_secs(60)).await;
    }
}
