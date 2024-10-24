#!/bin/bash

# Step 1: Copy approved-routers to the Anon data directory (force replacement)
cp -f approved-routers /usr/src/app/anon-data/approved-routers

printenv

echo $DA_HOST

# Step 2: Authenticate and send reload signal to ControlPort
echo -e "authenticate \"\"\nsignal reload\nQUIT\n" | nc -v 127.0.0.1 9051

echo "Configuration reloaded and approved-routers file updated."
