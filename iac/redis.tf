resource "scaleway_redis_cluster" "main" {
  name         = "test_redis_basic"
  version      = "7.0.5"
  node_type    = "RED1-MICRO"
  user_name    = var.redis_username
  password     = var.redis_password
  cluster_size = 1
  tls_enabled  = "true"

  

  acl {
    ip          = "0.0.0.0/0"
    description = "Allow all"
  }
}
