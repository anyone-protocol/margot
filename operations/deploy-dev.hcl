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
        volumes = ["local/bad-relays:/usr/src/margot/bad-relays:ro"]
        image = "ghcr.io/anyone-protocol/margot:3ffd572217a96d23e4c260ff0fe55b0c4ac6c0a6"
        command = "./target/x86_64-unknown-linux-gnu/release/margot"
        args = ["config", "rejectbad", "0", "ff:bad-relays"]
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
