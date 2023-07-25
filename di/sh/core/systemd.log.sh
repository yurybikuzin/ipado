
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

handlebars-cli "$JSON" "$di_dir/apps/$app/$systemd_log_sh" > "$di_dir/$kind/$app/~$host/~$systemd_log_sh"

[[ $dry_run ]] || set -e
# x $dry_run ssh $host "mkdir -p $proj/$kind/$app"
x $dry_run rsync -avz "$di_dir/$kind/$app/~$host/~$systemd_log_sh" "$host:$proj/$kind/$app/$systemd_log_sh"
x $dry_run ssh $host "chmod a+x $proj/$kind/$app/$systemd_log_sh"
x $dry_run echo "$di_dir/$kind/$app/~$host/~$systemd_log_sh"

