'use strict';

const jsdom = require('jsdom').JSDOM;
const path = require('path');
const writeJSON = require(__dirname + '/common').writeJSON;
const cleaner = require(__dirname + '/cleaner');

function parsePage(url, cbSuccess) {
    var anchors = [];
    jsdom.fromURL(url).then(dom => {
        var window = dom.window;
        var document = window.document;
        const elements = document.getElementsByClassName('anchored_heading');
        for (let i = 0; i < elements.length; i++) {
            let element = elements[i];
            let doc = { id: element.id };
            doc.name = element.childNodes[0].textContent.trim();
            try {
                /* jshint -W083 */
                // Parse ul > li
                element.nextSibling.nextSibling.childNodes.forEach(liChild => {
                    liChild.childNodes.forEach(elementDescr => {
                        if (elementDescr.nodeName.toLocaleLowerCase() === 'strong') {
                            switch (elementDescr.innerHTML.toLowerCase().trim()) {
                                case 'dynamic:':
                                    doc.dynamic = elementDescr.nextSibling.textContent.toLowerCase().trim() === 'yes';
                                    break;
                                case 'scope:':
                                    doc.scope = elementDescr.nextSibling.textContent
                                        .toLowerCase()
                                        .split(',')
                                        .map(item => {
                                            if (item.match(/session/)) {
                                                return 'session';
                                            } else if (item.match(/global/)) {
                                                return 'global';
                                            } else {
                                                return item.trim();
                                            }
                                        });
                                    doc.scope = doc.scope.filter(function(e) {
                                        return e === 0 || e;
                                    });
                                    break;
                                case 'type:':
                                    doc.type = cleaner.cleanType(
                                        elementDescr.nextSibling.textContent.toLowerCase().trim()
                                    );
                                    break;
                                case 'data type:':
                                    /*
                                     * Default method, <li> has a <code> child
                                     * Example: <li><strong>Data Type:</strong> <code>numeric</code></li>
                                     */
                                    let dataType = elementDescr.parentElement.getElementsByTagName('code');
                                    if (dataType[0] !== undefined) {
                                        doc.dataType = dataType[0].textContent.toLowerCase().trim();
                                    } else {
                                        /*
                                         * Fallback method, <li> has text
                                         * Example: <li><strong>Data Type:</strong> boolean</li>
                                         */
                                        let dataType = elementDescr.parentElement.textContent
                                            .replace('Data Type:')
                                            .replace(/undefined/gi, '');
                                        dataType = dataType.toLowerCase().trim();
                                        if (dataType !== '') {
                                            doc.dataType = dataType;
                                        } else if (dataType === '') {
                                            console.log('Empty datatype found for : ' + doc.id);
                                        } else {
                                            console.log('No datatype found for : ' + doc.id);
                                        }
                                    }
                                    break;
                                case 'description:':
                                    doc.type = cleaner.cleanType(
                                        elementDescr.nextSibling.textContent.toLowerCase().trim()
                                    );
                                    break;
                                case 'default value:':
                                    elementDescr.parentNode.childNodes.forEach(codeChild => {
                                        if (codeChild.nodeName.toLowerCase().trim() === 'code') {
                                            doc.default = codeChild.textContent;
                                        }
                                    });
                                    break;
                                case 'valid values:':
                                    doc.validValues = [];
                                    elementDescr.parentNode.childNodes.forEach(codeChild => {
                                        if (codeChild.nodeName.toLowerCase().trim() === 'code') {
                                            doc.validValues.push(codeChild.textContent);
                                        }
                                    });
                                    break;
                                case 'range:':
                                    doc.range = [];
                                    elementDescr.parentNode.childNodes.forEach(codeChild => {
                                        if (codeChild.nodeName.toLowerCase().trim() === 'code') {
                                            doc.range.push(codeChild.textContent);
                                        }
                                    });
                                    if (doc.range.length === 1) {
                                        // try x-y
                                        doc.range = doc.range[0].split('-').map(item => item.trim());
                                    }
                                    if (doc.range.length === 1) {
                                        // try x to y
                                        doc.range = doc.range[0].split('to').map(item => item.trim());
                                    }
                                    if (doc.range[1] !== undefined) {
                                        doc.range[1] = parseFloat(doc.range[1]);
                                    }
                                    if (doc.range.length === 1) {
                                        // try x upwards
                                        elementDescr.parentNode.childNodes.forEach(codeChild => {
                                            if (codeChild.nodeName.toLowerCase().trim() === '#text') {
                                                let rangeText = codeChild.textContent.trim();
                                                if (rangeText.includes('upwards')) {
                                                    doc.range[1] = rangeText;
                                                }
                                            }
                                        });
                                    }
                                    // Could be oneday a float
                                    doc.range = {
                                        from: parseFloat(doc.range[0]),
                                        to: doc.range[1],
                                    };
                                    doc.range = cleaner.cleanRange(doc.range);

                                    break;
                                case 'commandline:':
                                    doc.cli = cleaner.cleanCli(
                                        elementDescr.parentNode.textContent
                                            .toLowerCase()
                                            .replace('commandline: ', '')
                                            .trim()
                                    );
                                    break;
                                default:
                                    //console.log(elementDescr.innerHTML);
                                    break;
                            }
                        }
                    });
                });
                /* jshint +W083 */
            } catch (e) {
                console.error(e);
                console.log('Error at : ' + url + '#' + doc.id);
            }
            if (element.firstChild.nodeName.toLowerCase() === 'code') {
                if (doc.dataType !== undefined) {
                    //FIXME: remove strange hack
                    doc.type = '' + doc.dataType;
                    if (doc.type === 'numeric') {
                        doc.type = 'integer';
                    }
                    doc.type = cleaner.cleanType(doc.type);
                    delete doc.dataType;
                }
                anchors.push(doc);
            }
            //console.log(element.nextSibling.nextSibling.nodeName);
        }
        //console.log(JSON.stringify(anchors));//, null, 2
        cbSuccess(anchors, url);
    });
}

const KB_URL = 'https://mariadb.com/kb/en/library/documentation/';

const storageEngines = ['aria', 'myrocks', 'cassandra', 'galera-cluster', 'mroonga', 'myisam', 'tokudb', 'connect'];

const systemVariables = ['xtradbinnodb-server', 'mariadb-audit-plugin', 'ssltls', 'performance-schema'];

const custom = [
    {
        url: 'columns-storage-engines-and-plugins/storage-engines/spider/spider-server-system-variables/',
        name: 'spider-server-system-variables',
    },
    {
        url: 'semisynchronous-replication/',
        name: 'semisynchronous-replication-system-variables',
    },
    {
        url: 'replication-and-binary-log-server-system-variables/',
        name: 'replication-and-binary-log-server-system-variables',
    },
    {
        url: 'gtid/',
        name: 'gtid-system-variables',
    },
    {
        url: 'gtid/',
        name: 'gtid-system-variables',
    },
    {
        url: 'replication/optimization-and-tuning/system-variables/server-system-variables/',
        name: 'server-system-variables',
    },
    {
        url: 'system-versioned-tables/',
        name: 'versioned-tables-system-variables',
    },
];

const status = [
    'server',
    'galera-cluster',
    'aria-server',
    'cassandra',
    'mroonga',
    'spider-server',
    'sphinx',
    'tokudb',
    'xtradbinnodb-server',
    'replication-and-binary-log',
    'oqgraph-system-and',
    'thread-pool-system-and',
    'ssltls',
    'mariadb-audit-plugin',
    'semisynchronous-replication-plugin',
];

module.exports = {
    run: () => {
        return new Promise(resolve => {
            var nbrPagesProcessed = 0;
            const totalPages = storageEngines.length + custom.length + status.length + systemVariables.length;
            const endSuccessFile = () => {
                nbrPagesProcessed++;
                if (nbrPagesProcessed === totalPages) {
                    resolve();
                }
            };

            storageEngines.forEach(se => {
                console.log();
                console.log(
                    'Parsing storage engine : ' + se,
                    'URL : ' +
                        KB_URL +
                        'columns-storage-engines-and-plugins/storage-engines/' +
                        se +
                        '/' +
                        se +
                        '-system-variables/'
                );
                parsePage(
                    KB_URL +
                        'columns-storage-engines-and-plugins/storage-engines/' +
                        se +
                        '/' +
                        se +
                        '-system-variables/',
                    (data, url) => {
                        let page = {
                            url: url,
                            name: se + '-system-variables',
                            data: data,
                        };
                        writeJSON(
                            path.join(__dirname, '../', 'data', 'mariadb-' + page.name + '.json'),
                            page,
                            endSuccessFile
                        );
                    }
                );
            });

            custom.forEach(cu => {
                console.log('Parsing : ', cu.name, 'URL : ', cu.url);
                parsePage(KB_URL + cu.url, (data, url) => {
                    let page = {
                        url: url,
                        name: cu.name,
                        data: data,
                    };
                    writeJSON(
                        path.join(__dirname, '../', 'data', 'mariadb-' + page.name + '.json'),
                        page,
                        endSuccessFile
                    );
                });
            });

            status.forEach(statusName => {
                console.log('Parsing : ', statusName, 'URL : ' + KB_URL + statusName + '-status-variables/');
                parsePage(KB_URL + statusName + '-status-variables/', (data, url) => {
                    let page = {
                        url: url,
                        name: statusName + '-status-variables',
                        data: data,
                    };
                    writeJSON(
                        path.join(__dirname, '../', 'data', 'mariadb-' + page.name + '.json'),
                        page,
                        endSuccessFile
                    );
                });
            });

            systemVariables.forEach(systemVariableName => {
                console.log(
                    'Parsing : ',
                    systemVariableName,
                    'URL : ' + KB_URL + systemVariableName + '-system-variables/'
                );
                parsePage(KB_URL + systemVariableName + '-system-variables/', (data, url) => {
                    let page = {
                        url: url,
                        name: systemVariableName + '-system-variables',
                        data: data,
                    };
                    writeJSON(
                        path.join(__dirname, '../', 'data', 'mariadb-' + page.name + '.json'),
                        page,
                        endSuccessFile
                    );
                });
            });
        });
    },
};
