'use strict';

const expect = require('chai').expect;
const template = require(__dirname + '/../scripts/sudo-bot/template-docs.js');

module.exports = function () {
    suite('pr message', function () {
        test('prMessage for lambda files', function (done) {
            const commmitMsg = template.prMessage(['a.json', 'xyz']);
            expect(commmitMsg).to.equal('🤖 Updated documentation 📚');
            done();
        });
    });
    suite('commit message', function () {
        test('commitMessage for lambda files', function (done) {
            const commmitMsg = template.commitMessage(['a.json', 'xyz']);
            expect(commmitMsg).to.equal('docs: Updated documentation 📚');
            done();
        });
    });
    suite('pr', function () {
        test('prContent renderer.index', function (done) {
            const prContent = template.prContent(['renderer.index']);
            expect(prContent).to.equal(
                'Dear human 🌻🐓🦃🦎🦙🐂🐏🐐🐎🦉, after running my task the following file was updated:\n- `renderer.index` 👁️\n'
            );
            done();
        });
        test('prContent index.html', function (done) {
            const prContent = template.prContent(['index.html']);
            expect(prContent).to.equal(
                'Dear human 🌻🐓🦃🦎🦙🐂🐏🐐🐎🦉, after running my task the following file was updated:\n- `index.html` 👁️\n'
            );
            done();
        });
        test('prBranch', function (done) {
            const prBranch = template.prBranch([]);
            expect(prBranch).to.match(/^refs\/heads\/update\/docs-[0-9]{13}$/);
            done();
        });
    });
};
