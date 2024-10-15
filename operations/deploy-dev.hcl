job "margot-job-dev" {
  datacenters = ["ator-fin"]
  type = "batch"
  namespace = "ator-network"

  periodic {
    cron  = "*/10 * * * *" # Runs every 10 minutes
    prohibit_overlap = true
  }

  group "margot-job-dev" {

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

    task "margot-script" {
      driver = "docker"

      volume_mount {
        volume      = "margot-dev"
        destination = "/data"
        read_only   = false
      }

      config {
        image = "ghcr.io/anyone-protocol/margot:DEPLOY_TAG"
        command = "config"
        args = ["rejectbad", "89", "fp:9308F49A225022FA39011033E1C31EFF5B7B5000"]
      }
    }
  }
}
