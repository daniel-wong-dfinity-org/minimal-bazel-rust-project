This is what the name says: a minimal (yet practical) set of files that you need
to write something in Rust, and build it using Bazel.

A practical solution needs a way to get libraries from crates.io. This supports
that.

We might be implicitly assuming that you know cargo and bazel separately, but
you maybe do not know how to make them coexist. (Truth be told, I am only dimly
aware of how to make them play nice together.)


# Prerequisites

Unfortunately, I can only give instructions for how to install prereqs on Mac,
because that's what I have. But it should be fairly easy to do the same on other
platforms (hopefully??).

## Bazel

On Mac, I recommend that you do

```
brew install bazelisk
```

It might also be possible to do

```
brew install bazel
```

but I cannot vouch for this method, even though this is what the official bazel
documentation recommends.

Of course, this assumes that you have already [brew installed]. If you don't
already have that, I feel that you are very much missing out. In any case, I
have no other recommendations for how you should obtain bazel on Mac.

[brew installed]: https://docs.brew.sh/Installation

## Cargo

```
curl https://sh.rustup.rs -sSf | sh
```

[Deets].

[Deets]: https://doc.rust-lang.org/cargo/getting-started/installation.html


# Demo

Once you clone this repo, and `cd` into it,

```
CARGO_BAZEL_REPIN=true bazel run //foo:hello_bazel
```

## What Is This `CARGO_BAZEL_REPIN`??

To be perfectly honest, I'm not super clear on what `CARGO_BAZEL_REPIN` does,
but it has something to do with using third-party libraries (from crates.io).

I think you maybeprobably want `bazel` to always see `CARGO_BAZEL_REPIN=true`.
If you agree, you should probably put something like the following in your
.bashrc (or equivalent):

```
export CARGO_BAZEL_REPIN=true
```

(Don't omit the `export` in front!)

If you do this, then you will not need to every time prefix your `bazel`
commands with `CARGO_BAZEL_REPIN=true` (like we did in the demo). Instead, your
`bazel` commands will look like normal `bazel` commands and have less spam.

Not sure what bad things might happen if `bazel` always sees
`CARGO_BAZEL_REPIN=true`. I mean, presumably, there is _some_ reason it doesn't
behave that way by default. The reason might be just that it runs slower. Other
than that, it seems to do no harm.

What I do know is that if bazel does not see `CARGO_BAZEL_REPIN=true`, then, in
practice, it is inevitable that you will run into problems whenever you start
(newly) depending on a(nother) library from crates.io. If you never do that
(seems unrealistic), then maybe you can forget about `CARGO_BAZEL_REPIN`.

The good news is that if you forget, the output will remind you about this (but
bazel will "kindly" bury this useful information in a hay stack for you, the
native habitat of useful error output.)


# Starting Your Own Project

Here is one way to do that:

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

5. Optional: If you do not want to inherit the history of this repo, you can
probably use some incantation based on `git rebase -i` to squash all commits,
and give the one commit a new commit message, like "Starting project $X. The
plan is to write it in Rust, and build it using Bazel.". If you are feeling
generous, you can link back to this repo.


# Versions in Use

We are using the latest stuff as of Aug, 2024. More precisely,

- rust : 1.79.0

- [rules_rust] : 0.49.3 (This is the Bazel library that adds Rust support to Bazel.)

[rules_rust]: https://github.com/bazelbuild/rules_rust


# Maintenance

To update this to use later versions, there are a few lines in WORKSPACE that
you'd have to modify.
