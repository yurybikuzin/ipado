source "$(dirname "${BASH_SOURCE[0]}")/help.common.sh"
cat << EOM
DESCRIPTION:
    $op_sh (version $VERSION) - 
        deploy ~/$(realpath -s --relative-base ~ "$dir")'
        to following host(s): ${hosts[@]}
    IMPORTANT: it builds $app before deploy
$usage_options
EOM
exit 0
