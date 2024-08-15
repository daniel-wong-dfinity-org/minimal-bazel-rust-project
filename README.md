This is what the name says: a minimal set of files that you need to write
something in Rust, and build it using Bazel.


# Versions In Use

We are using the latest stuff as of Aug, 2024. More precisely,

- rust: 1.79.0

- [rust_rules]: 0.49.3 (This is the Bazel library that adds Rust support to Bazel.)

[rust_rules]: https://github.com/bazelbuild/rules_rust


# Maintenance

To update this to use later versions, there are a few lines in WORKSPACE that
you'd have to modify.
