
source "$(dirname "${BASH_SOURCE[0]}")/echoerr.sh"

get_HostName_User_of_Host_from() {
    local Host=$1; shift
    local from=$(realpath "$1"); shift
    local level=$1; shift

    export User=
    export HostName=

    local level_max=3
    if [[ ! $level ]]; then
        local level=0
    elif [[ $level -ge level_max ]]; then
        echoerr --no-exit "$0 '$Host' '$from' '$level': probably infinite recursion of Include, pwd: $(pwd)"
        return 1
    elif [[ ! -e $from ]]; then
        echoerr --no-exit "$0: file '$from' not found in $(pwd)"
        return 1
    fi

    pushd "$(dirname "$from")" >/dev/null # for 'Include' support

    local state=seek_host
    # https://www.cyberciti.biz/faq/unix-howto-read-line-by-line-from-file/
    while IFS= read -r line; do
        if [[ $line =~ ^Include[[:space:]]+([^[:space:]]+)$ ]]; then
            ${FUNCNAME[0]}  $Host "${BASH_REMATCH[1]}" $(( level + 1 )) \
                && break \
                || local state=seek_host
        elif [[ $line =~ ^Host[[:space:]]+$Host([[:space:]].*)?$ ]]; then
            local state=found_host
        elif [[ $state == found_host ]]; then
            if [[ $line =~ ^Host[[:space:]]+ ]]; then
                local state=seek_host
            elif [[ ! $User && $line =~ ^User[[:space:]]+([^[:space:]]+)$ ]]; then
                export User="${BASH_REMATCH[1]}"
                [[ $HostName ]] && break
            elif [[ ! $HostName && $line =~ ^Host[Nn]ame[[:space:]]+([^[:space:]]+)$ ]]; then
                export HostName="${BASH_REMATCH[1]}"
                [[ $User ]] && break
            fi
        fi
    done < "$(basename "$from")"

    popd >/dev/null

    [[ $HostName && $User ]] && return 0 || return 1
}
