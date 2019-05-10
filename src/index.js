'use strict';
const MariaDB = require('./MariaDB.js');
const MySQL = require('./MySQL.js');

console.log('Run build...');
//MariaDB.run(),
Promise.all([MySQL.run()])
    .then(() => {
        console.log('All done.');
    })
    .then(() => {
        console.log('End !');
    });
