resource "google_service_account" "monitor_sa" {
  account_id   = "kreivo-monitor-sa"
  display_name = "Kreivo Monitor Service Account"
}

# Grant access to read from the config bucket
resource "google_storage_bucket_iam_member" "monitor_sa_bucket_access" {
  bucket = google_storage_bucket.config_bucket.name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.monitor_sa.email}"
}

resource "google_compute_firewall" "monitor_allow_ingress" {
  name    = "kreivo-monitor-allow-ingress"
  network = "default"

  allow {
    protocol = "tcp"
    ports    = ["3000", "9091", "9093"]
  }

  source_ranges = var.allowed_source_ranges
  target_tags   = ["kreivo-monitor"]
}

resource "google_compute_instance" "monitor_vm" {
  name         = "kreivo-monitor-central"
  machine_type = var.machine_type
  zone         = var.zone

  tags = ["kreivo-monitor", "http-server", "https-server"]

  boot_disk {
    initialize_params {
      image = "ubuntu-os-cloud/ubuntu-2204-lts"
      size  = 20
    }
  }

  network_interface {
    network = "default"
    access_config {
      # Ephemeral public IP
    }
  }

  service_account {
    email  = google_service_account.monitor_sa.email
    scopes = ["cloud-platform"]
  }

  metadata = {
    config-bucket  = google_storage_bucket.config_bucket.name
    startup-script = file("${path.module}/scripts/startup.sh")
  }
}
