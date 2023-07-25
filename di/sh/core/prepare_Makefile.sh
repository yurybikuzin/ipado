
source "$(dirname "${BASH_SOURCE[0]}")/echoerr.sh"

prepare_Makefile() {
    local Makefile="$1"; shift
    [[ $Makefile ]] || echoerr '$Makefile expected'

    local did_op="$1"; shift
    [[ $did_op ]] || echoerr '$did_op expected'

    local core_sh="$1"; shift
    [[ $core_sh ]] || echoerr '$core_sh expected'
    core_sh="$di_dir/$core_sh"

    local dependencies=( )

    source "$(dirname "${BASH_SOURCE[0]}")/prepare_Makefile.dependencies.$op.sh"

    {
    cat << EOM
# Autogenerated by "$core_sh"
# To run this Makefile use following command:
# make -f "$Makefile"

SHELL=/usr/bin/env bash

$did_op: ${dependencies[@]}
	"$core_sh" "$op" $dry_run "$proj" "$kind" "$app" "$host" 
	"$core_sh" -x $dry_run touch "$did_op"
	"$core_sh" "after-$op" $dry_run "$proj" "$kind" "$app" "$host" 

EOM
    } > "$Makefile"
}
