#!/bin/bash
cd $(dirname $0)/../../
echo "Running in : $(pwd)"
./vendor/bin/phpunit --configuration ./tests/phpunit.xml