'use strict';

process.env.TZ = 'UTC';
const templateUpdate = require(__dirname + '/template-update');
const templateDocs = require(__dirname + '/template-docs');

suite('MariaDB MySQL KBS', function () {
    templateUpdate();
    templateDocs();
});
