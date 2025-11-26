use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "/etc/kreivo-monitor/config.yaml")]
    pub config: String,

    #[arg(long)]
    pub scrape_url: Option<String>,

    #[arg(long)]
    pub alertmanager_url: Option<String>,

    #[arg(long)]
    pub scrape_interval_seconds: Option<u64>,

    #[arg(long)]
    pub poll_interval_seconds: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub scrape_url: String,
    pub remote_write: Vec<RemoteWriteConfig>,
    pub alertmanager_url: String,
    pub actions: HashMap<String, String>,
    #[serde(default = "default_scrape_interval")]
    pub scrape_interval_seconds: u64,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_seconds: u64,
}

fn default_scrape_interval() -> u64 {
    15
}
fn default_poll_interval() -> u64 {
    30
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoteWriteConfig {
    pub url: String,
    pub auth_header: Option<String>,
}

impl Config {
    fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;
        let config: Config =
            serde_yaml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn build(args: Args) -> Result<Self> {
        let mut config = Self::load(&args.config)?;

        if let Some(url) = args.scrape_url {
            config.scrape_url = url;
        }
        if let Some(url) = args.alertmanager_url {
            config.alertmanager_url = url;
        }
        if let Some(interval) = args.scrape_interval_seconds {
            config.scrape_interval_seconds = interval;
        }
        if let Some(interval) = args.poll_interval_seconds {
            config.poll_interval_seconds = interval;
        }

        Ok(config)
    }
}
