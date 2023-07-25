#!/usr/bin/env bash
set -e

source "$(dirname "$0")/read_hosts_from.sh"
source "$(dirname "$0")/echoerr.sh"

verbose=
[[ $1 == --verbose ]] && { verbose=$1; shift; }

from=$1; shift

[[ $from ]] || { 
    echoerr "\$from be specified as first arg"
    exit 1
}

hosts=(some thing)
read_hosts_from $verbose "$from" && {
    echo "hosts: ${hosts[@]}"
} || {
    echoerr "failed to read hosts from '$from'"
    exit 1
}


