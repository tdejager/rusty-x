[![Build Status](https://travis-ci.org/tdejager/rusty-x.svg?branch=master)](https://travis-ci.org/tdejager/rusty-x)

Rusty-x
=======

A simple snippet manager to find your snippets with colorized terminal output. 

*Warning currently only works on posix platforms.*
*Uses $EDITOR env var to select your editor*

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

To find files:

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

To edit a snippt with a given keywords:
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

Installed
=========

When installed from with cargo install, the commands start with `x`. So that means, e.g.:

```bash
x --edit python file
```

To edit a snippet with the keywords python and file.
