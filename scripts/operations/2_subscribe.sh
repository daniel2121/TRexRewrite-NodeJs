#!/bin/bash
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

#Subscribe
printf "${GREEN}--- SUBSCRIBE ---\n"
printf "${BLUE}Sent: POST /subscriptions/$1/$2\n";
printf "${YELLOW}Received:\n"
OUTPUT=$(curl --fail --silent --show-error -H "Content-Type: application/json" -X POST http://127.0.0.1:8888/subscriptions/$1/$2)
CONN_ID=$(echo $OUTPUT | jq -r '.value')
echo "$OUTPUT"; echo
