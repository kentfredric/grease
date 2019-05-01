switchup() {
  cargo update -p $1 -Z minimal-versions -Z no-index-update --precise $2
}

switchlist() {
  local name=$1
  shift
  while true; do
    local this=$1
    local next=$2
    if [ "${this}" == "" ]; then
      break
    fi
    if [ "${next}" == "" ]; then
      break
    fi
    switchup "${name}:${this}" "${next}"
    shift
  done
}
cargo update -Z minimal-versions -Z no-index-update

# This file contains hacks required to get various cargo build targets working
# on modern rust.

advanced() {
  true
}

simple() {
  true
}

#advanced
simple

cargo build --all-targets &&
  cargo test
