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

      template {
        data = <<EOH
	      {{- range service "dir-auth-dev-control-port" }}
  	        DA_HOST="{{ .Address }}"
	      {{ end -}}
            EOH
        destination = "secrets/file.env"
        env         = true
      }

      volume_mount {
        volume      = "dir-auth-dev"
        destination = "/usr/src/app/anon-data"
        read_only   = false
      }

      config {
        volumes = ["local/approved-routers:/usr/src/app/approved-routers:ro"]
        image = "ghcr.io/anyone-protocol/margot:0d0025478266f5c5f105ef819a3b38fe2dd9a3ff"
      }

      template {
        change_mode = "noop"
        data        = <<EOH
!reject B6F95D3D76454610896D11ABA4734544B17E397C
        EOH
        destination = "local/approved-routers"
      }
    }
  }
}
