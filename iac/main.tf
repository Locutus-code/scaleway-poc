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

// TODO:

// CDN

// S3 Assets

// Application GW

// BE API Container

// Queue

// Async function

// Serverless SQL

// REDIS

