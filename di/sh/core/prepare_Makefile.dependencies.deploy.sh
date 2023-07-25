
get_HostName_User_of_Host_from "$host" "$ssh_config" \
    || echoerr "'Host $host' not found in '$ssh_config'"
dependencies+=( "$app_do" $("$app_do" get-dependencies-for-deploy "$proj" "$kind" "$app" "$host" "$User" "$HostName") )
