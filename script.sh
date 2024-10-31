#!/bin/bash

# Step 1: Copy approved-routers to the Anon data directory (force replacement)
cp -f approved-routers /usr/src/app/anon-data/approved-routers

echo $DA_HOST

# Step 2: Authenticate and send reload signal to ControlPort

COOKIE=$(hexdump -v -e '/1 "%02x"' /usr/src/app/anon-data/control_auth_cookie | tr -d '\n')

echo -e "authenticate ${COOKIE}\nsignal reload\nQUIT\n" | nc $DA_HOST 9051

echo "Configuration reloaded and approved-routers file updated."
