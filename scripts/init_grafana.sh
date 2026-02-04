#!/usr/bin/env bash
set -x
set -eo pipefail

SCRIPT_PATH=$(dirname $0)

if ! [ -x "$(command -v docker)" ]; then
    echo >&2 "Error: docker is not installed"
    exit 1
fi

RUNNING_CONTAINER=$(docker ps --filter 'name=grafana' --format '{{.ID}}')

if [[ -n $RUNNING_CONTAINER ]]; then
    echo >&2 "grafana already running"
    exit 1
fi

if ! [[ -z "${SKIP_DOCKER} " ]]; then 
    docker run \
    	--name "grafana_$(date '+%s')" \
	-p 3000:3000 \
	-e GF_SECURITY_ADMIN_USER=admin \
	-e GF_SECURITY_ADMIN_PASSWORD=grafana \
	--net=host \
	-d \
        grafana/grafana
	fi
