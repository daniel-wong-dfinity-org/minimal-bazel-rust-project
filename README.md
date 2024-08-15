This is what the name says: a minimal set of files that you need to write
something in Rust, and build it using Bazel.


# Usage

One way to use this is as follows:

1. Create a [new repository in GitHub][create].

[create]: https://github.com/new

2. Note the address of your repo. E.g. `git@github.com:hacker/new_repo.git`.
Optional: Save this to an environment variable:

```
REPO=git@github.com:hacker/new_repo.git
```

3. Run these commands:

```
git clone git@github.com:daniel-wong-dfinity-org/minimal-bazel-rust-project.git
git remote set-url origin $REPO
git push -u origin main
```

The second command in this step changes where you push and pull changes to and
from. Instead of pointing to the repo where you got this minimal set of files,
your local repo now points to the new GitHub repo you created in step 1.

4. Optional: You probably want to replace this README with information about your project.


# Versions In Use

We are using the latest stuff as of Aug, 2024. More precisely,

- rust : 1.79.0

- [rules_rust] : 0.49.3 (This is the Bazel library that adds Rust support to Bazel.)

[rules_rust]: https://github.com/bazelbuild/rules_rust


# Maintenance

To update this to use later versions, there are a few lines in WORKSPACE that
you'd have to modify.
