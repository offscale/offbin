offbin
======
Single command to run any of the commands that were previously in the directory.



## RFC

https://github.com/offscale/offscale-rfcs/blob/master/%5Boffbin%5D%20Binarise%20tool.md


## User guide

Note: Offbin in currently in development and not recommended for production use.

Running offbin will generate a new rust project and build a self-extracting binary located
in s  
```bash
$ offbin -i input_folder -o output_folder
```


## Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). This works on nightly and stable. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).
