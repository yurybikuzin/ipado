
find_in() {
    target=$1; shift
    for i in "$@"; do 
        [[ $i == $target ]] && return 0 
    done
    return 1
}
