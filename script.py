import re

# Step 1: Load bad fingerprints from the 'bad-relays' file
def load_bad_fingerprints(file_path):
    with open(file_path, 'r') as file:
        return {line.strip() for line in file.readlines()}

# Step 2: Find nicknames by fingerprints in cached-extrainfo
def find_nicknames_by_fingerprints(extrainfo_file, bad_fingerprints):
    nicknames = {}
    current_nickname = None
    current_fingerprint = None

    with open(extrainfo_file, 'r') as file:
        for line in file:
            # Match relay nickname and fingerprint
            fingerprint_match = re.search(r"^extra-info (\S+) ([A-F0-9]+)$", line)
            if fingerprint_match:
                current_nickname = fingerprint_match.group(1)
                current_fingerprint = fingerprint_match.group(2)
                # If the fingerprint is bad, save the nickname
                if current_fingerprint in bad_fingerprints:
                    nicknames[current_fingerprint] = current_nickname

    return nicknames

# Step 3: Find both IPv4 and IPv6 addresses by nicknames in cached-consensus
def find_ips_by_nicknames(consensus_file, nicknames):
    ips = {}
    current_nickname = None
    current_ipv4 = None
    current_ipv6 = None

    with open(consensus_file, 'r') as file:
        for line in file:
            # Match relay line to extract nickname and IPv4 address
            relay_match = re.search(r"^r (\S+) \S+ \S+ \S+ \S+ (\S+) \S+ \S+", line)  # r <nickname> <identity> <descriptor digest> <date> <time> <ipv4> <orport> <dirport>
            if relay_match:
                current_nickname = relay_match.group(1)
                current_ipv4 = relay_match.group(2)

                # If the nickname matches one of our bad nicknames, collect the IPv4
                if current_nickname in nicknames.values():
                    ips[current_nickname] = {'ipv4': current_ipv4, 'ipv6': None}

            # Match IPv6 address in the 'a' line (IPv6 address format)
            ipv6_match = re.search(r"^a \[(\S+)\]", line)
            if ipv6_match and current_nickname in nicknames.values():
                current_ipv6 = ipv6_match.group(1)
                ips[current_nickname]['ipv6'] = current_ipv6

    return ips

# Step 4: Generate reject rules for anonrc
def generate_reject_rules(ips):
    reject_rules = []
    for nickname, ip_data in ips.items():
        if ip_data['ipv4']:
            reject_rules.append(f"AuthDirReject {ip_data['ipv4']}")
        if ip_data['ipv6']:
            reject_rules.append(f"AuthDirReject {ip_data['ipv6']}")
    return reject_rules

# Example usage
if __name__ == "__main__":
    # Step 1: Load bad fingerprints from 'bad-relays' file
    bad_fingerprints = load_bad_fingerprints('bad-relays')  # Updated file name

    # Step 2: Find nicknames from cached-extrainfo
    nicknames = find_nicknames_by_fingerprints('anon-data/cached-extrainfo', bad_fingerprints)

    # Step 3: Find corresponding IPs in cached-consensus
    ips = find_ips_by_nicknames('anon-data/cached-consensus', nicknames)

    # Step 4: Generate reject rules for both IPv4 and IPv6 addresses
    reject_rules = generate_reject_rules(ips)

    # Write rules to the new output file (anonrc-reject-rules)
    with open('anon-data/anonrc-reject-rules', 'w') as f:
        for rule in reject_rules:
            f.write(f"{rule}\n")

    print(f"Reject rules written for {len(ips)} relays.")
