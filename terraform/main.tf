terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 4.0"
    }
  }

  backend "gcs" {
    bucket = "terraform-state-kreivo-monitor" # Replace with your actual bucket name
    prefix = "terraform/state"
  }
}

provider "google" {
  project     = var.project_id
  region      = var.region
  zone        = var.zone
  credentials = var.credentials_file != null ? file(var.credentials_file) : null
}
