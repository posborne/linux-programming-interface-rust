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

[file]: https://doc.rust-lang.org/std/fs/struct.File.html
[file-open]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[file-create]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[openoptions]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html

## Example: Copying a File

Copying a file can be done as follows using the basic file operations:
[copying](src/filecopy.rs).  Note that for actual use cases the rust
std provides a way for doing this via [`std::fs::copy`][fscopy].

[fscopy]: https://doc.rust-lang.org/std/fs/fn.copy.html

## Advanced File Operations

