source "$(dirname "${BASH_SOURCE[0]}")/help.common.sh"
cat << EOM
DESCRIPTION:
    $op_sh (version $VERSION) - 
        deploy '~/$(realpath -s --relative-base ~ "$dir")' nginx conf injection 
        to following host(s): ${hosts[@]}
    IMPORTANT: it restarts nginx after deploy;
    you can check (and modify) '$app' nginx conf injection template in: 
        ~/$(realpath -s --relative-base ~ "$dir/../../apps/$app/$include_conf")
$usage_options
EOM
exit 0
