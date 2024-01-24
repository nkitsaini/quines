#!/bin/bash
set -e
python3 quine.py > .quine-output
if diff quine.py .quine-output > /dev/null ; then
    echo "The program is a quine."
else
    echo "The program is not a quine."
    diff quine.py .quine-output
    exit 1
fi
