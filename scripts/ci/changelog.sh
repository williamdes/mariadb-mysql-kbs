#!/bin/bash
yarn install
yarn run changelog-file
git diff --no-prefix CHANGELOG.md
