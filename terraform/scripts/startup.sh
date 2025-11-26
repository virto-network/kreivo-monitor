#!/bin/bash

set -e

# Install Docker and Docker Compose
apt-get update
apt-get install -y docker.io docker-compose

# Create directory structure
mkdir -p /opt/kreivo-monitor/central/grafana/provisioning/datasources
cd /opt/kreivo-monitor

# Get Bucket Name from Metadata
BUCKET_NAME=$(curl -H "Metadata-Flavor: Google" http://metadata.google.internal/computeMetadata/v1/instance/attributes/config-bucket)

# Download configuration files
gsutil cp gs://${BUCKET_NAME}/docker-compose.yml .
gsutil cp gs://${BUCKET_NAME}/central/prometheus.yml ./central/
gsutil cp gs://${BUCKET_NAME}/central/alertmanager.yml ./central/
gsutil cp gs://${BUCKET_NAME}/central/alerts.yml ./central/
gsutil cp gs://${BUCKET_NAME}/central/grafana/provisioning/datasources/prometheus.yml ./central/grafana/provisioning/datasources/

# Start the stack
docker-compose up -d
