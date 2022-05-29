#!/bin/sh

set -eu

TMP_DIR="$(mktemp -d mariadb-mysql-kbs_check-data_tmp.XXXXXX)"

echo "Temp folder: ${TMP_DIR}"

echo "Fetching remote data..."
curl -s -# -o "${TMP_DIR}/README.md" https://raw.githubusercontent.com/AlexanderMisel/mariadb-documentation/gh-pages/mariadb-administration/variables-and-modes/full-list-of-mariadb-options-system-and-status-variables/README.md
grep -o 'href=".*"' "${TMP_DIR}/README.md" | sed 's/\(href="\|\">\)//g' | cut -d '/' -f 4 | sort | uniq > "${TMP_DIR}/found.txt"

echo "Resolving urls..."
URLS="$(cat "${TMP_DIR}/found.txt" | awk '$0="https://mariadb.com/kb/en/"$0"/"' | xargs -I {} -d \\n -n1 curl -I -Ls -o /dev/null -w "%{url_effective}\n" {})"
echo "${URLS}" | cut -d '/' -f 6 | sort | uniq > "${TMP_DIR}/found.txt"

echo "Building local data..."
jq -r '.url' "${PWD}/data/"mariadb-* | cut -d '/' -f 8 | sort | uniq > "${TMP_DIR}/found-parts.txt"

echo "Resolving urls..."
jq -r '.url' "${PWD}/data/"mariadb-* | xargs -d \\n -n1 curl -I -Ls -o /dev/null -w "%{url_effective}\n" | cut -d '/' -f 6 | sort | uniq >> "${TMP_DIR}/found-parts.txt"

echo "Found: $(cat "${TMP_DIR}/found.txt" | wc -l) parts"
echo "Found in the data: $(cat "${TMP_DIR}/found-parts.txt" | sort | uniq | wc -l) lines"

echo "Missing:"
set +e
grep -vf "${TMP_DIR}/found-parts.txt" "${TMP_DIR}/found.txt"
set -e
rm -r "${TMP_DIR}"
