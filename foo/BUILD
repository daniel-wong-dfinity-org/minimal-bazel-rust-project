load("@crate_index//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

rust_library(
    name = "foo",
    srcs = glob(
        ["src/**"],
        exclude = [
            "**/tests/**",
            "**/*tests.rs",
            "src/main.rs",
        ]
    ),
    deps = all_crate_deps(
        normal = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)

rust_binary(
    name = "hello_bazel",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(
        normal = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)
