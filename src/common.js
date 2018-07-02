const fs = require('fs');
exports.writeJSON = function writeJSON(filename, data) {

    fs.writeFile(filename, JSON.stringify(data, null, 2), function(err) {
        if(err) {
            return console.log(err);
        }
    });
};
