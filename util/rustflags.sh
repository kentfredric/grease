RFLAGS=(
  "-W" "absolute-paths-not-starting-with-crate"
  "-W" "anonymous-parameters"
  "-W" "bare-trait-objects"
  "-W" "box-pointers"
# Not in 1.31
  "-W" "deprecated-in-future"
  "-W" "elided-lifetimes-in-paths"
  "-W" "ellipsis-inclusive-range-patterns"
  "-W" "explicit-outlives-requirements"
  "-W" "future-incompatible"
  "-W" "keyword-idents"
  "-W" "macro-use-extern-crate"
  "-W" "missing-copy-implementations"
  "-W" "missing-debug-implementations"
  "-W" "missing-doc-code-examples"
  "-W" "missing-docs"
  "-W" "nonstandard-style"
# Not in 1.31
  "-W" "private-doc-tests"
  "-W" "question-mark-macro-sep"
  "-W" "rust-2018-compatibility"
  "-W" "rust-2018-idioms"
# Not in 1.31
  "-W" "rustdoc"
# Currently false-positives
# https://github.com/rust-lang/rust/issues/60554
# "-W" "single-use-lifetimes"
  "-W" "trivial-casts"
  "-W" "trivial-numeric-casts"
  "-W" "unreachable-pub"
  "-W" "unsafe-code"
# external doc is unstable :(
# "-W" "unstable-features"
  "-W" "unused"
  "-W" "unused-extern-crates"
  "-W" "unused-import-braces"
  "-W" "unused-labels"
  "-W" "unused-lifetimes"
  "-W" "unused-qualifications"
  "-W" "unused-results"
  "-W" "variant-size-differences"
  "-W" "warnings"
)
RDOCFLAGS=(
  "-Z" "unstable-options"
  "--disable-minification"
  "--enable-index-page"
  "--extern-html-root-url" "failure=https://docs.rs/failure/0.1.5"
)

CLIPPY_FLAGS_MASTER=(
  "-W" "clippy::checked_conversions"
)

CLIPPY_FLAGS_NIGHTLY=(
  "-W" "clippy::copy_iterator"
  "-W" "clippy::explicit_into_iter_loop"
  "-W" "clippy::explicit_iter_loop"
  "-W" "clippy::filter_map_next"
  "-W" "clippy::find_map"
  "-W" "clippy::large_digit_groups"
  "-W" "clippy::map_flatten"
  # Silly
  #  "-W" "clippy::module_name_repetitions"
  "-W" "clippy::needless_pass_by_value"
  "-W" "clippy::redundant_closure_for_method_calls"
  "-W" "clippy::shadow_unrelated"
  "-W" "clippy::too_many_lines"
)

CLIPPY_FLAGS_0_0_212=(
#  "-W" "clippy::stutter" # becomes module_name_repetitions in future
)
CLIPPYFLAGS=(
  "-W" "clippy::all"
#  "-W" "clippy::pedantic"
  "-W" "clippy::cast_possible_truncation"
  "-W" "clippy::cast_possible_wrap"
  "-W" "clippy::cast_precision_loss"
  "-W" "clippy::cast_sign_loss"
  "-W" "clippy::default_trait_access"
  "-W" "clippy::doc_markdown"
  "-W" "clippy::empty_enum"
  "-W" "clippy::enum_glob_use"
  "-W" "clippy::expl_impl_clone_on_copy"
  "-W" "clippy::filter_map"
  "-W" "clippy::if_not_else"
  "-W" "clippy::indexing_slicing"
  "-W" "clippy::inline_always"
  "-W" "clippy::invalid_upcast_comparisons"
  "-W" "clippy::items_after_statements"
  "-W" "clippy::linkedlist"
  "-W" "clippy::match_same_arms"
  "-W" "clippy::maybe_infinite_iter"
  "-W" "clippy::mut_mut"
  "-W" "clippy::needless_continue"
  "-W" "clippy::non_ascii_literal"
  "-W" "clippy::option_map_unwrap_or"
  "-W" "clippy::option_map_unwrap_or_else"
  "-W" "clippy::pub_enum_variant_names"
  "-W" "clippy::replace_consts"
  "-W" "clippy::result_map_unwrap_or_else"
  "-W" "clippy::similar_names"
  "-W" "clippy::single_match_else"
  "-W" "clippy::string_add_assign"
  "-W" "clippy::unicode_not_nfc"
  "-W" "clippy::unseparated_literal_suffix"
  "-W" "clippy::use_self"
  "-W" "clippy::used_underscore_binding"
  "-W" "clippy::nursery"
  "-W" "clippy::cargo"
  "${CLIPPY_FLAGS_0_0_212[@]}"
  "${CLIPPY_FLAGS_NIGHTLY[@]}"
)
