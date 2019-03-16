switchup() {
  cargo update -p $1 -Z minimal-versions -Z no-index-update --precise $2
}
cargo update -Z minimal-versions -Z no-index-update

# This file contains hacks required to get various cargo build targets working
# on modern rust.

#switchup criterion 0.2.10
switchup libc:0.1.1 0.1.12
switchup rand 0.3.10
#switchup num 0.1.27
#switchup byteorder 1.2.1
#switchup simplelog 0.5.1
#switchup winapi:0.0.1 0.2.4
