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
#switchup criterion 0.2.10

switchlist time 0.0.{1..3} 0.1.{0..9}  0.1.1{0..9} 0.1.2{0..5} # 0.1.2{0..7}
switchlist rustc-serialize 0.1.{0..5} 0.2.{0..15} 0.3.{0..7} 0.3.{9..20} # 0.3.{9..20}
switchlist libc 0.1.{0..5} #0.1.10 #0.1.12
switchlist rand 0.1.{1..4} 0.2.{0..1} 0.3.{0..10}
switchlist gcc 0.0.{1..2} 0.1.{0..7} 0.2.{0..1} 0.3.{0..4} # 0.3.{0..6} 0.3.{8..14}
#switchup num 0.1.27
#switchup byteorder 1.2.1
#switchup simplelog 0.5.1
#switchup winapi:0.0.1 0.2.4
switchlist backtrace 0.3.{3..9} 0.3.{11..13}
}

simple() {
  switchup time 0.1.25
  switchup rustc-serialize 0.3.20
  switchup libc:0.1.1 0.1.5
  switchup rand:0.1.1 0.3.10
  switchup rand:0.3.0 0.3.10
  switchup gcc:0.0.1 0.3.4
  switchup gcc:0.3.0 0.3.4
  switchup backtrace:0.3.3 0.3.13
}

#advanced
simple

cargo build --all-targets
