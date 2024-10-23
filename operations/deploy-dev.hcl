job "margot-job-dev" {
  datacenters = ["ator-fin"]
  type = "batch"
  namespace = "ator-network"

  periodic {
    crons  = ["*/10 * * * *"] # Runs every 10 minutes
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

    volume "dir-auth-dev" {
      type      = "host"
      read_only = false
      source    = "dir-auth-dev"
    }

    task "margot-script" {
      driver = "docker"

      volume_mount {
        volume      = "dir-auth-dev"
        destination = "/usr/src/app/anon-data"
        read_only   = false
      }

      config {
        volumes = ["local/bad-relays:/usr/src/app/bad-relays:ro"]
        image = "ghcr.io/anyone-protocol/margot:DEPLOY_TAG"
      }

      template {
        change_mode = "noop"
        data        = <<EOH
F740DDDB1A6B536B5CC47111BFE4E2F7CF9B4C28
A7C2DF525D373D6A0C4F2540C3927ADF511124CC
EE2A621042994B29452C12FF3B6F62D9E957758C
        EOH
        destination = "local/bad-relays"
      }
    }
  }
}