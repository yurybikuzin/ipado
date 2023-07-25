
source "$(dirname "${BASH_SOURCE[0]}")/echoerr.sh"

read_hosts_from() {
    local verbose=
    [[ $1 == --verbose ]] && { verbose=$1; shift; }

    local from=$(realpath "$1"); shift
    local level=$1; shift
 
    local level_max=3
    if [[ ! $level ]]; then
        level=0
        hosts=()
    elif [[ $level -ge level_max ]]; then
        echoerr --no-exit "$0 '$Host' '$from' '$level': probably infinite recursion of Include, pwd: $(pwd)"
        return 1
    elif [[ ! -e $from ]]; then
        echoerr --no-exit "$0: file '$from' not found in $(pwd)"
        return 1
    fi

    pushd "$(dirname "$from")" >/dev/null # for 'Include' support

    # https://www.cyberciti.biz/faq/unix-howto-read-line-by-line-from-file/
    while IFS= read -r line; do
        if [[ $line =~ ^Include[[:space:]]+([^[:space:]]+)$ ]]; then
            local include="${BASH_REMATCH[1]}"
            [[ $verbose ]] && echo "will include '$include'"
            ${FUNCNAME[0]} "$include" $(( level + 1 ))
        elif [[ $line =~ ^[[:space:]]*([\.[:alnum:]_-]+) ]]; then
            hosts+=( "${BASH_REMATCH[1]}" )
        elif [[ $verbose ]]; then
            echo "$0: skipped '$line'"
        fi
    done < "$(basename "$from")"

    popd >/dev/null
}
