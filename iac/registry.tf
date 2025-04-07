resource "scaleway_registry_namespace" "main" {
  name        = "poc-registry"
  description = "container registry"
  is_public   = false
}
