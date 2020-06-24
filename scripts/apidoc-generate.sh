#!/bin/bash
cd $(dirname $0)/../scripts
echo "Running in : $(pwd)"

echo "Cleaning..."
rm -rf ./../docs

echo "Installing..."
composer require --no-interaction code-lts/doctum:dev-main --prefer-dist --no-progress --no-suggest
echo "Parsing..."
./vendor/bin/doctum.php --no-interaction parse ./doctum-config.php
echo "Updating..."
./vendor/bin/doctum.php --no-interaction --verbose update ./doctum-config.php
ERR="$?"
sleep 2
echo "Done !"
exit $ERR
