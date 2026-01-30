#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v docker)" ]; then
    echo >&2 "Error: docker is not installed"
    exit 1
fi

RUNNING_CONTAINER=$(docker ps --filter 'name=jaeger' --format '{{.ID}}')

if [[ -n $RUNNING_CONTAINER ]]; then
    echo >&2 "jaeger already running"
    exit 1
fi

if ! [[ -z "${SKIP_DOCKER} " ]]; then 
    docker run \
	-p 6831:6831/udp \
	-p 6832:6832/udp \
	-p 16686:16686 \
	-p 4317:4317 \
    	--name "jaeger_$(date '+%s')" \
	-d \
        jaegertracing/all-in-one:latest
fi
