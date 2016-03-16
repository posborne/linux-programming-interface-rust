The Linux Programming Interface in Rust
---------------------------------------

This repository contains a number of examples showing how one might go
about accessing various features of a modern Linux (and sometimes
Posix) system from the [Rust Programming Language][rust].

These examples roughly follow the chapters from the most excellent
book, [The Linux Programming Interface][tlpi] by Michael Kerrisk.
Previously, I have provided my solutions for many of the exercises in
that book on [this repo on github][tlpi-exercises].

The original book, understandably, provides all examples in C.  Rust
is a new language coming out of Mozilla (and the open source
community) that is able to target all of the same areas that C has
historically targetted.  That includes systems programming.

[rust]: https://www.rust-lang.org/
[tlpi]: http://man7.org/tlpi/
[tlpi-exercises]: https://github.com/posborne/linux-programming-interface-exercises

External Code
=============

C has been wildly successful, but one of its major downfalls has been
that it is very difficult to reuse C code in a portable fashion.  In
this repository, we will seek to show usage of the various low-level
features of the Linux Programming Interface in rust using, when
possible, safe abstractions on top of the operating system.

The following crates (Rust's name for libraries) will be used widely
in the code of this repository:

* [libc][libc]: Currently an external crate but planned to be part of
  the standard library in the future.  Libc provides a binding to the
  system's libc and also defines many constants that would typically
  be defined by the system's libc.  This is a very thin wrapper.
* [nix][nix]: The nix project builds on top of libc and seeks to
  provide safe APIs to the features of the underlying OS.

Various other crates are used for things that are not central to the
features being demonstrated as makes sense (e.g. argument parsing,
etc.).

[libc][https://github.com/rust-lang/libc]
[nix][https://github.com/nix-rust/nix]

Contributing
============

Contributions to this library are very welcome in whatever form.
Common ways of contributing include:

* Reporting Problems: Open an issue on Github
* Fixing a Problem: Open a PR on Github
* Adding a Feature: Open a PR on Github.  If you are unsure about how
  to approach the problem or if a change is wanted, go ahead an open
  an issue first.

TravisCI is used for continuous integration and an effort is made to
test as much of the code here as possible.

License
=======

Copyright (c) 2016, Paul Osborne <osbpau@gmail.com>

Licensed under the MIT license. The files in this repository may not
be copied, modified, or distributed except according to those terms.
See [LICENSE][license] for more details.

[license]: LICENSE
