#!/bin/bash
if [ "$TRAVIS_EVENT_TYPE" != "cron" ]; then
	echo "Not a travis cron !";
	exit 1;
fi

if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
	echo "Not a travis cron !";
	exit 1;
fi

ME="$(dirname $0)"

BOT_DIR_FILES="$(realpath $ME/../sudo-bot)"

BOT_DIR_GIT="$(realpath $ME/../sudo-bot-git)"

REPO_DIR="$(realpath $ME/../../)"

git clone https://github.com/sudo-bot/sudo-bot.git --depth 1 $BOT_DIR_GIT

REPO="mariadb-mysql-kbs"
OWNER="williamdes"


#INSTALLATION_ID="123456"
JWT_PRIV_KEY_PATH="$BOT_DIR_FILES/sudo.2018-09-12.private-key.pem"
GPG_PRIV_PATH="$BOT_DIR_FILES/privkey.asc"
GPG_PUB_PATH="$BOT_DIR_FILES/pubkey.asc"
#GPG_PRIV_PASSWORD="gpgPasswordHere"

BOT_NAME="Sudo Bot"
BOT_EMAIL="sudo-bot@wdes.fr"

echo -e "JWT_PRIV_KEY_PATH=$JWT_PRIV_KEY_PATH\nGPG_PRIV_PATH=$GPG_PRIV_PATH\nGPG_PUB_PATH=$GPG_PUB_PATH\nGPG_PRIV_PASSWORD=$GPG_PRIV_PASSWORD\nREPO=$REPO\nOWNER=$OWNER\nINSTALLATION_ID=$INSTALLATION_ID\nBOT_NAME=$BOT_NAME\nBOT_EMAIL=$BOT_EMAIL\nREPO_DIR=$REPO_DIR" > $BOT_DIR_GIT/.env

nodejs "$REPO_DIR/src/MySQL.js"
nodejs "$REPO_DIR/src/MariaDB.js"
nodejs "$REPO_DIR/src/spy.js"

php -f "$REPO_DIR/src/merge.php"

cd $BOT_DIR_GIT
npm install
cd $REPO_DIR
nodejs "$BOT_DIR_GIT/index.js"

rm -rf $BOT_DIR_GIT