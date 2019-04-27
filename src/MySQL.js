'use strict';

const jsdom = require('jsdom').JSDOM;
const path = require('path');
const writeJSON = require(__dirname + '/common').writeJSON;
const regexCli = /([-]{2})([0-9a-z-_]+)/i;

/**
 * Complete a doc element with info found in table
 * @param {HTMLTableRowElement[]} rows The table rows
 */
function completeDoc(rows, doc) {
    for (let i = 0; i < rows.length; i++) {
        const tr = rows[i];
        var name = tr.firstElementChild.textContent.toLowerCase().trim();
        var value = tr.lastElementChild;
        switch (name) {
            case 'dynamic':
                doc.dynamic = value.textContent.toLowerCase().trim() === 'yes';
                break;
            case 'system variable':
                var theName = value.textContent.toLowerCase().trim();
                if (doc.name !== undefined) {
                    if (doc.name.match(regexCli)) {
                        doc.name = theName;
                    }
                } else {
                    doc.name = theName;
                }
                break;
            case 'scope':
                let scope = value.textContent.toLowerCase();
                if (scope === 'both') {
                    // found on mysql-cluster-options-variables.html
                    doc.scope = ['global', 'session'];
                } else if (scope != '') {
                    doc.scope = scope.split(',').map(item => {
                        if (item.match(/session/)) {
                            return 'session';
                        } else if (item.match(/global/)) {
                            return 'global';
                        } else {
                            return item.trim();
                        }
                    });
                }
                if (doc.scope !== undefined) {
                    doc.scope = doc.scope.filter(function(e) {
                        return e === 0 || e;
                    });
                }
                break;
            case 'type':
                let type = value.textContent.toLowerCase().trim();
                if (type != '') {
                    doc.type = type;
                }
                break;
            case 'default value':
                doc.default = value.textContent.toLowerCase().trim();
                break;
            case 'valid values':
                doc.validValues = [];
                var codes = value.getElementsByTagName('code');
                for (let j = 0; j < codes.length; j++) {
                    const code = codes[j];
                    doc.validValues.push(code.textContent);
                }
                break;
            case 'minimum value':
                if (doc.range == undefined) {
                    doc.range = {};
                }
                doc.range.from = parseFloat(value.textContent.trim());
                break;
            case 'maximum value':
                if (doc.range == undefined) {
                    doc.range = {};
                }
                doc.range.to = parseFloat(value.textContent.trim());
                break;
            case 'command-line format':
                doc.cli = value.textContent.trim();
                break;
        }
    }
}

/**
 * Create a doc element
 * @param {Element} element The root element
 * @returns object The doc object
 */
function createDoc(element, anchors) {
    let doc = {};
    doc.id = element.parentElement.getElementsByTagName('a')[0].name;
    doc.name = element.parentElement.getElementsByTagName('code')[0].textContent.trim();
    var cli = doc.name.match(regexCli);
    if (cli) {
        // cli format
        doc.name = cli[2].replace(/-/g, '_'); //Try to clean format
    }
    var tbody = element.getElementsByTagName('tbody')[0];

    completeDoc(tbody.getElementsByTagName('tr'), doc);

    return doc;
}

function parsePage(url, cbSuccess) {
    console.log('URL : ' + url);
    var anchors = [];
    jsdom.fromURL(url).then(dom => {
        var window = dom.window;
        var document = window.document;
        var elements = document.getElementsByClassName('informaltable');
        for (let i = 0; i < elements.length; i++) {
            let element = elements[i];
            if (element.getElementsByTagName('th')[0].textContent != 'Property') {
                continue;
            }
            anchors.push(createDoc(element));
        }
        elements = document.getElementsByClassName('table');
        for (let i = 0; i < elements.length; i++) {
            let element = elements[i];
            if (element.getElementsByTagName('a')[0].name.match(/-detailtable/) === null) {
                continue;
            }
            anchors.push(createDoc(element));
        }
        cbSuccess(anchors, url);
    });
}

const KB_URL = 'https://dev.mysql.com/doc/refman/8.0/en/';
const KB_URL57 = 'https://dev.mysql.com/doc/refman/5.7/en/';

const pages = [
    {
        url: KB_URL + 'server-system-variables.html',
        name: 'server-system-variables',
    },
    {
        url: KB_URL + 'innodb-parameters.html',
        name: 'innodb-parameters',
    },
    {
        url: KB_URL + 'performance-schema-system-variables.html',
        name: 'performance-schema-system-variables',
    },
    {
        url: KB_URL + 'x-plugin-options-system-variables.html',
        name: 'x-plugin-options-system-variables',
    },
    {
        url: KB_URL + 'replication-options-binary-log.html',
        name: 'replication-options-binary-log',
    },
    {
        url: KB_URL57 + 'replication-options-binary-log.html',
        name: 'replication-options-binary-log_5.7',
    },
    {
        url: KB_URL + 'pluggable-authentication-system-variables.html',
        name: 'pluggable-authentication-system-variables',
    },
    {
        url: KB_URL + 'audit-log-reference.html',
        name: 'audit-log-reference',
    },
    {
        url: KB_URL + 'replication-options-gtids.html',
        name: 'replication-options-gtids',
    },
    {
        url: KB_URL + 'replication-options-slave.html',
        name: 'replication-options-slave',
    },
    {
        url: KB_URL + 'replication-options-master.html',
        name: 'replication-options-master',
    },
    {
        url: KB_URL + 'replication-options.html',
        name: 'replication-options',
    },
    {
        url: KB_URL57 + 'mysql-cluster-options-variables.html',
        name: 'mysql-cluster-options-variables',
    },
    {
        url: KB_URL + 'server-options.html',
        name: 'server-options',
    },
    {
        url: KB_URL + 'version-tokens-reference.html',
        name: 'version-tokens-reference',
    },
];

module.exports = {
    run: () => {
        return new Promise(resolve => {
            var nbrPagesProcessed = 0;
            pages.forEach(page => {
                parsePage(page.url, (data, url) => {
                    let pageKB = {
                        url: url,
                        name: page.name,
                        data: data,
                    };
                    writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + pageKB.name + '.json'), pageKB, () => {
                        nbrPagesProcessed++;
                        if (nbrPagesProcessed === pages.length) {
                            resolve();
                        }
                    });
                });
            });
        });
    },
};
