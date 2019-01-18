#!/bin/bash
cd $(dirname $0)/../../
echo "Running in : $(pwd)"
npm install
npm run test
