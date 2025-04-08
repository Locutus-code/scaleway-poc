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

// Queue

// Async function

// Serverless SQL

// Application GW

// CDN

// S3 Assets

// REDIS

