#/bin/bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

rm localhost.crt localhost.hex localhost.key

echo -e "\033[32mDeleted localhost certificate!\033[0m"

