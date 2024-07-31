#!/bin/bash
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
# This script is expected to be run from the root of the `rust-scudo` project.
#
# Usage: pull-scudo.sh

version="llvmorg-18.1.8"  # Keep me up to date!

tmp_repo=$(mktemp -d)
tmp_license=$(mktemp)
scudo_standalone=third_party/llvm-scudo-standalone
git clone --branch $version --depth 1 https://github.com/llvm/llvm-project.git $tmp_repo
mv $scudo_standalone/LICENSE $tmp_license
rm -rf $scudo_standalone
mv $tmp_repo/compiler-rt/lib/scudo/standalone $scudo_standalone
mv $tmp_license $scudo_standalone/LICENSE
rm -rf $tmp_repo

