# margot

Margot is a Rust command line application using
[arti](https://gitlab.torproject.org/tpo/core/arti) that provides a series of
commands for the
[network health team](https://gitlab.torproject.org/tpo/network-health/team/).

## Current commands and subcommands

Note: part of this section should probably be moved into code documentation.

- `config`: `Create configuration entries`
  - `badexit <ticket_number> [filters]` : `Generate bad exit rule(s)` for the
    DirAuths. The parameters are a ticket number (in `bad-relay-reports` repo)
    and optionally some filters.

    A `filter` can be:
    - `addr:<IP address>`
    - `fl:<flag>`
    - `fp:<fingprint`
    - `p:<port>`
    - `v:<tor version>`
    A filter can be `exclude` (boolean), ie. not matching a filter, with the
    form `-:`, eg: `fl-:BADEXIT`

    Several `filter`s can be written one after another separated by an space.

    The output are the rules for `approved-routers.conf` in the form
    `!badexit <fp>`.

    Examples:
    - `config badexit 25`, output:

      ```bash

      [+] Rules for approved-routers.conf:

      ----
      # Ticket: <https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25>

      !badexit 000A10D43011EA4928A35F610405F92B4433B4DC
      [...]
      -----

      [+] Found 6810 relays: []
      ```

    - `config badexit 25 fl:BADEXIT`, output:

      ```bash
      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      !badexit 296B2178FD742AB35AB20C9ADF04D5DFD3D407EB
      [...]
      -----

      [+] Found 12 relays: [FindFilter { exclude: false, filter: Flags(BAD EXIT) }]

      ```

    - `config badexit 25 fp:FFFBFB50A83A414CC21B4CDA93A9674B004705E8` , output:

      ```bash

      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      !badexit FFFBFB50A83A414CC21B4CDA93A9674B004705E8
      -----

      [+] Found 1 relays: [FindFilter { exclude: false, filter: Fingerprint(Rsa("FFFBFB50A83A414CC21B4CDA93A9674B004705E8")) }]

      ```

    - `config badexit 25 addr:24.203.134.20`, output:

      ```bash

      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      !badexit FFFBFB50A83A414CC21B4CDA93A9674B004705E8
      -----

      [+] Found 1 relays: [FindFilter { exclude: false, filter: Address(V4(Ipv4Network { addr: 24.203.134.20, prefix: 32 })) }]

      ```

    - `config badexit 25 addr:24.203.134.2`, output:

      ```bash

      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      -----

      [+] Found 0 relays: [FindFilter { exclude: false, filter: Address(V4(Ipv4Network { addr: 24.203.134.2, prefix: 32 })) }]

      ```

    - `config badexit 25 n:nestor00patof`, output:

      ```bash

      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      !badexit FFFBFB50A83A414CC21B4CDA93A9674B004705E8
      -----

      [+] Found 1 relays: [FindFilter { exclude: false, filter: Nickname("nestor00patof") }]

      ```

    - `config badexit 25 p:8888`, output:

      ```bash

      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      !badexit 2BC31B73E0000B66981F7734D2B1F2C16C27D0BB
      !badexit 673510F48FA7EBE1C21A9A32566AB9B7AA8EFC48
      !badexit 94A8976E00C68ED23695D0668D87B3E7F126AF62
      -----

      [+] Found 3 relays: [FindFilter { exclude: false, filter: Port(8888) }]

      ```

    - `config badexit 25 v:0.4.7.0`, output:

      ```bash
      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      -----

      [+] Found 0 relays: [FindFilter { exclude: false, filter: Version("0.4.7.0") }]

      ```

  - `reject`: `Generate reject rule(s)` for the DirAuths. The parameters are
    a ticket number (in `bad-relay-reports` repo) and optionally some filters.

    It works as the previous command except that:
    - it also generates rules for `bad.conf` in the form `AuthDirReject <ip>`.
    - instead of generating rules for `approved-routers.conf` in the form
      `!badexit <fp>`, it generates rules like `!reject <fp>`.

    eg:
    - `config reject 25 p:8888`, output:

      ```bash
      [+] Rules for bad.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      AuthDirReject 65.109.16.131
      AuthDirReject 104.200.30.152
      AuthDirReject 2600:3c03::f03c:93ff:fecc:2d20
      AuthDirReject 155.248.213.203
      -----

      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      !reject 2BC31B73E0000B66981F7734D2B1F2C16C27D0BB
      !reject 673510F48FA7EBE1C21A9A32566AB9B7AA8EFC48
      !reject 94A8976E00C68ED23695D0668D87B3E7F126AF62
      -----

      [+] Found 3 relays: [FindFilter { exclude: false, filter: Port(8888) }]

      ```

  - `middleonly`: `Generate middleonly rule(s)` for the DirAuths.
  - The parameters are a ticket number (in `bad-relay-reports` repo) and
  - optionally some filters.

    It works as the previous commands except that:
    - it does not generate rules for `bad.conf`.
    - instead of generating rules for `approved-routers.conf` like
      `!badexit <fp>`, it generates rules like `!middleonly <fp>`.

    eg:

    - `config middleonly 25 fl:middleonly`, output:

      ```bash
      [+] Rules for approved-routers.conf:

      -----
      # Ticket: https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues/25
      !middleonly 006F965E89A9C3A61C9F08A6B31C28F66AF218FD
      !middleonly 0082C49022C0811D45620D408E068835E2BABA71
      !middleonly 081A5BAF9775499CAF7CCCAB2AF7765494F3B99F
      ```

- `count [filters]`: `Count relay(s) in the consensus`, optionally matching
  some `filter`.

  Filters are written as in the previous subcommands.

  eg:
  - `count fl:EXIT`, output:

     ```bash
     [+] 1603 relays match: FindFilter { exclude: false, filter: Flags(EXIT) }
     [+] 1603 relays matched all
     ```

  - `count pp:"accept 80,443"`, output:

    ```bash
    [+] 199 relays match: FindFilter { exclude: false, filter: PortPolicyFilter(PortPolicy { allowed: [PortRange { lo: 80, hi: 80 }, PortRange { lo: 443, hi: 443 }] }) }
    [+] 199 relays matched all
    ```

  - `pf:testdata/policy_accept.txt`, output:

    ```bash
    [+] 0 relays match: FindFilter { exclude: false, filter: PortPolicyFilter(PortPolicy { allowed: [PortRange { lo: 20, hi: 23 }, PortRange { lo: 43, hi: 43 }, PortRange { lo: 53, hi: 53 }, PortRange { lo: 79, hi: 81 }, PortRange { lo: 88, hi: 88 }, PortRange { lo: 110, hi: 110 }, PortRange { lo: 143, hi: 143 }, PortRange { lo: 194, hi: 194 }, PortRange { lo: 220, hi: 220 }] }) }
    [+] 0 relays matched all
    ```

- `find`: `Find relay(s) in the consensus`, optionally matching some `filter`s.

  Filters are written as in the previous subcommands.

  Same for `--help`.

  eg:
  - `find n:nestor00patof`, output:

    ```bash
    [+] Nickname: nestor00patof
    > Fingerprint: Rsa: $FFFBFB50A83A414CC21B4CDA93A9674B004705E8, Ed: X+VJlS224jbNRejibbCPRKgVO8G64vC8S6nPSubfSpI
    > Flags: STABLE | RUNNING | VALID | V2DIR
    > Weight: Measured(45)
    > Version: 0.4.7.8
    > ORPort(s): 24.203.134.20:1337
    > IPv4 Policy: reject 1-65535
    > IPv6 Policy: reject 1-65535
    > Family:
    ```

- `like <name>`: `Match alike relay(s) in the consensus`.
   Compute relays nicknames' Levenshtein distance and print the
  `Top 5 closest nicknames to <name>`.

  eg:
  - `like named`, output:

    ```bash
    [+] Getting relays from consensus
    [+] Computing nickname distances...
    :: 1000 relays processed
    :: 2000 relays processed
    :: 3000 relays processed
    :: 4000 relays processed
    :: 5000 relays processed
    :: 6000 relays processed
    [+] Top 5 closest nicknames to: named
    -> $18ef781925edf5463338ab450ef0c56c5caa2f1a: 2
    -> $0e9cf92a1840341aecb39d1f9700c1baea51c284: 2
    -> $a0296ddc9ec50aa42ed9d477d51dd4607d7876d3: 2
    -> $0af90285d6b3ecc52abfa3cb31404df6faf93134: 2
    -> $4a69f9226b6113b0681d5c93bbd60d5fccbe4817: 2

    ```

- `sybil`: `Sybil testing`
  - `exitpolicy`: `Inspect Exit Policies`
    - Matching Reduced Exit Policy and More
    - Not matching Reduced Exit Policy

    eg:
    `sybil exitpolicy`, output:

    ```bash
    [+] Matching Reduced Exit Policy and More: 'accept 1-24,26-118,120-134,140-444,446-1213,1215-65535'
    +-----------+-------------------------------------------+---------------------------------------------+----------+---------------------+
    | Nickname  | Rsa                                       | Ed                                          | Version  | ORPorts             |
    +-----------+-------------------------------------------+---------------------------------------------+----------+---------------------+
    | SkyLights | $B0E93B10BD817250A818ABF7F5C2444AF364DD67 | oaLl3WmRC+4mKCNAvl4q+ghBwxkBvPgpcnAmP5NrABo | 0.4.7.10 | 173.237.206.68:9001 |
    +-----------+-------------------------------------------+---------------------------------------------+----------+---------------------+
    ```

- `test`: `Run test(s) on one or many relay(s)`
  - `extend <filters>`: Circuit `Extend to a relay`, optionally matching
    some `filter`s.

    Filters are written as in the previous subcommands.

    Possible outputs:
    - `Unable to extend: <relay>`
    - `Successful one hop to: <relay>`
    - `No relays matching filters`

    eg:
    - `test extend fp:FFFBFB50A83A414CC21B4CDA93A9674B004705E8`, output:

      ```bash
      [+] Successful one hop to: nestor00patof - FFFBFB50A83A414CC21B4CDA93A9674B004705E8
      ```

    - `test extend fl:BADEXIT`, output:

      ```bash
      [..]
      [+] Successful one hop to: CGretski - 7ABA776A496C7B1D0C40F25ACA59F2EA60C3D429
      [-] Unable to extend: Problem building a circuit, while creating first hop with [87.120.37.231:9001 ed25519:qEHDLIxFvZ8FstcyXQY0gfqnpLVoEkjW8AkF64MdQAA $96733df529f50a69df592e4fcc116dc93832c91f]
      ```

## Developing

[Rust](https://www.rust-lang.org/tools/install)

`cargo build run -- <command>`
