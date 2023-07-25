
source "$(dirname "${BASH_SOURCE[0]}")/core/echoerr.sh"

check_utils() {
    [[ $# -gt 0 ]] || echoerr "no utils"
    for util in "$@"; do
        version_file="$target_dir/~$util.version"
        dependencies_from_utils+=("$version_file")
        cat << EOM
$version_file:
	$util --version | tee "$version_file" 

EOM
    done
}
