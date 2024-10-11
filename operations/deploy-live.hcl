job "margot-live" {
  datacenters = ["ator-fin"]
  type        = "service"
  namespace   = "ator-network"

  group "margot-live-group" {
    count = 7

    spread {
      attribute = "${node.unique.id}"
      weight    = 100
      target "067a42a8-d8fe-8b19-5851-43079e0eabb4" {
        percent = 14
      }
      target "16be0723-edc1-83c4-6c02-193d96ec308a" {
        percent = 14
      }
      target "e6e0baed-8402-fd5c-7a15-8dd49e7b60d9" {
        percent = 14
      }
      target "5ace4a92-63c4-ac72-3ed1-e4485fa0d4a4" {
        percent = 14
      }
      target "eb42c498-e7a8-415f-14e9-31e9e71e5707" {
        percent = 14
      }
      target "4aa61f61-893a-baf4-541b-870e99ac4839" {
        percent = 15
      }
      target "c2adc610-6316-cd9d-c678-cda4b0080b52" {
        percent = 15
      }
    }

    volume "margot-live" {
      type      = "host"
      read_only = false
      source    = "margot-live"
    }

    volume "margot-destination-live" {
      type      = "host"
      read_only = true
      source    = "margot-destination-live"
    }

    task "margot-live-task" {
      driver = "docker"

      env {
        INTERVAL_MINUTES = "60"
      }

      volume_mount {
        volume      = "margot-live"
        destination = "/usr/src/margot/data"
        read_only   = false
      }

      config {
        image   = "ghcr.io/anyone-protocol/margot-live:DEPLOY_TAG"
      }

      service {
        name     = "margot-live-task"
        tags     = ["logging"]
      }

      resources {
        cpu    = 1024
        memory = 3072
      }

    }
  }
}
