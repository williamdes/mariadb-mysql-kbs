#!/bin/bash
mkdir ~/.homebrew_logs
export HOMEBREW_LOGS="~/.homebrew_logs"
export HOMEBREW_TEMP="/tmp"
export HOMEBREW_INSTALL_BADGE="🌻"
brew update
brew install openldap libiconv php@7.2 composer
sed -i -e 's/^memory_limit = .*/memory_limit = -1/' /usr/local/etc/php/7.2/php.ini
