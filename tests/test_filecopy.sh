#!/bin/bash

set -ex

FC=target/debug/filecopy

tmpdir=$(mktemp -d)
src="${tmpdir}/src"
dst="${tmpdir}/dst"
dd if=/dev/urandom of="${tmpdir}/src" bs=1024 count=10
${FC} ${src} ${dst}

cmp --silent ${src} ${dst} || {
    echo "Source and destination files are different!"
    exit 1
}

rm -rf ${tmpdir}
