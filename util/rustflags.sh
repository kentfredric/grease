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
  "-W" "single-use-lifetimes"
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
CLIPPYFLAGS=(
  "-W" "clippy::all"
  "-W" "clippy::pedantic"
  "-W" "clippy::nursery"
  "-W" "clippy::cargo"
)
