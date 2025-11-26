#!/usr/bin/env python3
import json
import time
import os
import subprocess
import urllib.request
import urllib.error
from datetime import datetime, timezone, timedelta

# Configuration
ALERTMANAGER_URL = os.environ.get("ALERTMANAGER_URL", "http://central-alertmanager:9093")
INSTANCE_NAME = os.environ.get("INSTANCE_NAME", "kreivo-01") # Must match Prometheus instance label
POLL_INTERVAL = 30
SILENCE_DURATION_MINUTES = 15

def log(msg):
    print(f"[{datetime.now().isoformat()}] {msg}")

def get_alerts():
    url = f"{ALERTMANAGER_URL}/api/v2/alerts?filter=alert_action=%22restart%22&filter=instance=%22{INSTANCE_NAME}%22&active=true"
    try:
        with urllib.request.urlopen(url) as response:
            if response.status == 200:
                return json.loads(response.read().decode())
    except urllib.error.URLError as e:
        log(f"Error polling Alertmanager: {e}")
    return []

def restart_service():
    log("Executing restart command...")
    try:
        subprocess.run(["sudo", "systemctl", "restart", "kreivo"], check=True)
        log("Service restarted successfully.")
        return True
    except subprocess.CalledProcessError as e:
        log(f"Failed to restart service: {e}")
        return False

def create_silence():
    log("Creating silence in Alertmanager...")
    now = datetime.now(timezone.utc)
    end = now + timedelta(minutes=SILENCE_DURATION_MINUTES)
    
    payload = {
        "matchers": [
            {"name": "instance", "value": INSTANCE_NAME, "isRegex": False},
            {"name": "alert_action", "value": "restart", "isRegex": False}
        ],
        "startsAt": now.isoformat(),
        "endsAt": end.isoformat(),
        "createdBy": "kreivo-polling-agent",
        "comment": f"Auto-silence after restart by polling agent on {INSTANCE_NAME}"
    }
    
    url = f"{ALERTMANAGER_URL}/api/v2/silences"
    req = urllib.request.Request(url, data=json.dumps(payload).encode(), headers={'Content-Type': 'application/json'})
    
    try:
        with urllib.request.urlopen(req) as response:
            if response.status == 200:
                result = json.loads(response.read().decode())
                log(f"Silence created. ID: {result.get('silenceID')}")
    except urllib.error.URLError as e:
        log(f"Failed to create silence: {e}")

def main():
    log(f"Starting Polling Agent for instance: {INSTANCE_NAME}")
    log(f"Alertmanager URL: {ALERTMANAGER_URL}")
    
    while True:
        alerts = get_alerts()
        if alerts:
            log(f"Found {len(alerts)} active restart alerts.")
            if restart_service():
                create_silence()
            else:
                log("Restart failed, skipping silence creation.")
        
        time.sleep(POLL_INTERVAL)

if __name__ == "__main__":
    main()
