#!/usr/bin/env bash
set -e
# ======== VERSION part

VERSION=2.4.1

# VERSION=2.4.1
    # added support for $DOMAIN_RE, $BACK_RE, $FRONT_RE in nginx_conf.sh
# VERSION=2.4.0
    # added support for following kinds: prod, dev, demo, rc
    # added variables PortOfKind, RoutePrefixOfKindRegex to be use in include.conf # see example in app/ipado_{back,front}
# VERSION=2.3.1 
    # extracted di/core/sh/ansi.sh
# VERSION=2.3.0 
    # added support fo systemd.log command
# VERSION=2.2.0 di/sh/core/.sh 
    # added support for systemd.service: see example in abc/di/{apps,dev,prod}/yandex_feed 
# VERSION=2.1.0 di/sh/core/.sh 
    # fixed help issue (-h not worked properly)
    # fixed Hostname issue (did recognize HostName only)
    # fixed error issue (did not echo whilst error in x())
    # added _ansi* constants for colored output
    # added colored output for x()
# VERSION=2.0.0 di/sh/core/.sh 
    # uses single di/apps/$app/do.sh instead of .build.sh/deploy.sh; 
    # parses ~/.ssh/config
# VERSION=1.1.0 used for cloudpayments: hosts/.nginx_conf.sh integrated into .core.sh by option --nginx-conf
# VERSION=1.0.0 used for w7, w7m

# ======== symlink part

if [[ $(basename "$0") != .sh ]]; then
    realpath=$(realpath -s "$0")
    "$(dirname "$realpath")"/../../sh/core/.sh --from-symlink "$realpath" "$@"
    exit 0
fi

# ======== no arg part

if [[ $# -eq 0 ]]; then
    source "$(dirname "${BASH_SOURCE[0]}")/help.sh"
fi

# ======== arg part

main_conf="/etc/nginx/main.conf"
include_conf="include.conf"
systemd_service="systemd.service"
systemd_log_sh="systemd.log.sh"
ssh_config=~/.ssh/config
core_sh="sh/core/.sh"
kinds=(dev demo rc prod)
ops=(deploy nginx_conf systemd systemd.log)

export _ansiReset=$'\e[0m' # https://superuser.com/questions/33914/why-doesnt-echo-support-e-escape-when-using-the-e-argument-in-macosx/33950#33950
export _ansiBold=$'\e[1m'
export _ansiRed=$'\e[31m'
export _ansiLightGray=$'\e[37m'
export _ansiDarkGray=$'\e[90m'
export _ansiDim=$'\e[2m'
export _ansiGreen=$'\e[32m'

source "$(dirname "${BASH_SOURCE[0]}")/ansi.sh"
source "$(dirname "${BASH_SOURCE[0]}")/x.sh"
source "$(dirname "${BASH_SOURCE[0]}")/echoerr.sh"
source "$(dirname "${BASH_SOURCE[0]}")/get_HostName_User_of_Host_from.sh"
source "$(dirname "${BASH_SOURCE[0]}")/find_in.sh"

arg="$1"; shift

[[ $1 == --dry-run ]] && { dry_run=$1; shift; } || { dry_run= ; }

case "$arg" in 
    --ver* | -v | -V )
        echo "$VERSION"
        exit 0
    ;;
    --help | -h )
        core_help
        exit 0
    ;;
    -c )
        x find -name '~*' -exec $0 -x $dry_run rm -rf {} \; 
        exit $?
    ;;
    -l )
        dir="$1"; shift
        [[ $dir ]] && pushd $dir > /dev/null

        level=
        search_for="$core_sh"
        level_max=2
        for ((i=0; i<=level_max; i++)); do
            if [[ -e "$search_for" ]]; then
                level=$i
                break
            fi
            search_for="../$search_for"
        done

        exitcode=0
        recursive_call=( "$search_for" "$arg" $dry_run )
        if [[ $level -eq 0 ]]; then
            for kind in "${kinds[@]}"; do
                "${recursive_call[@]}" $kind
            done
        elif [[ $level -eq 1 ]]; then
            kind=$(basename "$(pwd)")
            find_in $kind "${kinds[@]}" || echoerr '$(basename "$(pwd)") must be one of: '${kinds[@]}', not '$kind'; pwd: '$(pwd)

            find -mindepth 1 -maxdepth 1 -type d -exec "${recursive_call[@]}" {}  \;
        elif [[ $level -eq 2 ]]; then
            kind=$(basename "$(dirname "$(pwd)")")
            find_in $kind "${kinds[@]}" || echoerr '$(basename "$(dirname "$(pwd)")") must be one of: '${kinds[@]}', not '$kind'; pwd: '$(pwd)

            echo "pwd: $(pwd)"
            for file in nginx_conf.sh deploy.sh; do
                x $dry_run rm -rf "$file"
                x $dry_run ln -s "$search_for" "$file"
            done
        else
            echoerr --no-exit "can not find "$core_sh" in ascending folders up to level $level_max, pwd: $(pwd)" 
            exitcode=1
        fi

        [[ $dir ]] && popd > /dev/null

        exit $exitcode
    ;;
    -x )
        x $dry_run "$@"
        exit $?
    ;;
    systemd.log | systemd | nginx_conf | deploy | after-deploy )

        proj="$1"; shift
        kind="$1"; shift
        app="$1"; shift
        host="$1"; shift

        di_dir="$(dirname $(dirname $(dirname "$(realpath -s "$0")")))"

        # https://gist.github.com/mihow/9c7f559807069a03e302605691f85572
        get_HostName_User_of_Host_from "$host" "$ssh_config" \
            || echoerr "'Host $host' not found in '$ssh_config'"

        case $arg in
            systemd.log | systemd | nginx_conf ) source "$(dirname "${BASH_SOURCE[0]}")/$arg.sh" ;;
            * ) "$di_dir/apps/$app/do.sh" $arg $dry_run "$proj" "$kind" "$app" "$host" "$User" "$HostName" ;;
        esac

        exit $?
    ;;
    after-systemd.log | after-systemd | after-nginx_conf )
        exit 0
    ;;
    --from-symlink )
    ;;
    * )
        echoerr "unknown arg '$arg', check '$0 -h'"
    ;;
esac

source "$(dirname "${BASH_SOURCE[0]}")/from-symlink.sh"

