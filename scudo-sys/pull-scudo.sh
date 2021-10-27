#!/bin/bash
# This script will pull Scudo standalone files from the llvm project and update
# the standalone directory. This is preferred over git submodule and git subtree
# because those tools do not support mirroring a single directory and LLVM is a
# large project.
#
# This script is expected to be run within the `scudo-sys` crate.
#
# Usage: pull-scudo.sh

version="llvmorg-13.0.0"  # Keep me up to date!

tmp=$(mktemp -d)
git clone --branch $version --depth 1 https://github.com/llvm/llvm-project.git $tmp
mv $tmp/compiler-rt/lib/scudo/standalone scudo-standalone
rm -rf $tmp
