terraform {
  required_providers {
    helm = {
      source  = "hashicorp/helm"
      version = "2.16.1"
    }
  }
}

provider "helm" {
  # Configuration options
}

resource "helm_release" "nginx_ingress" {
  name  = "code-challenge"
  chart = "../helm/code-challenge"

  set {
    name  = "image.tag"
    value = var.docker_image_tag
  }
}
