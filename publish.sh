#!/bin/bash
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
#
# This script will publish all crates in this repository. This script also provides
# a `--dry-run` option that runs `cargo publish` with the dry-run option.

cmd="cargo publish"
if [ "$1" == "--dry-run" ]; then
    cmd="${cmd} --dry-run"
fi

cd scudo-proc-macros || exit 1
echo "Executing: ${cmd}; in $(pwd)"
$cmd || exit 1
cd ../scudo-sys || exit 1
echo "Executing: ${cmd}; in $(pwd)"
$cmd || exit 1
cd ../scudo || exit 1
echo "Executing: ${cmd}; in $(pwd)"
$cmd || exit 1
cd .. || exit 1
