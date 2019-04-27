'use strict';
const spy = require('./spy.js');
const MariaDB = require('./MariaDB.js');
const MySQL = require('./MySQL.js');

console.log('Run build...');

Promise.all([MariaDB.run(), MySQL.run()])
    .then(() => {
        console.log('All done.');
    })
    .then(() => {
        spy.run().then(() => {
            console.log('End !');
        });
    });
