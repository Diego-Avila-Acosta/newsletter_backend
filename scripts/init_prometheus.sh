#!/usr/bin/env bash
set -x
set -eo pipefail

SCRIPT_PATH=$(dirname $0)
PROM_CONFIG_PATH="${PROM_CONFIG_PATH:="${SCRIPT_PATH}/config/prometheus.yml"}"

if ! [ -x "$(command -v docker)" ]; then
    echo >&2 "Error: docker is not installed"
    exit 1
fi

RUNNING_CONTAINER=$(docker ps --filter 'name=prometheus' --format '{{.ID}}')

if [[ -n $RUNNING_CONTAINER ]]; then
    echo >&2 "prometheus already running"
    exit 1
fi

if ! [ -f "${PROM_CONFIG_PATH}" ]; then
	echo "Prometheus configuration file not found"
	echo "Please, provide a config file before running this script"
	exit 1
fi

if ! [[ -z "${SKIP_DOCKER} " ]]; then 
    docker run \
	-v ${PROM_CONFIG_PATH}:/etc/prometheus/prometheus.yml \
    	--name "prometheus_$(date '+%s')" \
	--net=host \
	-d \
        prom/prometheus
fi
