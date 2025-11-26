use anyhow::{Context, Result};
use log::{info, error};
use reqwest::Client;
use std::collections::HashMap;
use std::process::Command;

pub struct Poller {
    client: Client,
    alertmanager_url: String,
    actions: HashMap<String, String>,
}

impl Poller {
    pub fn new(client: Client, alertmanager_url: String, actions: HashMap<String, String>) -> Self {
        Self { client, alertmanager_url, actions }
    }

    pub async fn poll_and_remediate(&self, instance: &str) -> Result<()> {
        // We poll for ANY alert that has an 'alert_action' label matching one of our configured actions
        // and matches our instance.
        
        for (action_name, command) in &self.actions {
            let url = format!("{}/api/v2/alerts?filter=alert_action=%22{}%22&filter=instance=%22{}%22&active=true", 
                self.alertmanager_url, action_name, instance);
            
            let resp = self.client.get(&url).send().await.context("Failed to poll Alertmanager")?;
            
            if !resp.status().is_success() {
                continue;
            }

            let alerts: serde_json::Value = resp.json().await?;
            
            if let Some(arr) = alerts.as_array() {
                if !arr.is_empty() {
                    info!("Found active alert for action '{}'. Executing: {}", action_name, command);
                    if let Err(e) = self.execute_command(command) {
                        error!("Failed to execute command '{}': {}", command, e);
                    } else {
                        info!("Command executed successfully.");
                        // Create silence
                        if let Err(e) = self.create_silence(instance, action_name).await {
                            error!("Failed to create silence: {}", e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn execute_command(&self, command: &str) -> Result<()> {
        // Execute shell command
        let status = Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .context("Failed to execute shell command")?;

        if !status.success() {
            anyhow::bail!("Command failed with exit code: {:?}", status.code());
        }
        Ok(())
    }

    async fn create_silence(&self, instance: &str, action: &str) -> Result<()> {
        let now = chrono::Utc::now();
        let end = now + chrono::Duration::minutes(15);
        
        let payload = serde_json::json!({
            "matchers": [
                {"name": "instance", "value": instance, "isRegex": false},
                {"name": "alert_action", "value": action, "isRegex": false}
            ],
            "startsAt": now.to_rfc3339(),
            "endsAt": end.to_rfc3339(),
            "createdBy": "kreivo-monitor-agent",
            "comment": format!("Auto-silence after {} by agent on {}", action, instance)
        });

        self.client.post(format!("{}/api/v2/silences", self.alertmanager_url))
            .json(&payload)
            .send()
            .await
            .context("Failed to create silence")?;
            
        Ok(())
    }
}
