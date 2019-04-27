'use strict';

const common = require(__dirname + '/common');
const dataDir = __dirname + '/../data/';
const realTypes = [
    'string',
    'boolean',
    'integer',
    'numeric',
    'enumeration',
    'set',
    'directory name',
    'file name',
    'byte',
];

function clearDoc(doc) {
    if (doc.range !== undefined) {
        // clean range
        if (doc.range.from !== undefined) {
            if (typeof doc.range.from !== 'number') {
                delete doc.range.from;
            }
        }
        if (doc.range.to !== undefined) {
            if (typeof doc.range.to !== 'number' && doc.range.to !== 'upwards') {
                if (typeof doc.range.to === 'string') {
                    if (doc.range.to.match(/upwards/i)) {
                        doc.range.to = 'upwards';
                    } else {
                        delete doc.range.to;
                    }
                } else {
                    delete doc.range.to;
                }
            }
        }
    }
    if (doc.dataType !== undefined) {
        doc.type = '' + doc.dataType;
        if (doc.type === 'numeric') {
            doc.type = 'integer';
        }
        delete doc.dataType;
    }
    if (doc.cli !== undefined) {
        if (doc.cli.match(/<code\>/i) || doc.cli.match(/<\/code\>/i)) {
            doc.cli = doc.cli.replace(/<code\>/gi, '');
            doc.cli = doc.cli.replace(/<\/code\>/gi, '');
            doc.cli = doc.cli.replace(/\>/gi, '');
            doc.cli = doc.cli.replace(/</gi, '');
        }
    }
    if (doc.type === undefined) {
        return;
    }

    if (realTypes.includes(doc.type) === false) {
        if (doc.type.match(/in bytes/i) || doc.type.match(/number of bytes/i) || doc.type.match(/size in mb/i)) {
            doc.type = 'byte';
        } else if (
            doc.type.match(/number of/i) ||
            doc.type.match(/size of/i) ||
            doc.type.match(/in microseconds/i) ||
            doc.type.match(/in seconds/i)
        ) {
            doc.type = 'integer';
        } else {
            delete doc.type;
        }
    }
}

module.exports = {
    run: () => {
        return new Promise(resolve => {
            common.listDirectory(dataDir, files => {
                var totalFilesProcessed = 0;
                const success = () => {
                    totalFilesProcessed++;
                    if (totalFilesProcessed === files.length) {
                        resolve();
                    }
                };
                files.forEach(file => {
                    common.readJSON(dataDir + file, (fileData, path) => {
                        if (fileData.data === undefined) {
                            success();
                            return;
                        }
                        var data = fileData.data;
                        data.forEach(doc => clearDoc(doc));
                        common.writeJSON(path, fileData, success);
                    });
                });
            });
        });
    },
};
