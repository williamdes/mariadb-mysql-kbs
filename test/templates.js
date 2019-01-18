'use strict';

const expect = require('chai').expect;
const templates = require(__dirname + '/../scripts/sudo-bot/template.js');

module.exports = function() {
    suite('pr message', function() {
        test('prMessage for lambda files', function(done) {
            const commmitMsg = templates.prMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
            ]);
            expect(commmitMsg).to.equal('🤖 Some updates to review 🤖');
            done();
        });
        test('prMessage for MariaDB files', function(done) {
            const commmitMsg = templates.prMessage([
                'data/mariadb-aria-server-status-variables.json'
            ]);
            expect(commmitMsg).to.equal('🤖 [MariaDB] updates');
            done();
        });
        test('prMessage for MariaDB files and others', function(done) {
            const commmitMsg = templates.prMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
                'data/mariadb-aria-server-status-variables.json'
            ]);
            expect(commmitMsg).to.equal('🤖 [MariaDB] updates 🚨🚨');
            done();
        });
        test('prMessage for MySQL files', function(done) {
            const commmitMsg = templates.prMessage([
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('🤖 [MySQL] updates');
            done();
        });
        test('prMessage for MySQL files and others', function(done) {
            const commmitMsg = templates.prMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('🤖 [MySQL] updates 🚨🚨');
            done();
        });
        test('prMessage for MySQL and MariaDB files', function(done) {
            const commmitMsg = templates.prMessage([
                'data/mariadb-aria-server-status-variables.json',
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('🤖 [MariaDB] && [MySQL] updates');
            done();
        });
        test('prMessage for MySQL and MariaDB files and others', function(done) {
            const commmitMsg = templates.prMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
                'data/mariadb-aria-server-status-variables.json',
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('🤖 [MariaDB] && [MySQL] updates 🚨🚨');
            done();
        });
    });
    suite('commit message', function() {
        test('commitMessage for lambda files', function(done) {
            const commmitMsg = templates.commitMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
            ]);
            expect(commmitMsg).to.equal('🤖 Some updates 🤖');
            done();
        });
        test('commitMessage for MariaDB files', function(done) {
            const commmitMsg = templates.commitMessage([
                'data/mariadb-aria-server-status-variables.json'
            ]);
            expect(commmitMsg).to.equal('[MariaDB] updates');
            done();
        });
        test('commitMessage for MariaDB files and others', function(done) {
            const commmitMsg = templates.commitMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
                'data/mariadb-aria-server-status-variables.json'
            ]);
            expect(commmitMsg).to.equal('[MariaDB] updates and other changes');
            done();
        });
        test('commitMessage for MySQL files', function(done) {
            const commmitMsg = templates.commitMessage([
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('[MySQL] updates');
            done();
        });
        test('commitMessage for MySQL files and others', function(done) {
            const commmitMsg = templates.commitMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('[MySQL] updates and other changes');
            done();
        });
        test('commitMessage for MySQL and MariaDB files', function(done) {
            const commmitMsg = templates.commitMessage([
                'data/mariadb-aria-server-status-variables.json',
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('[MariaDB] && [MySQL] updates');
            done();
        });
        test('commitMessage for MySQL and MariaDB files and others', function(done) {
            const commmitMsg = templates.commitMessage([
                'a.json',
                'ab/cd/ef.json',
                'README.md',
                'data/mariadb-aria-server-status-variables.json',
                'data/mysql-server-options.json'
            ]);
            expect(commmitMsg).to.equal('[MariaDB] && [MySQL] updates and other changes');
            done();
        });
    });
    suite('pr', function() {
        test('prContent', function(done) {
            const prContent = templates.prContent([
                'a.json',
                'ab/cd/ef.json',
                'data/mariadb-aria-server-status-variables.json',
                'data/mysql-server-options.json',
                'README.md'
            ]);
            expect(prContent).to.equal('Dear human 🌻, after running my task the following files where updated:\n- `a.json` 👽\n- `ab/cd/ef.json` 👽\n- `data/mariadb-aria-server-status-variables.json` 🐳\n- `data/mysql-server-options.json` 🐬\n- `README.md` 👽\n');
            done();
        });
        test('prBranch', function(done) {
            const prBranch = templates.prBranch([]);
            expect(prBranch).to.match(/^refs\/heads\/update\/[0-9]{13}$/);
            done();
        });
    });
};
