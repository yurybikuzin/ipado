
# line="include /home/$User/$proj/$kind/$app/$include_conf;"

if [[ $kind == "dev" ]]; then
    kind_01=0
else 
    kind_01=1
fi

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
    "HOST_NAME": "'$HostName'"
}
'

cargo install handlebars-cli
handlebars-cli "$JSON" "$di_dir/apps/$app/$systemd_service" > "$di_dir/$kind/$app/~$host/~$systemd_service"

[[ $dry_run ]] || set -e
# x $dry_run ssh $host "mkdir -p $proj/$kind/$app"
x $dry_run rsync -avz "$di_dir/$kind/$app/~$host/~$systemd_service" "$host:$proj/$kind/$app/$systemd_service"
x $dry_run ssh $host "sudo mv $proj/$kind/$app/$systemd_service /etc/systemd/system/${app}_$kind.service && sudo systemctl daemon-reload && sudo systemctl restart ${app}_$kind"
x $dry_run echo "$di_dir/$kind/$app/~$host/~$systemd_service"

