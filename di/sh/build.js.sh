
source "$(dirname "${BASH_SOURCE[0]}")/check_utils.sh"

fill_target_files_with_scripts() {
    [[ ${#scripts[@]} -gt 0 ]] || echoerr "no scripts"
    for script in ${scripts[@]}; do
        target_files+=( "$script.js" "$script.js.map" )
    done
}

build_js() {
    [[ ${#scripts[@]} -gt 0 ]] || echoerr "no scripts"
    local dependencies_from_utils=()
    check_utils terser
    for script in "${scripts[@]}"; do
        src_file="$src_dir/$script.js"
        target_file="$target_dir/$script.js"
        script_js_map="$script.js.map"
        cat << EOM
$target_file $target_dir/$script_js_map: ${dependencies_from_utils[@]} $src_file
	cd $src_dir/..
	terser "$src_file" -c -m --ecma 5 --source-map "url=$script.js.map,includeSources" --output "$target_file"

EOM
	# pushd $src_dir/..
	# popd
	# terser "$src_file" -c -m --ecma 5 --source-map "url=$script.js.map,includeSources" --output "$target_file"
done
}
