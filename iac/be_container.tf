resource "scaleway_container_namespace" "main" {
  name        = "backend"
  description = "backend"
}

resource "scaleway_container" "main" {
  name           = "backend-container"
  description    = "backend container"
  namespace_id   = scaleway_container_namespace.main.id
  registry_image = "${scaleway_container_namespace.main.registry_endpoint}/scaleway-be:latest"
  port           = 8000
  cpu_limit      = 140
  memory_limit   = 256
  min_scale      = 1
  max_scale      = 1
  timeout        = 60
  privacy        = "public"
  protocol       = "http1"
  deploy         = true

  environment_variables = {
    "SQS_ENDPOINT_URL" : "${scaleway_mnq_sqs_queue.main.sqs_endpoint}",
    "PRODUCER_QUEUE_URL" : "${scaleway_mnq_sqs_queue.main.url}"
  }
  secret_environment_variables = {
  }

  health_check {
    http {
      path = "/health"
    }
    interval          = "10s"
    failure_threshold = 10
  }
}
