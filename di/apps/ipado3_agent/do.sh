#!/usr/bin/env bash

source "$(realpath "$(dirname "${BASH_SOURCE[0]}")/../../sh/core/do.common.sh")"

src_rust_dir="$proj_dir/src/rust"
target="x86_64-pc-windows-gnu"
exe="target/$target/release/$app.exe"
dependencies_for_deploy=(
    "$src_rust_dir/$exe" 
)

case $cmd in
    build )
        set -e
        pushd "$src_rust_dir" 
        x rustup target add "$target"
        x sudo apt install -y mingw-w64
        x cargo build --release --target $target -p $app 
        x ls -lah $exe 
        target_files=()
        popd 
    ;;
    get-dependencies-for-deploy )
        echo "${dependencies_for_deploy[@]}"
    ;;
    deploy )
        [[ $dry_run ]] || set -e
        x $dry_run ssh "$host" "mkdir -p $proj/$kind/$app" 
        x $dry_run rsync -avz "${dependencies_for_deploy[@]}" $host:$proj/$kind/$app/ 
    ;;
    after-deploy )
            cat << EOM
== DID ${kind} DEPLOY ${app} TO ${host}
EOM
    ;;
esac

