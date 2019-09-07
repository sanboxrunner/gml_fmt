# gml_fmt: GML Code Formatter

[![Build Status](https://travis-ci.org/sanboxrunner/gml_fmt.svg?branch=master)](https://travis-ci.org/sanboxrunner/gml_fmt)

`gml_fmt` is an autoformatter written in Rust for GML. It is fast and works on code which will not compile. It has succesfully parsed and formatted huge GML projects, such as GMLive's autogenerated GML, ImGUIGML, and has been tested by the developers of Forager.

# How To Install and Run

Simply go to [releases](https://github.com/sanboxrunner/gml_fmt/releases/tag/v1.0.1) and download the most recent binary in whatever OS you're on. Or, click here:

- [Windows 10](https://github.com/sanboxrunner/gml_fmt/releases/download/v1.0.1/gml_fmt-v1.0.1-x86_64-windows-msvc.tar.gz)
- [macOS](https://github.com/sanboxrunner/gml_fmt/releases/download/v1.0.1/gml_fmt-v1.0.1-x86_64-apple-darwin.tar.gz)
- [Linux](https://github.com/sanboxrunner/gml_fmt/releases/download/v1.0.1/gml_fmt-v1.0.1-x86_64-unknown-linux-gnu.tar.gz)

Once you have downloaded and extracted the program (perhaps using 7zip) from the .gzip and the .tar, simply place it next to your `.yyp` file in your project's root directory. Navigate in your native terminal to that folder (Command Prompt on Windows, Bash on other platforms) and run:

```bash
gml_fmt --version
```

(You may need to do `./gml_fmt --version` depending on your platform.)

If you get back a version number, the tool is working.

To format your code, first **make sure that you are using source control or have another backup. This is an autoformatter, and though it is battle tested, it could be your project that shows the bug. Have a backup ready for that case.**

Run:

```
gml_fmt
```

Run `gml_fmt -f path/to/file` to format only a single file. Otherwise, gml_fmt will format everything in the directory its in that is a `.gml` file.

Run `gml_fmt --help` to get a full listing of commands available.

Currently, watch mode is not enabled, but future updates will bring it, if the tool sees adoption.

If you would like to use the tool without moving it between projects, add it to your PATH and then invoke like so:
```
gml_fmt path/to/directory/of/project
```
Creating a bash script to automate this task might be a good idea.

# Wait, wait..what are you doing to my code?
`gml_fmt` is opinionated, primarily because making configuration options makes it slower to make and slower to run. 

`gml_fmt` outputs K&R Java compliant code. In normal human terms, that means:
```js
if (formatted) {
    show_debug_message("We're K&R all day.");
}
```
Additionally, `gml_fmt` removes excess newlines, adds spacing and indentation, and always leaves an extra blank line at the end of a file. It also adds semicolons where they are absent and will soon add `()` around conditionals where absent.

For example:
```js
// =========INPUT=========
if (a(b[i])&& b[i] .c    < 0)
    {

    var c = b[i].q
    var l = call();

    b[i].q = c;
    b[i].q[0] = 30;


    }
//=========OUTPUT=========
if (a(b[i]) && b[i].c < 0) {
    var c = b[i].q;
    var l = call();
    
    b[i].q = c;
    b[i].q[0] = 30;
}


```
Since the amount of formatting that `gml_fmt` does is reasonably substantial, it is recommended to download it and try to format some code yourself. It formats code in the style that most GML or JS programmers are familiar with.

If you have used formatters in other languages, such as `prettier` or `rs_fmt` you'll find that this formatter is dumber than those. This is because, unlike those languages, we don't handle line breaks in "chained" phrases (Dot.Acess.Chains or Binary Operator Chains or Accessor[Chains]) because it is exceptionally complex. Asking the user to handle that themselves is reasonable. As a result, `gml_fmt` does not, and could not, enforce a line limit, since to break a line over that line limit, we would have to descibe how to break chained phrases.

Additionally, as a result of this, we allow users to use their own indentation levels in chained phrases. Essentially, this means you can have some wild indentation in `if (x && y)` phrases. 

# Configuration Options

There is a very limited number of configurable things in `gml_fmt`. Add a file called `gml_fmt.toml` or `.gml_fmt.toml` to the directory where `gml_fmt.exe` is location. In the future, you will be able to add it to target locations as well, but that is not supported yet.

The configuration file, like many Rust projects, is in TOML. It is simple to use.

The following options are available:
```toml
use_spaces = boolean
space_size = number
newlines_at_end = number
```
All, or none, of these options may be present. Newlines at end, in particular, refers to how many newlines we will end your file with. The standard configuration (ie, what is chosen if you have no config file) is the following:
```toml
use_spaces = true
space_size = 4
newlines_at_end = 1
```
Future configuration options may be added.

# What do I do if the formatter breaks my code?

Log an issue! To correctly fix any problems, all that is needed is the input code. Output code is appreciated, but can be remade based on the input code. 

To keep using the tool before a fix is made, appending this comment anywhere in a file:
```
// @gml_fmt ignore
```
will ask gml_fmt to ignore that file completely. In the future, line based ignores will be created.

# Contributing

## So how does it work?

Under the hood, `gml_fmt` is really a bad parser. It is a recursive descent parser with extremely loose syntax. After that, it pretty-prints the resulting AST. Both the printer and the parser are single-pass.

Right now, the entire process is single-threaded, but I'm interested in making the printing a second thread and adding messaging between the two. 

The entire program is written in Rust, for speed and for my own learning. We use very few dependencies, but we do use `clap` for handling our command-line interface, `bitflags` for...making bitflags, `fnv` for a small hasher to help with performance, and in the future, we will use `logos` to make a faster lexer. That might not happen, as we currently lex 10k LOC in about 200 micro-seconds, but it might even simplify our lexing, which would always be appreciated. 

## So how do I help?

Please fork, contact me here, submit an issue or a PR, or contact me via Discord at `jack (sanbox)#0001` if you would like to contribute. All contributions are welcome!

The project is simple to build -- `cargo build` will build it and install the necessary dependencies.

To test, see `./benches/samples/small_test.gml` and the built in bash script `run_sample.sh`. Alternatively, run the test suit with `test.sh`. Both require you to have a file `./ignored/output.yaml` (you will need to create the "ignored" directory).

Before submitting a PR, you should run the test suit. Anything that fails the test suit will not be accepted. 

# Current To Do List

## Platforms

It is currently only a CLI, though the following platforms will be supported:

- [x] A simple CLI to autoformat on request.
- [ ] A watcher, spawned by the CLI, to format all .gml files in a project on save.
- [ ] A GMEdit plugin to support formatting without saving.

## Features

- [x] Can handle code which will not compile.
- [x] Extremely fast with few allocations.
- [x] Opinionated. It will have only a few configuration options.
- [ ] Can not yet handle line breaks.
