extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate tlpirust;

use std::fs::{self, File};
use docopt::Docopt;

const USAGE: &'static str = "
File Copy

Usage:
  filecopy <src> <dest>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_src: String,
    arg_dest: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut src = File::open(&args.arg_src).unwrap_or_else(|e| {
        logexit!(1, "Unable to open src path {}: {}", &args.arg_src, e);
    });

    let mut dst = File::create(&args.arg_dest).unwrap_or_else(|e| {
        logexit!(1, "Unable to open dst path {}: {}", &args.arg_dest, e);
    });

    // copy data from src to dest
    std::io::copy(&mut src, &mut dst).unwrap_or_else(|e| {
        logexit!(1, "Error copying data to dest: {}", e);
    });

    // set permissions on destination to match source
    let metadata = src.metadata().unwrap_or_else(|e| {
        logexit!(1, "Unable to get src metadata: {}", e);
    });

    fs::set_permissions(args.arg_dest, metadata.permissions()).unwrap_or_else(|e| {
        logexit!(1, "Unable to set dest permissions: {}", e);
    });
}
