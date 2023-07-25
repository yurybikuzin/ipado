#!/usr/bin/env bash

# EXAMPLE: ipado3_back/remove.service.sh v9z ipado2_back_dev

set -e
function remove_service_at_host() {
    host=${1?host required as first argument}
    service_name=${2?sevice_name required as second argument}
    ssh $host "
        sudo systemctl stop $service_name
        sudo systemctl disable $service_name
        sudo rm /etc/systemd/system/$service_name.service
        sudo systemctl daemon-reload
        sudo systemctl reset-failed
    "
}
remove_service_at_host "$@"
