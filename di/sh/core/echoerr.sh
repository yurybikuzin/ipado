echoerr() {
    # https://stackoverflow.com/questions/3055755/equivalent-of-file-and-line-in-bash/3056595#3056595
    local no_exit=
    local exit_code=1
    local bash_source=1
    while [[ $# -gt 0 ]]; do
        case $1 in 
            --no-exit|--noexit )
                no_exit=$1; shift
            ;;
            --exit-code|--exit_code|--exitcode )
                shift; exit_code=$1; shift
            ;;
            --bash-source|--bash_source|--bashsource )
                shift; bash_source=$1; shift
            ;;
            * )
                break
            ;;
        esac
    done
    echo "${_ansiRed}${_ansiBold}ERR: ${_ansiLightGray}$@${_ansiReset} at ${BASH_SOURCE[$bash_source]}:${BASH_LINENO[$bash_source]}" 1>&2
    # echo no_exit: $no_exit, exit_code: $exit_code
    [[ $no_exit ]] || exit $exit_code
    # echo echoerr: HERE
}

unreachabe() {
    echo "ERR: unreachable at ${BASH_SOURCE[1]}:${BASH_LINENO[0]}" 1>&2
}

