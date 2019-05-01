source ./util/rustflags.sh

cargo fmt &&
./util/quickclean.sh &&
  RUSTFLAGS="${RFLAGS[*]}" cargo clippy --all-targets -- "${CLIPPYFLAGS[@]}" &&
  RUSTFLAGS="${RFLAGS[*]}" cargo clippy --profile test --all-targets -- "${CLIPPYFLAGS[@]}"
