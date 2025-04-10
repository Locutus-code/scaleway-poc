terraform {
  required_providers {
    scaleway = {
      source = "scaleway/scaleway"
    }
  }
  required_version = ">= 0.13"
}

variable "project_id" {
  type        = string
  description = ""
}

output "producer_queue_url" {
  value = scaleway_mnq_sqs_queue.main.url
}

output "sqs_endpoint_url" {
  value = scaleway_mnq_sqs_queue.main.sqs_endpoint
}

// TODO:

// Async function

// Serverless SQL

// Application GW

// CDN

// S3 Assets

// REDIS
