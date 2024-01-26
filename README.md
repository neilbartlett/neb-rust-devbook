# Rust Book
This repo captures code from working through the [Rust Book](https://doc.rust-lang.org/book/)

It contains actual examples from the book as well as code examples to try out ideas the book covers.

The code is set up to be directly usable from Visual Studio Code.

## Structure
This is structured as a single workspace with sub-projects corresponding to different code examples. If the code is from a specific chapter I generally prefix the subproject with the chapter. Workspace technique is outlined in the rust book in chapter 14:
https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

## Running code
The simplest way to run the code is to clone the repo and use Visual Code Studio Dev Containers. The code already contains a devcontain.json so it should be as simple as a *Reopen as Dev Container* in the top level directory once docker and the Visual Cosde Dev Container extension is installed. Potentially this is also runnable from a Github Cotnainer. This will build a container and set up rust.

To run any sample 
```
cargo run --bin <name>
```
where nae is any of the names of the project directories. These are listed under the workspace members in the toplevel `Cargo.toml` file.

## Adding more examples
To add further examples, under the main directory from the command line use

```
cargo new <projectname>
```

This will automatically add the project *projectanem* to the workspace, set up the project directory structure under the main directory and add a Cargo.toml to that structure. Dependencies for the project should be added to the projects `Cargo.toml` file, not the top level.