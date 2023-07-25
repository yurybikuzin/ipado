
di_dir="$(realpath "$(dirname "${BASH_SOURCE[0]}")/../..")"
proj_dir="$(realpath "$di_dir/..")"

source "$(dirname "${BASH_SOURCE[0]}")/x.sh"
source "$(dirname "${BASH_SOURCE[0]}")/echoerr.sh"

cmd=$1; shift

[[ $1 == --dry-run ]] && { dry_run=$1; shift; } || dry_run=

proj="$1"; shift
kind="$1"; shift
app="$1"; shift
if [[ $cmd == build ]]; then
    force="$1"; shift
else
    host="$1"; shift
    # User="$1"; shift
    # HostName="$1"; shift
fi
