#!/bin/bash
#
# Copyright 2021 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
#
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
