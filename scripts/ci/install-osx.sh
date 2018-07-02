#!/bin/bash

brew install php72
sed -i -e 's/^memory_limit = .*/memory_limit = -1/' /usr/local/etc/php/7.2/php.ini
curl https://getcomposer.org/installer | php
ln -s "$(pwd)/composer.phar" /usr/local/bin/composer
