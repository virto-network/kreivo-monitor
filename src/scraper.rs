use anyhow::{Context, Result};
use prometheus_reqwest_remote_write::{Label, Sample, TimeSeries};
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Scraper {
    client: Client,
    scrape_url: String,
}

impl Scraper {
    pub fn new(client: Client, scrape_url: String) -> Self {
        Self { client, scrape_url }
    }

    pub async fn scrape(&self) -> Result<(Vec<TimeSeries>, Option<String>)> {
        let resp = self.client.get(&self.scrape_url).send().await.context("Failed to fetch metrics")?;
        let body = resp.text().await?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;
        let mut timeseries = Vec::new();
        let mut instance_name = None;

        for line in body.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if let Some((metric_part, value_part)) = line.rsplit_once(' ') {
                if let Ok(value) = value_part.parse::<f64>() {
                    let (name, labels_str) = if let Some(idx) = metric_part.find('{') {
                        (&metric_part[..idx], Some(&metric_part[idx+1..metric_part.len()-1]))
                    } else {
                        (metric_part, None)
                    };

                    let mut labels = vec![
                        Label { name: "__name__".to_string(), value: name.to_string() },
                        Label { name: "job".to_string(), value: "parachain-node-scraper".to_string() },
                    ];

                    if let Some(l_str) = labels_str {
                        for pair in l_str.split(',') {
                            if let Some((k, v)) = pair.split_once('=') {
                                let val = v.trim_matches('"');
                                labels.push(Label { name: k.to_string(), value: val.to_string() });
                                
                                // Extract instance name from substrate_build_info
                                if name == "substrate_build_info" && k == "name" {
                                    instance_name = Some(val.to_string());
                                }
                            }
                        }
                    }

                    timeseries.push(TimeSeries {
                        labels,
                        samples: vec![Sample { value, timestamp: now }],
                    });
                }
            }
        }
        
        // If we found an instance name, inject it into all timeseries
        if let Some(ref inst) = instance_name {
            for ts in &mut timeseries {
                // Check if instance label already exists (it shouldn't from scrape, but good to be safe)
                if !ts.labels.iter().any(|l| l.name == "instance") {
                    ts.labels.push(Label { name: "instance".to_string(), value: inst.clone() });
                }
            }
        }

        Ok((timeseries, instance_name))
    }
}
