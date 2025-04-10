resource "scaleway_function_namespace" "main" {
  name        = "scaleway-poc-function-namespace"
  description = "function namespace"
}

resource "scaleway_function" "main" {
  namespace_id = scaleway_function_namespace.main.id
  name         = "queue-consumer"
  runtime      = "rust185"
  handler      = "Handle"
  privacy      = "private"

  environment_variables = {
    "ENDPOINT_URL" : "${scaleway_mnq_sqs_queue.main.sqs_endpoint}",
    "PRODUCER_QUEUE_URL" : "${scaleway_mnq_sqs_queue.main.url}"
  }
  secret_environment_variables = {
    "AWS_SECRET_ACCESS_KEY" : "${scaleway_mnq_sqs_credentials.main.access_key}"
    "AWS_ACCESS_KEY_ID" : "${scaleway_mnq_sqs_credentials.main.secret_key}"
  }
}

resource "scaleway_function_trigger" "main" {
  function_id = scaleway_function.main.id
  name        = "sqs-queue-trigger"
  sqs {
    project_id = scaleway_mnq_sqs.main.project_id
    queue      = scaleway_mnq_sqs_queue.main.name
    region     = scaleway_mnq_sqs.main.region
  }
}
