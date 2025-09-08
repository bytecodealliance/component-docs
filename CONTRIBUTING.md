# Contributing

The Component Model documentation is a [Bytecode Alliance](https://bytecodealliance.org/) project, and follows the Bytecode Alliance's [Code of Conduct](CODE_OF_CONDUCT.md) and [Organizational Code of Conduct](ORG_CODE_OF_CONDUCT.md).

## Using this repository

You can run the website locally using the [mdBook](https://rust-lang.github.io/mdBook/index.html) command line tool.

### Prerequisites

To use this repository, you need [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html) installed on your workstation.

This repository also makes use of mdBook plugins. To install mdBook and the plugins for this project, you can use [`cargo`][cargo]:

```console
cargo install --version 0.4.21 mdbook
cargo install --version 0.6.7 mdbook-alerts
cargo install --version 0.7.7 mdbook-linkcheck
```

[cargo]: https://doc.rust-lang.org/cargo

## Running the website locally

After installing mdBook, you'll need to clone the code via git and navigate to the directory.

```bash
git clone https://github.com/bytecodealliance/component-docs
cd component-docs
```

To build and test the site locally, run:

```bash
cd component-model
mdbook serve --open
```

You can use mdbook-linkcheck to check the links in the docs automatically. First, add the lines following lines in `book.toml`.

```toml
[output.linkcheck]
follow-web-links = true
```

After this, install the extension and build the project again. You should see the link checker do its work in the console output.

```bash
cargo install mdbook-linkcheck
mdbook build
```

Don't forget to remove the changes in `book.toml` before you commit!

## Writing style guide

This section contains what is a *somewhat loosely* applied style guide for writing that is contributed to `component-docs`.

### Executable code fragments

For code that should be executed by the user in their console of choice, prefer using `sh`/`bash`. While not every user's
shell is `sh` or `bash` (with `zsh` being incredilby common), `sh`/`bash` provide a decent approximation.

Importantly, code that is meant to be executed should be copy-pastable -- and should not contain `$` as a prefix. For example:

```sh
echo 'this is an example';
```

On the other hand, when commands *and* output are shown, use `console` and *do* prefix the command with `$` to differentiate it (or `#` in a sudo context):

```console
$ echo 'this is an example';
this is an example
```

## Submitting Changes

You can click the Fork button in the upper-right area of the screen to create a copy of this repository in your GitHub account. This copy is called a fork. Make any changes you want in your fork, and when you are ready to submit those changes, go to your fork and create a new pull request to let us know about it.

Everyone is welcome to submit a pull request! Once your pull request is created, we'll try to get to reviewing it or responding to it in at most a few days. As the owner of the pull request, it is your responsibility to modify your pull request to address the feedback that has been provided to you by the reviewer.
