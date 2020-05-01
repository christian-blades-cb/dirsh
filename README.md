# dirsh

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

Dirsh hashes the contents of a directory, but respects your `.gitignore` file and its own `.hashignore`.

## Motivation

Hashing the inputs of a build is a convenient way to avoid rebuilding. Current solutions to this problem are either domain-specific, or lack a convenient way to configure which files should be included in the hash.

## Usage

By default, dirsh hashes the current directory. `.gitignore` and `.hashignore` are parsed in the [gitignore spec](https://git-scm.com/docs/gitignore) and used for filtering.

Dirsh calculates the md5 for the directory contents and returns the hash in base32 without padding.

```
$ dirsh -h
dirsh 0.1.0
Ignore-file-respecting consistent hasher of directory contents

USAGE:
    dirsh [FLAGS] [path]

FLAGS:
    -h, --help            Prints help information
        --no-gitignore    don't parse gitignore (including global gitignore and local git excludes)
    -V, --version         Prints version information

ARGS:
    <path>     [default: ./]
```


## The algorithm

Paths are recursively walked in alphabetical order.

File contents are fed to the digest, followed by 64 bits of modification time, and 32 bits of mode. 

The md5 hash is then computed and base32 encoded without padding (for ease of url encoding, and compatibility with filesystems which ignore capitalization).

## Development

This project is built with nix, and the development environment is available with `nix-shell`, or even better, [lorri](https://github.com/target/lorri).

You can build the release binary with `nix-build`.

### Without nix

This is a standard rust project. You'll need the [rust toolchain](https://www.rust-lang.org/tools/install) to build this project. 

`cargo build`
