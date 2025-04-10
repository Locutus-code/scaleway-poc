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

variable "redis_username" {
  type = string
}

variable "redis_password" {
  type = string
}

output "producer_queue_url" {
  value = scaleway_mnq_sqs_queue.main.url
}

output "sqs_endpoint_url" {
  value = scaleway_mnq_sqs_queue.main.sqs_endpoint
}

output "redis_url" {
  value = "${scaleway_redis_cluster.main.public_network[0].ips[0]}:${scaleway_redis_cluster.main.public_network[0].port}"
}

// TODO:

// Async function

// Serverless SQL

// Application GW

// CDN

// S3 Assets

// REDIS
