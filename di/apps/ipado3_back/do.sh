#!/usr/bin/env bash

source "$(realpath "$(dirname "${BASH_SOURCE[0]}")/../../sh/core/do.common.sh")"

src_rust_dir="$proj_dir/src/rust"
target="x86_64-unknown-linux-musl"
exe="target/$target/release/$app"
dependencies_for_deploy=(
    "$src_rust_dir/$exe" 
    "$src_rust_dir/$app/.env"
    "$src_rust_dir/$app/settings.toml"
    "$src_rust_dir/$app/sales@yury.bikuzin.42.json"
)

case $cmd in
    build )
        set -e
        pushd "$src_rust_dir" 
        x rustup target add x86_64-unknown-linux-musl
        x sudo apt install -y musl-tools
        x cargo build --release --target $target -p $app 
        x ls -lah $exe 
        popd 
    ;;
    get-dependencies-for-deploy )
        echo "${dependencies_for_deploy[@]}"
    ;;
    deploy )
        [[ $dry_run ]] || set -e
        x $dry_run $src_rust_dir/$exe -w "$src_rust_dir/$app" -t 
        x $dry_run ssh "$host" "mkdir -p $proj/$kind/$app" 
        x $dry_run rsync -avz "${dependencies_for_deploy[@]}" $host:$proj/$kind/$app/ 
    ;;
    after-deploy )
        # if [[ -e $di_dir/$kind/$app/~$host/~did_systemd ]]; then
        #     cmd="sudo systemctl restart ${app}_$kind && sudo systemctl enable ${app}_$kind"
        service_name="${app}_$kind"
        if [[ $(ssh $host "ls /etc/systemd/system/${service_name}.service") ]]; then
            cmd="sudo systemctl restart ${service_name} && sudo systemctl enable ${service_name}"
            x $dry_run ssh $host "cd $proj/$kind/$app/ && $cmd"
            url="https://ipado.ru"
            route=/$app
            prefix=
            if [[ $kind == 'prod' ]]; then
                prefix=""
            else
                prefix="/$kind"
            fi
            url="$url$prefix$route/health"
            x $dry_run curl "$url"
            cat << EOM
== DID DEPLOY AND $cmd
EOM
        elif [[ -e $di_dir/apps/$app/systemd.service ]]; then
            cat << EOM
== AFTER DEPLOY NOTE:
    run $di_dir/$kind/$app/systemd.sh
    OR
    Enter to '$host' via ssh and run 'cd $proj/$kind/$app && ./$app server' in tmux session
    To leave ssh session use Enter-tilda-dot sequence (Enter ~ .)
EOM
        else
            ls 
            cat << EOM
== AFTER DEPLOY NOTE:
    Enter to '$host' via ssh and run 'cd $proj/$kind/$app && ./$app server' in tmux session
    To leave ssh session use Enter-tilda-dot sequence (Enter ~ .)
EOM
        fi
    ;;
esac

