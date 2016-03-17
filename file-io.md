# File I/O in Rust

Basic File I/O in Rust is a feature provided by the Rust standard
library that generally works the same across all platforms that have
an operating system (e.g. Linux, Windows, OSX, FreeBSD, ...).

## Basic File Operations

[`std::fs::File`][file] is the central structure in rust for
performing operations on Files and can be used to easily allow access
to a file in read-only mode (via [open][file-open]) or write-only mode
(via [create][file-create]).  Opening a file with [open][file-open]
uses the flags `O_RDONLY` and opening with [create][file-create] uses
`O_CREAT | O_TRUNC | O_WRONLY`.

To open files with flags other than these, it is necessary to first
create an [`std::fs::OpenOptions`][openoptions] which acts as a factory
for creating a [`std::fs::File`][file] with the appropriate flags.
The mapping between the methods on `OpenOptions` and the standard
posix file flags should be pretty obvious.  For example, opening a
file with posix flags `O_CREAT | O_APPEND | O_WRONLY` can be done as
follows:

```rust
use std::fs::OpenOptions;

let mut file = OpenOptions::new().create(true)
                                 .append(true)
                                 .write(true)
                                 .open("file.txt");
```

Basic access to the file itself is provided by the following traits:

* [Read][https://doc.rust-lang.org/std/io/trait.Read.html]
* [Write][https://doc.rust-lang.org/std/io/trait.Write.html]
* [Seek][https://doc.rust-lang.org/std/io/trait.Seek.html]

## Example: Copying a File

Copying a file can be done as follows using the basic file operations:
[copying](src/filecopy.rs).  Note that for actual use cases the rust
std provides a way for doing this via [`std::fs::copy`][fscopy].

## Advanced File Operations

Although [OpenOptions][openoptions] exposes a moderate set of options
to the user, there are several flags which can only be supported on
Unix platforms -- these are supported via the
[OpenOptionsExt trait][ooext] which provides a `mode` function which
takes the raw mode as an argument.

The following table goes over the various
open flags that are available and their mapping (or lack of mapping)
to [`std::fs::OpenOptions`][OpenOptions]:

| Flag          | OpenOption                                      |
|---------------|-------------------------------------------------|
| `O_RDONLY`    | [read(true)][oo-read]                           |
| `O_WRONLY`    | [write(true)][oo-write]                         |
| `O_RDWR`      | [write(true)][oo-write] / [read(true)][oo-read] |
| `O_CLOEXEC`   | -                                               |
| `O_CREAT`     | [create(true)][oo-create]                       |
| `O_DIRECT`    | -                                               |
| `O_DIRECTORY` | -                                               |
| `O_EXCL`      | -                                               |
| `O_LARGEFILE` | -                                               |
| `O_NOATIME`   | -                                               |
| `O_NOCTTY`    | -                                               |
| `O_NOFOLLOW`  | -                                               |
| `O_TRUNC`     | [truncate(true)][oo-truncate]                   |
| `O_APPEND`    | [append(true)][oo-append]                       |
| `O_ASYNC`     | -                                               |
| `O_DSYNC`     | -                                               |
| `O_NONBLOCK`  | -                                               |
| `O_SYNC`      | -                                               |

To open a file with options outside of the set exposed by
`OpenOptions`, you can use the `mode` function combined with flag
constants provided by [libc][libc].

```rust
extern crate libc;

// unix prelude pulls in OpenOptionsExt trait
use std::io::unix::prelude::*;
use std::fs::OpenOptions;

let mode = (libc::O_CREAT |
            libc::O_RDWR |
            libc::O_NONBLOCK |
            libc::NO_CTTY);
let mut f = try!(OpenOptions::new().mode(mode).open());
```

## Closing Files

When writing a program in C, it is very important to ensure that
resources that are opened are properly free'd at a later point in
time.  The same is true in Rust, but the patterns for doing so are
different and much safer.

When using a [`std::fs::File`][file] in Rust, the file is
automatically closed whenever the resource is dropped (goes out of
scope).  That is, in general, you never need to worry about forgetting
to close a file ([or socket][forget-to-close-socket]) as the compiler
does it for you.

In some cases, it may be desirable to force a file or other resource
to be closed at a point earlier than the end of the scope for that
resource.  There are two ways this can be achieved:

1. By adding some brackets to clarify the scope/lifetime boundaries
   for the resource.
2. By calling [`drop`][drop] on the resource explicitly.

Of these two, the first option is generally considered to be the
idiomatic way of handling things.  Since the resource should never be
used after being closed, tying its lifetime to the lifetime of the
struct makes perfect sense.

## Getting a File Descriptor

In general, sticking with the [File][file] struct exposed by Rust is
the way to go, but in some cases it may be necessary to get the
underlying file descriptor.  For this, Rust provides the
[`std::os::unix::io::AsRawFd`][asrawfd] trait.  This trait is
implemented for a variety of types including files and provides access
to the underlying file descriptor.  A common pattern for taking a
reference to an object on which the file descriptor needs to be
accessed is to do the following:

```rust
use std::os::unix::prelude::*;

fn requires_fd(file: &AsRawFd) {
    ioctl(file.as_raw_fd(), ...);
}
```

Code using this will not be portable to platforms (such as Windows)
which do not implement this trait.

## Outside the Universal I/O Model: `ioctl()`

Although not used on a typical filesystem file, there are many devices
which expose device files on which the standard read, write, and seek
operations are insufficient in providing all of the functionality
required.

This is where many of those devices turn to the [`ioctl`][ioctl]
system call.  The ioctl system call, unfortunately, provides very
little in the way of semantics or structure and, as such, rust must
expose the `ioctl` system call as [unsafe][unsafe].

The raw [`ioctl`][ioctl] system call is exposed via
[`libc::ioctl`][libc-ioctl].  This is very low-level.  The nix project
provides a slight [higher-level interface to ioctl][nix-ioctl] via the
use of helper macros and functions.  Follow the link to read up on its
usage and to get a little bit more context in how it attempt to hide
the dirty complexity of ioctls.

[file]: https://doc.rust-lang.org/std/fs/struct.File.html
[file-open]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[file-create]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[openoptions]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
[libc]: https://github.com/rust-lang/libc
[fscopy]: https://doc.rust-lang.org/std/fs/fn.copy.html
[ooext]: https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html
[oo-create]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create
[oo-read]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.read
[oo-write]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.write
[oo-truncate]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.truncate
[oo-append]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append
[forget-to-close-socket]: http://blog.skylight.io/rust-means-never-having-to-close-a-socke/
[drop]: https://doc.rust-lang.org/std/mem/fn.drop.html
[asrawfd]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html
[ioctl]: http://man7.org/linux/man-pages/man2/ioctl.2.html
[unsafe]: https://doc.rust-lang.org/book/unsafe.html
[libc-ioctl]: https://doc.rust-lang.org/libc/i686-unknown-linux-gnu/libc/fn.ioctl.html
[nix-ioctl]: http://rustdoc.s3-website-us-east-1.amazonaws.com/nix/master/linux/nix/sys/ioctl/index.html
