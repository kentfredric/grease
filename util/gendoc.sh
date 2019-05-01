source ./util/rustflags.sh
[ -e target/doc ] && rm -r target/doc
  RUSTFLAGS="${RFLAGS[*]}" RUSTDOCFLAGS="${RDOCFLAGS[*]}" cargo doc -q --no-deps &&
  util/tidy.sh
