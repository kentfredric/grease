set -u

declare -a DO
DO=(
  html
  js
  xml
  png
  css
)

md5file() {
  md5sum "$1" | cut -f 1 -d " "
}

want_do() {
  local target="$1"
  local wanted=0
  for i in "${DO[@]}"; do
    if [ "${i}" == "${target}" ]; then
      wanted=1
      continue
    fi
    if [ "${i}" == "-${target}" ]; then
      wanted=0
      continue
    fi
  done
  if [ "${wanted}" == "1" ]; then
    return 0
  else
    return 1
  fi
}

restore_cache() {
  local file="$1"
  local orig_md5="$(md5file "${file}")"
  local orig_link="./.md5_cache/${file}/${orig_md5}"

  if [ -h "${orig_link}" ]; then
#    printf "Cache found: %s\n" "${orig_link}"
    link_dest="$(readlink "${orig_link}")";
    link_dest_md5="$(basename "${link_dest}")"
#    printf "Cache dest: %s\n" "${link_dest}"
    link_dest_actual_md5="$(md5file "${link_dest}")"
    if [ "${link_dest_actual_md5}" != "${link_dest_md5}" ]; then
      printf "\e[31m Error resurrecting $file, corrupt cache: cache-entry: ${link_dest_md5} vs actual: ${link_dest_actual_md5}\e[0m\n";
      exit 1
    fi
    cp --reflink=auto -L "${orig_link}" "${file}"
    return 0
  fi
  return 1
}

header() {
  local color="$1"
  local label="$2"
  local wanting="$3"
  local file="$4"
  if want_do "${wanting}"; then
    if restore_cache "${file}"; then
      printf "\e[%sm--[ \e[33m %15s:\e[0m %s \e[34;1m(restored)\e[0m\e[%sm ]--\e[0m\r" "${color}" "${label}" "${file}" "${color}"
    elif is_cached "${file}"; then
      printf "\e[%sm--[ \e[33m %15s:\e[0m %s \e[32;1m(unchanged)\e[0m\e[%sm ]--\e[0m\r" "${color}" "${label}" "${file}" "${color}"
    else
      printf "\e[%sm--[ \e[33;1m %15s:\e[0m %s \e[%sm ]--\e[0m\n" "${color}" "${label}" "${file}" "${color}"

    fi
  else
    printf "\e[%sm--[ \e[33m %15s:\e[0m %s \e[31;1m(skipped)\e[0m\e[%sm ]--\e[0m\n" "${color}" "${label}" "${file}" "${color}"
  fi
}

set_cached() {
  local file="$1"
  local orig_md5="$2"
  mkdir -p "./.md5_cache/${file}"
  local md5="$(md5file "${file}")"
  # printf "Caching $orig_md5 -> $md5 for ${file}\n"
  cp --reflink=auto "${file}" "./.md5_cache/${file}/${md5}"
  if [ "${orig_md5}" != "${md5}" ]; then
    ln -s "$PWD/.md5_cache/${file}/${md5}" "./.md5_cache/${file}/${orig_md5}"
  fi
}



is_cached() {
  local file="$1"
  if [ ! -d "./.md5_cache/${file}" ]; then
    return 1
  fi
  local md5="$(md5file "${file}")"
  if [ -f "./.md5_cache/${file}/${md5}" ]; then
    return 0
  fi
  return 1
}


tidy_html() {
  local file="$1"
  header 35 "tidying html" html "${file}"
  want_do html || return
  is_cached "${file}" && return
  local orig_md5="$(md5file "${file}")"
  tidy -m -q \
    --add-meta-charset yes \
    --doctype html5 \
    --newline LF \
    --merge-divs no \
    --merge-spans no \
    --fix-style-tags no \
    --literal-attributes yes \
    --escape-scripts yes \
    --indent auto \
    --indent-spaces 1 \
    --indent-with-tabs no \
    --indent-attributes yes \
    --preserve-entities yes \
    --omit-optional-tags no \
    --tidy-mark no \
    --wrap-attributes yes \
    --wrap-sections no \
    --drop-empty-elements no \
    --drop-empty-paras no \
    --drop-proprietary-attributes no \
    --wrap 240 \
    "${file}";
  if [ $? -lt 2 ]; then
    set_cached "${file}" "${orig_md5}"
  fi
}

tidy_html_crude() {
  local file="$1"
  header 35 "(crudely) tidying html" html "${file}"
  want_do html || return
  is_cached "${file}" && return
  local orig_md5="$(md5file "${file}")"
  perl util/html_tidy.pl "${file}" && set_cached "${file}" "${orig_md5}"
}

tidy_js() {
  local file="$1"
  header 34 "tidying js" js "${file}"
  want_do js || return
  is_cached "${file}" && return
  local orig_md5="$(md5file "${file}")"
  js-beautify \
    --indent-size=1 \
    --eol="\n" \
    --end-with-newline \
    --replace \
    --comma-first \
    --operator-position=after-newline \
    "${file}" && set_cached "${file}" "${orig_md5}"

}

tidy_xml() {
  local file="$1"
  header 32 "tidying xml" xml "${file}"
  want_do xml || return
  is_cached "${file}" && return
  local orig_md5="$(md5file "${file}")"
  xmltidy "${file}" " " && \
   perl -i -pl -0777 -e "s/<!--.*?-->\n*?//smg; s/\n\n+/\n/smg" "${file}" && \
   set_cached "${file}" "${orig_md5}"
}

compress_png() {
  local file="$1"
  header 33 "compressing png" png "${file}"
  want_do png || return
  is_cached "${file}" && return
  local orig_md5="$(md5file "${file}")"

  cp -vf "${file}" "${file}.opt"
  advpng -z -0 -f "${file}.opt"
  optipng -f0-5 -strip all -zc1-9 -zm1-9 -zs0-3 -zw32k "${file}.opt"
  advpng -z -4 -i 100 "${file}.opt"
  zopflipng -y -m --filters=01234meb --iterations=500 "${file}.opt" "${file}.opt"
  new="$(wc -c <"${file}.opt")"
  orig="$(wc -c <"${file}")"

  printf " : original : \e[33m%s\e[0m\n" "${orig}"
  printf " : new      : \e[33m%s\e[0m\n" "${new}"

  if [ "${new}" -lt "${orig}" ]; then
    mv -vf "${file}.opt" "${file}"
  else
    rm -v "${file}.opt"
  fi
  set_cached "${file}" "${orig_md5}"
}

tidy_css() {
  local file="$1"
  header 36 "tidying css" css "${file}"
  want_do css || return
  is_cached "${file}" && return
  local orig_md5="$(md5file "${file}")"
  perl ./util/css_tidy.pl "${FILE}" && \
   set_cached "${file}" "${orig_md5}"
}

find_apply() {
  local callback=$1
  local FILE findargs
  shift
  declare -a findargs
  findargs=( "-name" "$1" )
  shift
  for argv; do
    findargs+=( "-o" "-name" "${argv}" )
  done
  while read -d "" -r FILE; do
    "${callback}" "${FILE}"
  done < <(find target/doc -type f "-(" "${findargs[@]}" "-)" -print0)
  printf "\n";
}

# Non-crude implementation fucks <h4><div> combos, thanks rust.
find_apply tidy_html "*.htm" "*.html"
find_apply tidy_js "*.js" "*.json"
find_apply tidy_xml "*.xml" "*.svg"
find_apply tidy_css "*.css"
find_apply compress_png "*.png"
