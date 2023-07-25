
source "$(dirname "${BASH_SOURCE[0]}")/core/x.sh"

target_dir="$di_dir/~target/$app"
did_build="$target_dir/~did_build"
Makefile_build="$target_dir/~Makefile_build"
mkdir -p "$target_dir" || exit 1

start_prepare_Makefile_build() {
    [[ ${#target_files[@]} -gt 0 ]] || echoerr "not target_files"

    local did_build_dependencies=()
    for file in ${target_files[@]}; do
        did_build_dependencies+=( "$target_dir/$file" )
    done

    cat << EOM
# Autogenerated by "$0"
# To run this Makefile use following command:
# make -f "$Makefile"

SHELL=/usr/bin/env bash

$did_build: ${did_build_dependencies[@]}
	touch $did_build

EOM
}