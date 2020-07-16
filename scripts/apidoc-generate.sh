#!/bin/bash
cd $(dirname $0)/../scripts
echo "Running in : $(pwd)"

echo "Cleaning..."
rm -rf ./../docs/*

echo "Installing..."
curl -O https://doctum.long-term.support/releases/latest/doctum.phar
echo "Parsing..."
php ./doctum.phar --no-interaction parse ./doctum-config.php
echo "Updating..."
php ./doctum.phar --no-interaction --verbose render ./doctum-config.php --force
ERR="$?"
mv ./../docs/build/* ./../docs/
sleep 2
echo "Done !"
exit $ERR
