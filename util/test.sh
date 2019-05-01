source ./util/rustflags.sh

if [ "$1"  == "--verbose" ]; then
  do_test() {
    RUSTFLAGS="${RFLAGS[*]}" cargo test "$@"
  }
else
do_test() {
  local out_file;
  local err_file;
  local e_code;
  out_file="$(tempfile cargo_test_out_XXXXXXXXXXXXX)" || exit 1;
  err_file="$(tempfile cargo_test_err_XXXXXXXXXXXXX)" || exit 1;
  RUSTFLAGS="${RFLAGS[*]}" cargo test "$@" >"$out_file" 2>"$err_file"
  e_code=$?
  if [ $e_code != 0 ]; then
    printf "\e[31;1mcargo test failed\e[0m"
    echo "$@"
    printf "\e[31;1m-=-[ stdout ]-=-\e[0m\n"
    cat "${out_file}";
    printf "\e[31;1m-=-[ stderr ]-=-\e[0m\n"
    cat "${out_file}";
    return $e_code;
  fi
  unlink "${out_file}";
  unlink "${err_file}";
  return 0;
}
fi

cargo fmt &&
  do_test --all-targets &&
  do_test --doc
