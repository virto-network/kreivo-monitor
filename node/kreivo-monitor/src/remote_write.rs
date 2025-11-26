use crate::config::RemoteWriteConfig;
use log::{debug, error};
use prometheus_reqwest_remote_write::{TimeSeries, WriteRequest};
use reqwest::Client;

pub struct RemoteWriteClient {
    client: Client,
    configs: Vec<RemoteWriteConfig>,
}

impl RemoteWriteClient {
    pub fn new(client: Client, configs: Vec<RemoteWriteConfig>) -> Self {
        Self { client, configs }
    }

    pub async fn send(&self, timeseries: Vec<TimeSeries>) {
        if timeseries.is_empty() {
            return;
        }

        for config in &self.configs {
            let write_request = WriteRequest {
                timeseries: timeseries.clone(),
            };

            // Build request
            let mut req_builder = match write_request.build_http_request(
                self.client.clone(),
                config.url.as_str(),
                "kreivo-monitor/0.1.0",
            ) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to build request for {}: {}", config.url, e);
                    continue;
                }
            };

            // Add Auth Header if present
            if let Some(token) = &config.auth_header {
                let headers = req_builder.headers_mut();
                if let Ok(val) = reqwest::header::HeaderValue::from_str(token) {
                    headers.insert(reqwest::header::AUTHORIZATION, val);
                }
            }

            // Send
            if let Err(e) = self.client.execute(req_builder).await {
                error!("Failed to send metrics to {}: {}", config.url, e);
                return;
            }

            debug!("Sent metrics to {}", config.url);
        }
    }
}
