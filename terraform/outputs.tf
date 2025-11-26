output "monitor_vm_public_ip" {
  description = "The public IP address of the monitoring instance"
  value       = google_compute_instance.monitor_vm.network_interface[0].access_config[0].nat_ip
}

output "grafana_url" {
  description = "URL to access Grafana"
  value       = "http://${google_compute_instance.monitor_vm.network_interface[0].access_config[0].nat_ip}:3000"
}

output "prometheus_remote_write_url" {
  description = "URL for Prometheus Remote Write"
  value       = "http://${google_compute_instance.monitor_vm.network_interface[0].access_config[0].nat_ip}:9091/api/v1/write"
}
