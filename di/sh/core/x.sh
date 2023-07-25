x() {
    dry_run=
    exit_on_error=
    while [[ $# -gt 0 ]]; do
        case $1 in 
            --dry-run )
                dry_run=$1; shift
            ;;
            --exit-on-error )
                exit_on_error=$1; shift
            ;;
            * )
                break
            ;;
        esac
    done

    if [[ $dry_run ]]; then
        shift
		echo "${_ansiDarkGray}${_ansiBold}DRY RUN: ${_ansiLightGray}$@${_ansiReset}"
    else
		will "$@"
        "$@"
        local exitcode=$?
        if [[ $exitcode -eq 0 ]]; then
            did "$@"
        else
            if [[ $exit_on_error ]] || [[ $- =~ e ]]; then # https://superuser.com/questions/997755/is-it-possible-to-check-whether-e-is-set-within-a-bash-script
                echoerr --bash-source 2 "$@"
            else 
                echo "${_ansiGreen}${_ansiYellow}WARN: ${_ansiLightGray}$@${_ansiReset} did exit with code $exitcode"
                # echoerr --bash-source 2 --no-exit "$@"
            fi
        fi
        return $exitcode
    fi
}

will() {
	echo "${_ansiDarkGray}${_ansiBold}WILL ${_ansiLightGray}$@${_ansiDarkGray} . . .${_ansiReset}"
}

did() {
	echo "${_ansiGreen}${_ansiBold}OK: ${_ansiLightGray}$@${_ansiReset}"
}
