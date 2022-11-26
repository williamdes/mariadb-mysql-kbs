use crate::data::{Page, PageProcess};

/**
 * Create a doc element
 * @param {Element} element The root element
 * @returns object The doc object
 */
/*
const createDoc = function ($, element) {
    let doc = {
        id: $(element).attr('id'),
        name: $(element).text().trim(),
    };
    if (doc.id === 'select' && doc.name === 'SELECT') {
        // Handle an edge case for https://mariadb.com/kb/en/temporal-data-tables/
        delete doc.id;
    }
    try {
        /* jshint -W083 */
        // Parse ul > li
        const ulElementList = $(element).nextUntil('.anchored_heading');
        if (ulElementList.find('li > strong').length === 0) {
            return { id: null };
        }
        ulElementList.find('li').each((i, elementDescr) => {
            const valueKey = $(elementDescr);
            const key = valueKey.find('strong').text().toLowerCase().trim();
            const value = $(elementDescr).text().replace(valueKey.find('strong').text(), '').trim();
            switch (key) {
                case 'dynamic:':
                    doc.dynamic = value.toLowerCase() === 'yes';
                    break;
                case 'scope:':
                    doc.scope = value
                        .toLowerCase()
                        .split(',')
                        .map((item) => {
                            if (item.match(/session/)) {
                                return 'session';
                            } else if (item.match(/global/)) {
                                return 'global';
                            }
                        });
                    doc.scope = doc.scope.filter(function (e) {
                        return e === 0 || e;
                    });
                    break;
                case 'type:':
                    doc.type = cleaner.cleanType(value.toLowerCase());
                    break;
                case 'data type:':
                    /*
                     * Default method, <li> has a <code> child
                     * Example: <li><strong>Data Type:</strong> <code>numeric</code></li>
                     */
                    let dataType = valueKey.find('code');
                    if (dataType.length > 0) {
                        doc.type = cleaner.cleanType(dataType.first().text().toLowerCase().trim());
                    } else {
                        /*
                         * Fallback method, <li> has text
                         * Example: <li><strong>Data Type:</strong> boolean</li>
                         */
                        let dataType = value.replace(/undefined/gi, '');
                        dataType = dataType.toLowerCase().trim();
                        if (dataType !== '') {
                            doc.type = cleaner.cleanType(dataType);
                        } else if (dataType === '') {
                            console.log('Empty datatype found for : ' + doc.id);
                        } else {
                            console.log('No datatype found for : ' + doc.id);
                        }
                    }
                    break;
                case 'description:':
                    doc.type = cleaner.cleanType(value.toLowerCase());
                    break;
                case 'default value:':
                case 'default:':
                    doc.default = cleaner.cleanDefault(
                        valueKey.text().replace(valueKey.find('strong').text(), '').trim()
                    );
                    break;
                case 'valid values:':
                    if (valueKey.has('code').length > 0) {
                        doc.validValues = valueKey
                            .find('code')
                            .get()
                            .map((el) => $(el).text());
                    }
                    if (valueKey.has('code').length === 0) {
                        doc.validValues = []; // Default if the value is non detected further
                        let cleanValue = cleaner.cleanTextValidValues(value.trim());
                        if (cleanValue !== '') {
                            doc.validValues = cleanValue.split(',').map((el) => el.trim());
                        }
                    }
                    break;
                case 'range:':
                    doc.range = valueKey
                        .find('code')
                        .get()
                        .map((el) => $(el).text());
                    if (doc.range.length === 1) {
                        // try x-y
                        doc.range = doc.range[0].split('-').map((item) => item.trim());
                    }
                    if (doc.range.length === 1) {
                        // try x to y
                        doc.range = doc.range[0].split('to').map((item) => item.trim());
                    }
                    if (doc.range[1] !== undefined) {
                        doc.range[1] = parseFloat(doc.range[1]);
                    }
                    if (doc.range.length === 1) {
                        // try x upwards
                        if (value.includes('upwards')) {
                            doc.range[1] = value;
                        }
                    }
                    // Could be oneday a float
                    doc.range = {
                        from: parseFloat(doc.range[0]),
                        to: doc.range[1],
                    };
                    doc.range = cleaner.cleanRange(doc.range);

                    break;
                case 'commandline:':
                    if (
                        typeof value === 'string' &&
                        value.toLowerCase() !== 'no' &&
                        value.toLowerCase() !== 'none' &&
                        value.toLowerCase() !== 'n/a' &&
                        value.toLowerCase() !== 'no commandline option'
                    ) {
                        doc.cli = cleaner.cleanCli(value, true);
                    }
                    break;
                default:
                    break;
            }
        });
        /* jshint +W083 */
    } catch (e) {
        console.error(e);
        console.log('Error at : #' + doc.id);
    }
    if (doc.type !== undefined) {
        if (doc.type === 'numeric') {
            doc.type = 'integer';
        }
    }
    return doc;
};

function parsePage($, cbSuccess) {
    var anchors = [];
    $('h4.anchored_heading,h3.anchored_heading').each(function (i, el) {
        let doc = createDoc($, el);
        if (doc.id && typeof doc.id === 'string') {
            anchors.push(doc);
        }
    });
    cbSuccess(anchors);
}



module.exports = {
    parsePage: parsePage,
    createDoc: createDoc,
    run: () => {
        /*var pages = [
            {
                url: 'http://7.2.local/Global%20Transaction%20ID%20-%20MariaDB%20Knowledge%20Base.html',
                name: 'gtid-system-variables'
            }
        ]*/
        return common.processDataExtraction(pages, 'mariadb-', parsePage);
    },
};
*/

const KB_URL: &str = "https://mariadb.com/kb/en/library/documentation/";
const STORAGE_ENGINES: [&str; 7] = [
    "s3-storage-engine",
    "aria",
    "myrocks",
    "cassandra",
    "galera-cluster",
    "myisam",
    "connect",
];
const PLUGINS: [&str; 13] = [
    "mariadb-audit-plugin",
    "semisynchronous-replication-plugin",
    "sql-error-log-plugin",
    "authentication-plugin-gssapi",
    "authentication-plugin-pam",
    "aws-key-management-encryption-plugin",
    "cracklib-password-check-plugin",
    "disks-plugin",
    "feedback-plugin",
    "file-key-management-encryption-plugin",
    "query-cache-information-plugin",
    "query-response-time-plugin",
    "simple-password-check-plugin",
];

const SYSTEM_VARIABLES: [&str; 7] = [
    "pbxt",
    "mroonga",
    "tokudb",
    "xtradbinnodb-server",
    "mariadb-audit-plugin",
    "ssltls",
    "performance-schema",
];

const CUSTOM_PAGES: [Page; 9] = [
    Page {
        url: "columns-storage-engines-and-plugins/storage-engines/spider/spider-server-system-variables/",
        name: "spider-server-system-variables",
    },
    Page {
        url: "semisynchronous-replication/",
        name: "semisynchronous-replication-system-variables",
    },
    Page {
        url: "replication-and-binary-log-server-system-variables/",
        name: "replication-and-binary-log-server-system-variables",
    },
    Page {
        url: "gtid/",
        name: "gtid-system-variables",
    },
    Page {
        url: "replication/optimization-and-tuning/system-variables/server-system-variables/",
        name: "server-system-variables",
    },
    Page {
        url: "system-versioned-tables/",
        name: "versioned-tables-system-variables",
    },
    Page {
        url: "handlersocket-configuration-options/",
        name: "handlersocket-configuration-options-variables",
    },
    Page {
        url: "storage-engine-independent-column-compression/",
        name: "storage-engine-independent-column-compression-variables",
    },
    Page {
        url: "user-statistics/",
        name: "user-statistics-variables",
    },
];

const STATUS: [&str; 15] = [
    "server",
    "galera-cluster",
    "aria-server",
    "cassandra",
    "mroonga",
    "spider-server",
    "sphinx",
    "tokudb",
    "xtradbinnodb-server",
    "replication-and-binary-log",
    "oqgraph-system-and",
    "thread-pool-system-and",
    "ssltls",
    "performance-schema",
    "myrocks",
];

pub fn get_pages() -> Vec<PageProcess<'static>> {
    let mut pages = vec![];

    for se in &STORAGE_ENGINES {
        pages.push(PageProcess {
            url: KB_URL.to_owned()
                + "columns-storage-engines-and-plugins/storage-engines/"
                + se
                + "/"
                + se
                + "-system-variables/",
            name: format!("{}-system-variables", se),
            data_type: "variables",
        });
    }

    for cu in &CUSTOM_PAGES {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + cu.url,
            name: cu.name.to_string(),
            data_type: "variables",
        });
    }

    for status_name in &STATUS {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + status_name + "-status-variables/",
            name: format!("{}-status-variables", status_name),
            data_type: "variables",
        });
    }

    for system_variable_name in &SYSTEM_VARIABLES {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + system_variable_name + "-system-variables/",
            name: format!("{}-system-variables", system_variable_name),
            data_type: "variables",
        });
    }

    for plugin_name in &PLUGINS {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + plugin_name + "/",
            name: format!("{}-variables", plugin_name),
            data_type: "variables",
        });
    }

    pages
}
