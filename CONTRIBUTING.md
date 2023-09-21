# Contributing

The Component Model documentation is a [Bytecode Alliance](https://bytecodealliance.org/) project, and follows the Bytecode Alliance's [Code of Conduct](CODE_OF_CONDUCT.md) and [Organizational Code of Conduct](ORG_CODE_OF_CONDUCT.md).

## Using this repository

You can run the website locally using the [mdBook](https://rust-lang.github.io/mdBook/index.html) command line tool.

### Prerequisites

To use this repository, you need [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html) installed on your workstation.

### Running the website locally

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

## Submitting Changes

You can click the Fork button in the upper-right area of the screen to create a copy of this repository in your GitHub account. This copy is called a fork. Make any changes you want in your fork, and when you are ready to submit those changes, go to your fork and create a new pull request to let us know about it.

Everyone is welcome to submit a pull request! Once your pull request is created, we'll try to get to reviewing it or responding to it in at most a few days. As the owner of the pull request, it is your responsibility to modify your pull request to address the feedback that has been provided to you by the reviewer.

### Adding New Pages

If you add a new page to the site, please note that the `sitemap.xml` will need to be updated to include that new page. The sitemap can be updated using the following proceedure:

```bash
cargo install mdbook-sitemap-generator
cd component-docs
mdbook-sitemap-generator -d component-model.bytecodealliance.org -o sitemap.xml
```
