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

cargo update -Z no-index-update
cargo update -Z minimal-versions -Z no-index-update

# This file contains hacks required to get various cargo build targets working
# on modern rust.

advanced() {
  switchlist libc  0.1.{0..5} 0.1.10 0.1.12 \
                   0.2.{0..7}
}

simple() {
  # backtrace 0.3.3 uses RTLD_LAZY first seen in libc 0.2.7
  switchup libc:0.2.0 0.2.7
}

#advanced
simple

cargo build --all-targets &&
  cargo test
