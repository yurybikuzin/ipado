
source "$(dirname "${BASH_SOURCE[0]}")/check_utils.sh"

fill_target_files_with_styles() {
    [[ ${#styles[@]} -gt 0 ]] || echoerr "no styles"
    for style in ${styles[@]}; do
        target_files+=( "$style.css" )
    done
}

build_css() {
    [[ ${#styles[@]} -gt 0 ]] || echoerr "no styles"
    local dependencies_from_utils=()
    check_utils grass css-minifier
    for style in "${styles[@]}"; do
        local target_file="$target_dir/$style.css"
        local tmp_file="/tmp/$style.css"
        local src_file="$src_dir/$style.scss"
        cat << EOM
$target_file: ${dependencies_from_utils[@]} $src_file
	grass "$src_file" > "$tmp_file" 
	css-minifier -l 1 --input "$tmp_file" | tail -n +2 > "$target_file"

EOM
    done
}
