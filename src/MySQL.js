'use strict';

const common = require(__dirname + '/common');
const cleaner = require(__dirname + '/cleaner');
const regexCli = /([-]{2})([0-9a-z-_]+)/i;

/**
 * Complete a doc element with info found in table
 * @param {HTMLTableRowElement[]} rows The table rows
 * @param {Object} doc The doc object
 */
function completeDoc($, rows, doc) {
    $(rows).each((i, elem) => {
        let tds = $(elem).find('td'); // first is key and last is value
        var name = tds
            .first()
            .text()
            .toLowerCase()
            .trim();
        var value = tds.last();
        switch (name) {
            case 'dynamic':
                doc.dynamic =
                    value
                        .text()
                        .toLowerCase()
                        .trim() === 'yes';
                break;
            case 'system variable':
                var theName = value
                    .text()
                    .toLowerCase()
                    .trim();
                if (doc.name !== undefined) {
                    if (doc.name.match(regexCli)) {
                        doc.name = theName;
                    }
                } else {
                    doc.name = theName;
                }
                break;
            case 'scope':
                let scope = value.text().toLowerCase();
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
                let type = value
                    .text()
                    .toLowerCase()
                    .trim();
                if (type != '') {
                    doc.type = cleaner.cleanType(type);
                }
                break;
            case 'default value':
                doc.default = value
                    .text()
                    .toLowerCase()
                    .trim();
                break;
            case 'valid values':
                doc.validValues = $(value)
                    .find('code')
                    .get()
                    .map(el => $(el).text());
                break;
            case 'minimum value':
                if (doc.range == undefined) {
                    doc.range = {};
                }
                doc.range.from = parseFloat(value.text().trim());
                break;
            case 'maximum value':
                if (doc.range == undefined) {
                    doc.range = {};
                }
                doc.range.to = parseFloat(value.text().trim());
                break;
            case 'command-line format':
                doc.cli = cleaner.cleanCli(value.text().trim());
                break;
        }
    });
}

/**
 * Create a doc element
 * @param {Element} element The root element
 * @returns object The doc object
 */
function createDoc($, element) {
    let doc = {};
    doc.id = $(element)
        .parent()
        .find('a')
        .first()
        .attr('name');
    doc.name = $(element)
        .parent()
        .find('code')
        .first()
        .text()
        .trim();
    var cli = doc.name.match(regexCli);
    if (cli) {
        // cli format
        doc.name = cli[2].replace(/-/g, '_'); //Try to clean format
    }

    completeDoc($, $(element).find('tbody > tr'), doc);
    if (doc.range !== undefined) {
        doc.range = cleaner.cleanRange(doc.range);
    }

    return doc;
}

function parsePage($, cbSuccess) {
    var anchors = [];
    $('.informaltable')
        .filter(function(i, elem) {
            var thText = $(elem)
                .find('th')
                .first()
                .text();
            return thText === 'Property';
        })
        .each(function(i, el) {
            anchors.push(createDoc($, el));
        });

    // Find all anchors on the webpage
    $('.table')
        .find('a')
        .filter(function(i, el) {
            var elName = $(el).attr('name');
            return typeof elName === 'string' && elName.match(/-detailtable$/);
        })
        .each(function(i, el) {
            var doc = createDoc(
                $,
                $(el)
                    .parent()
                    .find('table')
                    .get()
            );
            doc.id = $(el)
                .parent()
                .prev()
                .find('a')
                .first()
                .attr('name');
            doc.cli = cleaner.cleanCli(
                $(el)
                    .parent()
                    .prev()
                    .find('code')
                    .first()
                    .text()
                    .trim()
            );
            anchors.push(doc);
        });
    cbSuccess(anchors);
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
        return common.processDataExtraction(pages, 'mysql-', parsePage);
    },
};
