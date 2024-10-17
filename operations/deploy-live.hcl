job "margot-job-live" {
  datacenters = ["ator-fin"]
  type = "batch"
  namespace = "ator-network"

  periodic {
    crons  = ["*/10 * * * *"] # Runs every 10 minutes
    prohibit_overlap = true
  }

  group "margot-job-live" {

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

    task "margot-script" {
      driver = "docker"

      volume_mount {
        volume      = "margot-live"
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
