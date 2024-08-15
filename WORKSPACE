workspace(name = "nightly")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    integrity = "sha256-3QBrdyIdWeTRQSB8DnrfEbH7YNFEC4/KA7+SVheTKmA=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.49.3/rules_rust-v0.49.3.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.79.0",
    ],
)
