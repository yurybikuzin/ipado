
line="include /home/$User/$proj/$kind/$app/$include_conf;"

if [[ $kind == "dev" ]]; then
    kind_01=0
else 
    kind_01=1
fi

# https://gist.github.com/mihow/9c7f559807069a03e302605691f85572
if [[ -f "$di_dir/apps/$app/.env" ]]; then 
    export APP=$app
    export $(echo $(cat "$di_dir/apps/$app/.env" | sed 's/#.*//g'| xargs) | envsubst)
    if [[ $ENV_PATH ]]; then
        if [[ -f "$di_dir/../$ENV_PATH" ]]; then
            echo ENV_PATH: $ENV_PATH
            export $(echo $(cat "$di_dir/../$ENV_PATH" | sed 's/#.*//g'| xargs) | envsubst)
            echo PORT: $PORT
            echo PORT_DEV: $PORT_DEV
            echo PORT_DEMO: $PORT_DEMO
            echo ROUTE: $ROUTE
            echo DOMAIN: $DOMAIN
            echo DOMAIN_RE: $DOMAIN_RE
            echo BACK: $BACK
            echo SERVER_USER: $SERVER_USER
        else
            echo WARN: no "$di_dir/../$ENV_PATH"
        fi
    else 
        echo WARN: no ENV_PATH
    fi
else 
    echo WARN: no "$di_dir/apps/$app/.env"
fi

route_prefix_of_kind=
if [[ $kind == 'prod' ]]; then
    route_prefix_of_kind=""
else
    route_prefix_of_kind="/$kind"
fi
echo "RoutePrefixOfKind: $route_prefix_of_kind"

function join_by { # https://stackoverflow.com/questions/1527049/how-can-i-join-elements-of-an-array-in-bash
    local d=${1-} f=${2-}
    if shift 2; then
        printf %s "$f" "${@/#/$d}"
    fi
}
function get_route_prefix_of_kind_regex {
    local kind=${1-}
    if shift 1; then
        case $kind in 
            prod ) echo '(?!/(?:'$(join_by '|' $( printf -- '%s\n' "${kinds[@]}" | grep -v prod ) )'))' ;;
            * ) echo "/$kind" ;;
        esac
    fi
}
route_prefix_of_kind_regex="$(get_route_prefix_of_kind_regex $kind)"
echo "RoutePrefixOfKindRegex: $route_prefix_of_kind_regex"
# echo route_prefix_of_kind_regex

case $kind in 
    prod ) port_of_kind=PORT ;;
    * ) port_of_kind=PORT_${kind^^} ;;
esac

echo "port_of_kind: $port_of_kind"
port_of_kind=${!port_of_kind}
echo "PortOfKind: ${port_of_kind:=0}"
echo "ROUTE: $ROUTE"
echo "DOMAIN: $DOMAIN"
if [[ $GROUP ]]; then
    echo use APP_GROUP instead of GROUP
    exit 1
fi
echo "APP_GROUP: $APP_GROUP"
echo "SERVER_USER: $SERVER_USER"

JSON='
{
    "host": "'$host'",
    "Host": "'$host'",
    "HOST": "'$host'",

    "proj": "'$proj'",
    "Proj": "'$proj'",
    "PROJ": "'$proj'",

    "kind": "'$kind'",
    "Kind": "'$kind'",
    "KIND": "'$kind'",

    "kind01": '$kind_01',
    "Kind01": '$kind_01',
    "KIND01": '$kind_01',

    "app": "'$app'",
    "App": "'$app'",
    "APP": "'$app'",

    "user": "'$User'",
    "User": "'$User'",
    "USER": "'$User'",

    "host_name": "'$HostName'",
    "HostName": "'$HostName'",
    "HOST_NAME": "'$HostName'",

    "ROUTE": "'$ROUTE'",
    "FRONT_RE": "'$FRONT_RE'",
    "DOMAIN": "'$DOMAIN'",
    "DOMAIN_RE": "'$DOMAIN_RE'",
    "SERVER_USER": "'$SERVER_USER'",
    "BACK": "'$BACK'",
    "BACK_RE": "'$BACK_RE'",

    "PortOfKind": '${port_of_kind:=0}',
    "RoutePrefixOfKindRegex": "'$route_prefix_of_kind_regex'",

    "APP_GROUP": "'$APP_GROUP'",
    "PORT": "'$PORT'",
    "PORT_DEV": "'$PORT_DEV'"
}
' # last three is deprecated


handlebars-cli "$JSON" \
    "$di_dir/apps/$app/$include_conf" \
    > "$di_dir/$kind/$app/~$host/~$include_conf"

[[ $dry_run ]] || set -e
x $dry_run ssh $host "mkdir -p $proj/$kind/$app"
x $dry_run rsync -avz "$di_dir/$kind/$app/~$host/~$include_conf" "$host:$proj/$kind/$app/$include_conf"
x $dry_run ssh $host "grep -qF '$line' "$main_conf" || echo '$line' | sudo tee -a $main_conf"
x $dry_run ssh $host "sudo nginx -t && sudo nginx -s reload"
