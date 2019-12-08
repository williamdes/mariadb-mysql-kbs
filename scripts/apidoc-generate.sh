#!/bin/bash
cd $(dirname $0)/../scripts
echo "Running in : $(pwd)"

echo "Installing..."
composer require --no-interaction sami/sami --prefer-dist --no-progress --no-suggest
echo "Parsing..."
./vendor/bin/sami.php --no-interaction parse ./sami-config.php
echo "Updating..."
./vendor/bin/sami.php --no-interaction --verbose update ./sami-config.php
ERR="$?"
sleep 2
echo "Done !"
exit $ERR
