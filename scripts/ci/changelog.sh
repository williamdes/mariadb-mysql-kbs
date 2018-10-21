#!/bin/bash
npm install --only=dev
./scripts/changelog.js > CHANGELOG.md
git diff --no-prefix CHANGELOG.md
