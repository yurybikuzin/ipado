#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/echoerr.sh"
source "$(dirname "${BASH_SOURCE[0]}")/get_HostName_User_of_Host_from.sh"

Host=$1; shift
from=$1; shift

[[ $Host ]] || echoerr "\$Host to find must be specified as first arg"
[[ $from ]] || {
    from=~/.ssh/config
    echo "\$from for not specified will be set to default value '$from'"
}

get_HostName_User_of_Host_from $Host $from && {
    echo "Host: $Host, HostName: $HostName, User: $User"
} || echoerr "'Host $Host' not found in '$from'"

