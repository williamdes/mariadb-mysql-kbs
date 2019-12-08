#!/bin/bash

ME=$(dirname $0)

# Load up .env
set -o allexport
[[ -f $ME/.env ]] && source $ME/.env
set +o allexport

if [ -z "${GITHUB_TOKEN}" ]; then
    echo "GitHub token missing"
    exit 1
fi

DISPATCH_NAME="$1"

if [ -z "${DISPATCH_NAME}" ]; then
    echo "Dispatch name is missing"
    exit 1
fi

curl --request POST \
    --url https://api.github.com/repos/williamdes/mariadb-mysql-kbs/dispatches \
    --header 'accept: application/vnd.github.everest-preview+json' \
    --header 'authorization: token '${GITHUB_TOKEN}'' \
    --header 'content-type: application/json' \
    --data '{ "event_type": "'${DISPATCH_NAME}'" }'
