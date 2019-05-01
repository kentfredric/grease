shopt -s extglob
declare -a ASSETS
declare -a ARTIFACTS

for target in debug release; do
  ASSETS=()
  ARTIFACTS=( 'libgrease' 'grease' )
  for dest in "target/${target}/incremental"; do
        [ -e "${dest}" ] && ASSETS+=( "${dest}" )
  done
  for artifact in "target/${target}/"*; do
    my_artifact=$(basename "${artifact}" | sed 's/\.[^.]*//' );
    [ -f "${artifact}" ] || continue;
    ARTIFACTS+=( "${my_artifact}" )
  done
  for artifact in "${ARTIFACTS[@]}"; do
    for result in "target/${target}/deps/${artifact}"*    \
                  "target/${target}/${artifact}"*         \
                  "target/${target}/deps/lib${artifact}"* \
                  "target/${target}/lib${artifact}"*      \
    ; do
      [ -e "${result}" ] && ASSETS+=( "${result}" )
    done
  done

  if [ ${#ASSETS[@]} != 0 ]; then
    printf "\e[33;1m---[\e[0m ${target} \e[33;1m]---\e[0m\n"
    for asset in "${ASSETS[@]}"; do
      [ -e "${asset}" ] || continue;
      size="$(du -sh "${asset}" | cut -f 1)"
      printf "\e[31;1m(-) \e[36;1m${asset}\e[0m (%s): " ${size}
      rm -vr "${asset}" | wc -l
    done
  fi
done
