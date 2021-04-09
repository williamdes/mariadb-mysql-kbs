#!/bin/bash

ROOT="$(realpath "$(dirname $0)/../../")"
HOME="$(realpath ~/)"

echo "Running in: ${ROOT}"
echo "Home: ${HOME}"

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

extractSecrets() {
    printf '%s' "${GH_APP_JWT_PRIV_PEM_CONTENTS}" > ~/.secret_jwt.pem
    printf '%s' "${GPG_PRIVATE_KEY}" > ~/.private-key.asc
}

copyUsefullFiles() {
    cp ./scripts/sudo-bot/template-docs.js ~/template-docs.js
    cp ./scripts/sudo-bot/.sudo-bot-ignore ~/.sudo-bot-ignore
}

flushSecrets() {
    rm ~/.secret_jwt.pem
}

flushUsefullFiles() {
    rm ~/template-docs.js
    rm ~/.sudo-bot-ignore
}

printf 'Running scipt...\n'

# Before they do not exist anymore, changing branch in moveBuildToTempFolder
copyUsefullFiles

moveBuildToTempFolder
cleanGhPages
moveBuildFilesToCurrentDir

extractSecrets

printf 'Starting to run sudo-bot\n'

~/.yarn/bin/sudo-bot --version

printf 'Running...\n'

# Manually expand ~ because NodeJs seems not understanding it
~/.yarn/bin/sudo-bot --verbose \
    --target-branch='gh-pages' \
    --jwt-file="${HOME}/.secret_jwt.pem" \
    --gh-app-id='17453' \
    --installation-id="${INSTALLATION_ID}" \
    --repository-slug='williamdes/mariadb-mysql-kbs' \
    --target-branch="${TARGET_BRANCH}" \
    --assign='williamdes' \
    --template="${HOME}/template-docs.js" \
    --ignore-file="${HOME}/.sudo-bot-ignore" \
    --commit-author-email='sudo-bot@wdes.fr' \
    --commit-author-name='Sudo Bot' \
    --gpg-private-key-file="${HOME}/.private-key.asc" \
    --gpg-private-key-passphrase="${GPG_PASSPHRASE}"

printf 'End.\n'

flushSecrets
flushUsefullFiles
