#!/bin/sh

set -eu

TMP_DIR="$(mktemp -d mariadb-mysql-kbs_check-data_tmp.XXXXXX)"

echo "Temp folder: ${TMP_DIR}"

curl -s -# -o "${TMP_DIR}/README.md" https://raw.githubusercontent.com/AlexanderMisel/mariadb-documentation/gh-pages/mariadb-administration/variables-and-modes/full-list-of-mariadb-options-system-and-status-variables/README.md

grep -o 'href=".*"' "${TMP_DIR}/README.md" | sed 's/\(href="\|\">\)//g' | cut -d '/' -f 4 | sort | uniq > "${TMP_DIR}/found.txt"
jq -r '.url' "${PWD}/data/"mariadb-* | cut -d '/' -f 8 | sort | uniq > "${TMP_DIR}/found-parts.txt"
echo "Resolving urls..."
jq -r '.url' "${PWD}/data/"mariadb-* | xargs -d \\n -n1 curl -Ls -o /dev/null -w "%{url_effective}\n" | cut -d '/' -f 6 | sort | uniq >> "${TMP_DIR}/found-parts.txt"

echo "Found: $(cat "${TMP_DIR}/found.txt" | wc -l) parts"
echo "Found in the data: $(cat "${TMP_DIR}/found-parts.txt" | wc -l) lines"

echo "Missing:"
set +e
grep -vf "${TMP_DIR}/found-parts.txt" "${TMP_DIR}/found.txt"
set -e
rm -r "${TMP_DIR}"
