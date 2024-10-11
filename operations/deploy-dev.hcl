job "margot-dev" {
  datacenters = ["ator-fin"]
  type        = "service"
  namespace   = "ator-network"

  group "margot-dev-group" {
    count = 3

    spread {
      attribute = "${node.unique.id}"
      weight    = 100
      target "c8e55509-a756-0aa7-563b-9665aa4915ab" {
        percent = 34
      }
      target "c2adc610-6316-cd9d-c678-cda4b0080b52" {
        percent = 33
      }
      target "4aa61f61-893a-baf4-541b-870e99ac4839" {
        percent = 33
      }
    }

    volume "margot-dev" {
      type      = "host"
      read_only = false
      source    = "margot-dev"
    }

    volume "margot-destination-dev" {
      type      = "host"
      read_only = false
      source    = "margot-destination-dev"
    }

    task "margot-dev-task" {
      driver = "docker"

      volume_mount {
        volume      = "margot-dev"
        destination = "/usr/src/margot/data"
        read_only   = false
      }

      config {
        image   = "ghcr.io/anyone-protocol/margot-dev:DEPLOY_TAG"
      }

      service {
        name     = "margot-dev-task"
        tags     = ["logging"]
      }

      resources {
        cpu    = 1024
        memory = 2560
      }

    }

  }
}
