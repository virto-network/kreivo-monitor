variable "project_id" {
  description = "The GCP Project ID"
  type        = string
}

variable "region" {
  description = "The GCP Region"
  type        = string
  default     = "us-central1"
}

variable "zone" {
  description = "The GCP Zone"
  type        = string
  default     = "us-central1-a"
}

variable "machine_type" {
  description = "The machine type for the monitoring instance"
  type        = string
  default     = "e2-medium"
}

variable "allowed_source_ranges" {
  description = "List of IP ranges allowed to access the monitoring stack"
  type        = list(string)
  default     = ["0.0.0.0/0"] # Open to world by default, user should restrict this
}

variable "credentials_file" {
  description = "Path to the GCP service account key JSON file"
  type        = string
  default     = null
}
