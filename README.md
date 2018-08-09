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

To find files:

```bash
cargo run <KEYWORDS>
```

To add a snippet with a keyword line:
```bash
cargo run <KEYWORDS> add
```

To edit a snippt with a given keywords:
```bash
cargo run <KEYWORDS> edit
```

Installed
=========

When installed from with cargo install, the commands start with `x`. So that means, e.g.:

```bash
x python file
```

To find a snippet with the keywords python and file
