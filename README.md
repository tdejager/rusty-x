[![Build Status](https://travis-ci.org/tdejager/rusty-x.svg?branch=master)](https://travis-ci.org/tdejager/rusty-x)

Rusty-x
=======

A simple snippet manager to find your snippets with colorized terminal output. 

Snippets can be setup as the following example:

```md
vim,
========    
# Splits                                                                                    
  * To balance splits, use Ctrl+w = 
```

Above we see that the first line is a number of keywords, which are delimited with an `,` character. The line of `========` is optional, it can be used to create a split. Then markdown is used to describe the content.

#### Caveats:
* *Warning currently only works on posix platforms. So now windows yet.*
* *Uses $EDITOR env var to select your editor.*

Install
=========

1. To install make sure that `cargo` and `rust` are installed
2. Install with `cargo install rusty-x --force`, `--force` is needed when the binary already exists

When installed from with cargo install, the commands start with `x`. So that means, e.g.:

```bash
x --edit python file
```

To edit a snippet with the keywords python and file.


Usage
=====

```
Usage: x
       x [--add=<filename>] <keywords>...
       x --new
       x [--edit] <keywords>...
       x --pull
       x --save

Options:
    -h, --help           Show this message
    --new                Add a new snippet without a given name and you need to fill in the keywords
    --add=<filename>     Add a new snippet with given filename and keywords
    -e, --edit           Edit a existing snippet
    --pull               Sync snippet repo (git pull)
    --save               Save snippet repo (git add, git commit, git push)
```

To list all snippets, when installed replace all `cargo run` with simply `x`:

```bash
cargo run
```

To find specific files according to tags:

```bash
cargo run <KEYWORDS>
```

To add a snippet with a keyword line, and a given filename:
```bash
cargo run --  --add=foo.md <KEYWORDS>
```

To add a snippet by just opening your `$EDITOR` at the given snippet location
```bash
cargo run -- --new
```

To edit a snippet with a given keywords:
```bash
cargo run -- --edit <KEYWORDS>
```

To sync and pull snippets from your snippet repo's:
```bash
cargo run -- --pull
```

To save snippets to your repositories:
```bash
cargo run -- --save
```
The command above aks for a commit message in case this is needed, and always tries to do a push for now.


Configuration
=============
The default snippet location is `~/.snippets/` 

A TOML is created as default configuration file, which can be found at: `~/.rusty-x.toml`
A default config is added to the config file automatically to change the location edit the toml.


Below a default toml can be found, multiple sources can added which `rusty-x` searches:

```toml
[[locations]]
local = "/home/tdejager/.snippets"
ext = "md"


[[locations]]
local = "/home/tdejager/.snippets-sjoerd"
ext = "md"
```



## Changelog

0.72: Added pprint library for printing

0.73: Sort snippets by number of tags that were matched

0.74: Removed rayon as dependency for now

0.75: Updated skim dependency
