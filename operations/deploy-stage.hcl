job "margot-stage" {
  datacenters = ["ator-fin"]
  type        = "service"
  namespace   = "ator-network"

  spread {
    attribute = "${node.unique.id}"
    weight    = 100
    target "c8e55509-a756-0aa7-563b-9665aa4915ab" {
      percent = 14
    }
    target "c2adc610-6316-cd9d-c678-cda4b0080b52" {
      percent = 43
    }
    target "4aa61f61-893a-baf4-541b-870e99ac4839" {
      percent = 43
    }
  }

  group "margot-stage-group" {
    count = 3

    volume "margot-stage" {
      type      = "host"
      read_only = false
      source    = "margot-stage"
    }

    volume "margot-destination-stage" {
      type      = "host"
      read_only = true
      source    = "margot-destination-stage"
    }

    network {
      mode = "bridge"

      port "http-port" {
        static = 9177
      }

      port "orport" {
        static = 19101
      }

      port "control-port" {
        static = 19151
        host_network = "wireguard"
      }
    }

    task "margot-stage-task" {
      driver = "docker"

      volume_mount {
        volume      = "margot-stage"
        destination = "/usr/src/margot/data"
        read_only   = false
      }

      config {
        image   = "ghcr.io/anyone-protocol/margot-stage:DEPLOY_TAG"
      }

      service {
        name     = "margot-stage-task"
        tags     = ["logging"]
      }

      resources {
        cpu    = 1024
        memory = 2560
      }

    }

  }

  group "margot-stage-group-2" {
    count = 2

    constraint {
      operator = "distinct_hosts"
      value    = "true"
    }

    volume "margot-stage-2" {
      type      = "host"
      read_only = false
      source    = "margot-stage-2"
    }

    volume "margot-destination-stage-2" {
      type      = "host"
      read_only = true
      source    = "margot-destination-stage-2"
    }

    task "margot-stage-task" {
      driver = "docker"

      volume_mount {
        volume      = "margot-stage-2"
        destination = "/usr/src/margot/data"
        read_only   = false
      }

      config {
        image   = "ghcr.io/anyone-protocol/margot-stage:DEPLOY_TAG"
      }

      service {
        name     = "margot-stage-task-2"
        tags     = ["logging"]
      }

      resources {
        cpu    = 1024
        memory = 2560
      }

    }

  }

  group "margot-stage-group-3" {
    count = 2

    constraint {
      operator = "distinct_hosts"
      value    = "true"
    }

    volume "margot-stage-3" {
      type      = "host"
      read_only = false
      source    = "margot-stage-3"
    }

    volume "margot-destination-stage-3" {
      type      = "host"
      read_only = true
      source    = "margot-destination-stage-3"
    }

    task "margot-stage-task" {
      driver = "docker"

      volume_mount {
        volume      = "margot-stage-3"
        destination = "/usr/src/margot/data"
        read_only   = false
      }

      config {
        image   = "ghcr.io/anyone-protocol/margot-stage:DEPLOY_TAG"
      }

      service {
        name     = "margot-stage-task-3"
        tags     = ["logging"]
      }

      resources {
        cpu    = 1024
        memory = 2560
      }

    }
  }
}
