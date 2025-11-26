resource "random_id" "bucket_suffix" {
  byte_length = 4
}

resource "google_storage_bucket" "config_bucket" {
  name          = "kreivo-monitor-config-${var.project_id}-${random_id.bucket_suffix.hex}"
  location      = var.region
  force_destroy = true

  uniform_bucket_level_access = true
}

# Upload docker-compose.yml
resource "google_storage_bucket_object" "docker_compose" {
  name   = "docker-compose.yml"
  source = "../docker-compose.yml"
  bucket = google_storage_bucket.config_bucket.name
}

# Upload central/prometheus.yml
resource "google_storage_bucket_object" "prometheus_config" {
  name   = "central/prometheus.yml"
  source = "../central/prometheus.yml"
  bucket = google_storage_bucket.config_bucket.name
}

# Upload central/alertmanager.yml
resource "google_storage_bucket_object" "alertmanager_config" {
  name   = "central/alertmanager.yml"
  source = "../central/alertmanager.yml"
  bucket = google_storage_bucket.config_bucket.name
}

# Upload central/alerts.yml
resource "google_storage_bucket_object" "alerts_config" {
  name   = "central/alerts.yml"
  source = "../central/alerts.yml"
  bucket = google_storage_bucket.config_bucket.name
}

# Upload central/grafana/provisioning/datasources/prometheus.yml
resource "google_storage_bucket_object" "grafana_datasource" {
  name   = "central/grafana/provisioning/datasources/prometheus.yml"
  source = "../central/grafana/provisioning/datasources/prometheus.yml"
  bucket = google_storage_bucket.config_bucket.name
}
