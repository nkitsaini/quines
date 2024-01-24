#!/bin/bash
set -e
rustc quine.rs
./quine > gen_quine1.rs
rustc gen_quine1.rs
./gen_quine1 > gen_quine2.rs
if diff gen_quine2.rs gen_quine1.rs > /dev/null ; then
    echo "The program is a quine."
else
    echo "The program is not a quine."
    diff quine.rs .quine-output
    exit 1
fi
