# Central Monitoring Terraform Module

This Terraform module deploys a central monitoring stack (Prometheus, Alertmanager, Grafana) to Google Cloud Platform (GCP) using a Compute Engine VM and Docker Compose.

## Architecture

*   **Compute Engine VM**: Runs Ubuntu 22.04 LTS.
*   **Docker Compose**: Orchestrates Prometheus, Alertmanager, and Grafana containers.
*   **GCS Bucket**: Stores configuration files (`central/` directory) which are downloaded by the VM on startup.
*   **Firewall Rules**: Allows ingress on ports 3000 (Grafana), 9091 (Prometheus Remote Write), and 9093 (Alertmanager).

## Prerequisites

1.  **Terraform** >= 1.0
2.  **Google Cloud SDK** (`gcloud`) installed and authenticated.
3.  A **GCP Project** with billing enabled.
4.  **APIs Enabled**:
    *   Compute Engine API
    *   Cloud Storage API

## Usage

### 1. Initialize

**Important**: The GCS backend cannot read the `credentials_file` variable. You must set the environment variable for initialization:

```bash
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/key.credentials.json"
terraform init
```

### 2. Configure Variables

Create a `terraform.tfvars` file to specify your project details:

```hcl
project_id       = "your-gcp-project-id"
region           = "us-central1"
zone             = "us-central1-a"

# Optional: Path to Service Account Key JSON
# credentials_file = "/path/to/service-account-key.json"

# Optional: Restrict access (defaults to 0.0.0.0/0)
# allowed_source_ranges = ["1.2.3.4/32"]
```

### 3. Deploy

Preview the changes:

```bash
terraform plan
```

Apply the configuration:

```bash
terraform apply
```

### 4. Access Services

After a successful apply, Terraform will output the connection details:

*   **Grafana**: `http://<VM_IP>:3000` (Default credentials: `admin` / `admin`)
*   **Prometheus**: `http://<VM_IP>:9091` (Remote Write Receiver)

## Maintenance

### Updating Configuration
To update the monitoring configuration (e.g., `prometheus.yml` or `alerts.yml`):
1.  Modify the files in the `../central/` directory.
2.  Run `terraform apply` to upload the new files to the GCS bucket.
3.  **Restart the Stack**: The VM does not automatically reload changes. You must either:
    *   SSH into the VM and restart Docker Compose.
    *   Or, taint the instance to force a replacement: `terraform taint google_compute_instance.monitor_vm && terraform apply`.

## CI/CD

This module includes a GitHub Actions workflow in `.github/workflows/terraform.yml`.

### Secrets Required
*   `GCP_PROJECT_ID`
*   `GCP_SA_KEY` (JSON key content)

### Backend
The `main.tf` is configured to use a GCS backend. You must create this bucket **before** running the pipeline:
```bash
gsutil mb -l us-central1 gs://terraform-state-kreivo-monitor
```

## Inputs

| Name | Description | Type | Default |
|------|-------------|------|---------|
| `project_id` | GCP Project ID | `string` | **Required** |
| `region` | GCP Region | `string` | `us-central1` |
| `zone` | GCP Zone | `string` | `us-central1-a` |
| `machine_type` | VM Machine Type | `string` | `e2-medium` |
| `credentials_file` | Path to GCP SA Key JSON | `string` | `null` |
| `allowed_source_ranges` | Allowed Ingress IPs | `list(string)` | `["0.0.0.0/0"]` |

## Outputs

| Name | Description |
|------|-------------|
| `monitor_vm_public_ip` | Public IP of the VM |
| `grafana_url` | URL for Grafana |
| `prometheus_remote_write_url` | URL for Prometheus Remote Write |
