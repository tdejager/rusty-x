[![Build Status](https://travis-ci.org/tdejager/rusty-x.svg?branch=master)](https://travis-ci.org/tdejager/rusty-x)

Rusty-x
=======

A simple snippet manager to find your snippets with colorized terminal output. 

*Warning currently only works on posix platforms.*
*Warning only supports vim as current editor*

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



Usage
=====

```
Usage: x [--add=<filename>] <keywords>...
       x [--edit] <keywords>...

Options:
    -h, --help           Show this message
    --add=<filename>     Add a new snippet with given filename and keywords
    -e, --edit           Edit a existing snippet

```

To find files:

```bash
cargo run <KEYWORDS>
```

To add a snippet with a keyword line, and a given filename:
```bash
cargo run --add=foo.md <KEYWORDS>
```

To edit a snippt with a given keywords:
```bash
cargo run --edit <KEYWORDS>
```

Installed
=========

When installed from with cargo install, the commands start with `x`. So that means, e.g.:

```bash
x --edit python file
```

To edit a snippet with the keywords python and file.
