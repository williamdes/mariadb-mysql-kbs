#!/bin/sh -eu

ROOT="$(realpath "$(dirname $0)/../../")"
HOME="$(realpath ~/)"

echo "Running in: ${ROOT}"
echo "Home: ${HOME}"

cd "${ROOT}"

set -e

moveBuildToTempFolder() {
    # Remove tmp dir, do not upload it
    rm -rf ./tmp

    TEMP_DIR="$(mktemp -d --suffix=mariadb-mysql-kbs-docs)"
    cp -rp build "${TEMP_DIR}"
    # Remove build dir, do not upload it
    rm -rf ./build
}

cleanGhPages() {
    git checkout gh-pages
    git ls-files ./ | xargs -r -n 1 rm -v
    # Delete remaining files
    find ./ -not -path '*/.git*' -not -path '*/.github*' -print -delete
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
    cp ./scripts/sudo-bot/${TEMPLATE_FILE} ~/${TEMPLATE_FILE}
    cp ./scripts/sudo-bot/.sudo-bot-ignore ~/.sudo-bot-ignore
}

flushSecrets() {
    rm ~/.secret_jwt.pem
}

flushUsefullFiles() {
    rm ~/${TEMPLATE_FILE}
    rm ~/.sudo-bot-ignore
}

printf 'Running script...\n'

# Before they do not exist anymore, changing branch in moveBuildToTempFolder
copyUsefullFiles

if [ -z "${SKIP_DOCS_STEPS:-}" ]; then
    moveBuildToTempFolder
    cleanGhPages
    moveBuildFilesToCurrentDir
fi

extractSecrets

printf 'Starting to run sudo-bot\n'

SUDO_BOT="$(yarn global bin sudo-bot)"

# To fix yarn giving the value "/usr/local/bin"
if [ ! -f "${SUDO_BOT}" ]; then
    if [ -f "${SUDO_BOT}/sudo-bot" ]; then
        SUDO_BOT="${SUDO_BOT}/sudo-bot"
    fi
fi

printf 'Found at: %s\n' "${SUDO_BOT}"

$SUDO_BOT --version

printf 'Running...\n'

# Manually expand ~ because NodeJs seems not understanding it
$SUDO_BOT --verbose \
    --jwt-file="${HOME}/.secret_jwt.pem" \
    --gh-app-id='17453' \
    --installation-id="${INSTALLATION_ID}" \
    --repository-slug='williamdes/mariadb-mysql-kbs' \
    --target-branch="${TARGET_BRANCH}" \
    --assign='williamdes' \
    --template="${HOME}/${TEMPLATE_FILE}" \
    --ignore-file="${HOME}/.sudo-bot-ignore" \
    --commit-author-email='sudo-bot@wdes.fr' \
    --commit-author-name='Sudo Bot' \
    --gpg-private-key-file="${HOME}/.private-key.asc" \
    --gpg-private-key-passphrase="${GPG_PASSPHRASE}"

printf 'End.\n'

flushSecrets
flushUsefullFiles
