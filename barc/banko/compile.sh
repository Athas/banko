#!/bin/sh

set -e
cd "$(dirname "$0")"

../dist/build/barc/barc banko.barc > check.c

gcc -Wall -O3 -o check check.c
