#!/bin/sh

ROOT="$(realpath "$(dirname $0)/../../")"

echo "Running in: ${ROOT}"

cd "${ROOT}"

set -e

moveBuildToTempFolder() {
    # Remove tmp dir, do not upload it
    # Use sudo because build was not ran as the same user
    ${SUDO_BIN:-sudo} rm -rf ./tmp

    TEMP_DIR="$(mktemp -d --suffix=mariadb-mysql-kbs-docs)"
    cp -rp build "${TEMP_DIR}"
    # Remove build dir, do not upload it
    # Use sudo because build was not ran as the same user
    ${SUDO_BIN:-sudo} rm -rf ./build
}

cleanGhPages() {
    git checkout gh-pages
    git ls-files ./ | xargs -r -n 1 rm
    rm -rfd ./*
}

moveBuildFilesToCurrentDir() {
    mv "${TEMP_DIR}"/build/* ./
    rm -rf "${TEMP_DIR}"
}

moveBuildToTempFolder
cleanGhPages
moveBuildFilesToCurrentDir

~/.yarn/bin/sudo-bot --verbose \
    --target-branch gh-pages
    --jwt-file


moveBuildFilesToCurrentDir() {
    mv "${TEMP_DIR}"/build/* ./
    rm -rf "${TEMP_DIR}"
}

extractSecrets() {
    printf '%s' "${GH_APP_JWT_PRIV_PEM_CONTENTS}" > ~/.secret_jwt.pem
    printf '%s' "${GPG_PRIVATE_KEY}" > ~/.private-key.asc
}

flushSecrets() {
    rm ~/.secret_jwt.pem
}

moveBuildToTempFolder
cleanGhPages
moveBuildFilesToCurrentDir

extractSecrets

~/.yarn/bin/sudo-bot --verbose \
    --target-branch 'gh-pages' \
    --jwt-file '~/.secret_jwt.pem' \
    --gh-app-id '17453' \
    --installation-id "${INSTALLATION_ID}" \
    --repository-slug 'williamdes/mariadb-mysql-kbs' \
    --target-branch "${TARGET_BRANCH}" \
    --assign 'williamdes' \
    --template "./scripts/sudo-bot/template-docs.js" \
    --ignore-file "./scripts/sudo-bot/.sudo-bot-ignore" \
    --commit-author-email "sudo-bot@wdes.fr" \
    --commit-author-name "Sudo Bot" \
    --gpg-private-key-file "~/.private-key.asc" \
    --gpg-private-key-passphrase "${GPG_PASSPHRASE}"

flushSecrets
