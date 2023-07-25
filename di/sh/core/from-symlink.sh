
source "$(dirname "${BASH_SOURCE[0]}")/read_hosts_from.sh"
source "$(dirname "${BASH_SOURCE[0]}")/find_in.sh"

[[ $1 ]] || echoerr "path to di/\$kind/\$app/\$op.sh is required"

op_sh="$1"; shift
dir="$(dirname "$op_sh")"
filename=$(basename "$op_sh")

di_dir="$dir/../.."
proj_dir="$di_dir/.."
op="${filename%.*}"

find_in $op "${ops[@]}" || echoerr "\$op must be one of: ${ops[@]}, not '$op'"
 
app="$(basename "$dir")"
kind="$(basename "$(dirname "$dir")")" # prod or dev
proj="$(basename "$(dirname "$(dirname "$(dirname "$dir")")")")" 

find_in $kind "${kinds[@]}" || echoerr "\$dir '$dir' seems to be a wrong path to app to deploy, 'cause \$kind must be one of: ${kinds[@]}, not '$kind'" 

force=
dry_run=
clean=
hosts_source="$di_dir/$kind/$app/hosts"
read_hosts_from "$hosts_source" \
    || echoerr "Failed to read hosts from '$from'"
hosts_to_be_used=()

for arg in "$@"; do
    case $arg in
        -c )
            clean=$arg
        ;;
        -f )
            force=$arg
        ;;
        --dry-run )
            dry_run=$arg
        ;;
        -h | --help )
            op_sh="~/$(realpath -s --relative-base ~ "$op_sh")"
            source "$(dirname "${BASH_SOURCE[0]}")/help.$op.sh"
        ;;
        -* )
            echoerr "unknown option '$arg'"
        ;;
        * )
            if [[ ! $clean ]]; then
                host="$arg"
                get_HostName_User_of_Host_from "$host" "$ssh_config" \
                    || echoerr "'Host $host' not found in '$ssh_config'"
                found=
                for h in ${hosts[@]}; do
                    [[ $h == $host ]] && { found=true; break; }
                done
                [[ $found ]] || echoerr "host '$host' is not specified for '$app' in '$hosts_source'"
                hosts_to_be_used+=( $arg )
            fi
        ;;
    esac
done

if [[ $clean ]]; then
    if [[ ${#hosts_to_be_used[@]} -eq 0 ]]; then
        x find "$dir" -name '~*' -exec $0 -x $dry_run rm -rf {} \; 
    else
        for host in "${hosts_to_be_used[@]}"; do
            host_dir="$dir/~$host"
            x find "$host_dir" -name '~*' -exec $0 -x $dry_run rm -rf {} \; 
        done
    fi
else
    [[ ${#hosts_to_be_used[@]} -eq 0 ]] && hosts_to_be_used=( "${hosts[@]}" )

    app_dir="$di_dir/apps/$app"
    app_do="$app_dir/do.sh"

    if [[ $op == deploy ]]; then
        "$app_do" build "$proj" "$kind" "$app" "$force" || echoerr "Failed to build $proj/$kind/$app"
        # echo 82: HERE 
    fi
    # echo 84: will prepare Makefile
    source "$(dirname "${BASH_SOURCE[0]}")/prepare_Makefile.sh"
    for host in "${hosts_to_be_used[@]}"; do
        host_dir="$dir/~$host"
        x mkdir -p "$host_dir"
        did_op="$host_dir/~did_$op"
        [[ $force ]] && rm -f "$host_dir/~did_$op"
        Makefile="$host_dir/~Makefile_$op"
        prepare_Makefile "$Makefile" "$did_op" "$core_sh"
        make -f "$Makefile" 
    done
fi
