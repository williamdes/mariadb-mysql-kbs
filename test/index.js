'use strict';

process.env.TZ = 'UTC';
const templateUpdate = require(__dirname + '/template-update');
const templateDocs = require(__dirname + '/template-docs');
const cleaner = require(__dirname + '/cleaner');
const parser = require(__dirname + '/parser');

suite('MariaDB MySQL KBS', function () {
    templateUpdate();
    templateDocs();
    cleaner();
    parser();
});
