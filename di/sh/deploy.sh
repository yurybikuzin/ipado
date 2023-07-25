
[[ $dry_run ]] || set -e
x $dry_run ssh $host "mkdir -p $proj/$kind/$app"
x $dry_run rsync -avz --exclude '~*' "$target_dir/"* "$host:$proj/$kind/$app/"
