const jsdom = require('jsdom').JSDOM;
const path = require('path');
const writeJSON = require(__dirname+'/common').writeJSON;

function parsePage(url, cbSuccess) {
    var anchors = [];
    jsdom.fromURL(url).then(dom => {
        var window = dom.window;
        var document = window.document;

        const elements = document.getElementsByClassName('anchored_heading');
        for (let i = 0; i < elements.length; i++) {
            let element = elements[i];
            let doc = { id: element.id };
            doc.name = element.childNodes[0].textContent;
            try {
                // Parse ul > li
                element.nextSibling.nextSibling.childNodes.forEach(liChild => {
                    liChild.childNodes.forEach(elementDescr => {
                        if (elementDescr.nodeName.toLocaleLowerCase() === "strong") {

                            switch (elementDescr.innerHTML.toLowerCase().trim()) {
                                case 'dynamic:':
                                    doc.dynamic = elementDescr.nextSibling.textContent.toLowerCase().trim() === "yes";
                                    break;
                                case 'scope:':
                                    doc.scope = elementDescr.nextSibling.textContent
                                    .toLowerCase()
                                    .split(",").map(item => item.trim());
                                    break;
                                case 'type:':
                                    doc.type = elementDescr.nextSibling.textContent
                                    .toLowerCase().trim();
                                    break;
                                case 'data type:':
                                    if (elementDescr.nextSibling.nextSibling != undefined) {
                                        doc.dataType = elementDescr.nextSibling.nextSibling.textContent;
                                    } else {
                                        doc.dataType = elementDescr.nextSibling.textContent;
                                    }
                                    doc.dataType = doc.dataType
                                    .toLowerCase().trim();
                                    break;
                                case 'description:':
                                    doc.type = elementDescr.nextSibling.textContent
                                    .toLowerCase().trim();
                                    break;
                                case 'default value:':
                                    elementDescr.parentNode.childNodes.forEach(codeChild => {
                                        if(codeChild.nodeName.toLowerCase().trim() === "code") {
                                            doc.default = codeChild.textContent;
                                        }
                                    });
                                    break;
                                case 'valid values:':
                                    doc.validValues = [];
                                    elementDescr.parentNode.childNodes.forEach(codeChild => {
                                        if(codeChild.nodeName.toLowerCase().trim() === "code") {
                                            doc.validValues.push(codeChild.textContent);
                                        }
                                    });
                                    break;
                                case 'range:':
                                    doc.range = [];
                                    elementDescr.parentNode.childNodes.forEach(codeChild => {
                                        if(codeChild.nodeName.toLowerCase().trim() === "code") {
                                            doc.range.push(codeChild.textContent);
                                        }
                                    });
                                    if (doc.range.length === 1) {// try x-y
                                        doc.range = doc.range[0].split("-").map(item => item.trim());
                                    }
                                    if (doc.range.length === 1) {// try x to y
                                        doc.range = doc.range[0].split("to").map(item => item.trim());
                                    }
                                    if (doc.range[1] != undefined)
                                        doc.range[1] = parseFloat(doc.range[1]);
                                    if (doc.range.length === 1) {// try x upwards
                                        elementDescr.parentNode.childNodes.forEach(codeChild => {
                                            if(codeChild.nodeName.toLowerCase().trim() === "#text") {
                                                var txtOriginal = codeChild.textContent.trim();
                                                var txtSansUpwards = codeChild.textContent.trim().replace("upwards");
                                                if (txtOriginal.length != txtSansUpwards.length)
                                                doc.range[1] = txtOriginal;
                                            }
                                        });
                                    }
                                    // Could be oneday a float
                                    doc.range = { from: parseFloat(doc.range[0]), to: doc.range[1] };

                                    break;
                                default:
                                    //console.log(elementDescr.innerHTML);
                                    break;
                            }

                        }
                    });
                });
            } catch(e) {
                console.log("Error at : "+url+"#"+doc.id)
            }
            if(element.firstChild.nodeName.toLowerCase() === "code")
                anchors.push(doc);
            //console.log(element.nextSibling.nextSibling.nodeName);
        }
        //console.log(JSON.stringify(anchors));//, null, 2
        cbSuccess(anchors, url);
    });

}

const KB_URL = 'http://kb-mirror.mariadb.com/kb/en/library/documentation/';

parsePage(
    KB_URL+'replication/optimization-and-tuning/system-variables/server-system-variables/',
    (data, url)=> {
        let page = {
            url: url,
            name: 'server-system-variables',
            data: data,
        };
    writeJSON(path.join(__dirname, "../", "data", "mariadb-"+page.name+".json"), page);
});


const storageEngines = [
    'aria', 'myrocks', 'cassandra', 'galera-cluster', 'mroonga', 'myisam', 'tokudb',
    'connect'

];

storageEngines.forEach(se => {
    console.log("Parsing storage engine : "+se);
    console.log("URL : "+KB_URL+'columns-storage-engines-and-plugins/storage-engines/'+se+'/'+se+'-system-variables/');
    parsePage(
        KB_URL+'columns-storage-engines-and-plugins/storage-engines/'+se+'/'+se+'-system-variables/',
        (data, url)=> {
            let page = {
                url: url,
                name: se+'-system-variables',
                data: data,
            };
        writeJSON(path.join(__dirname, "../", "data", "mariadb-"+page.name+".json"), page);
    });
});


const custom = [
    {
        id: 'spider',
        desc: 'spider-server',
        name: 'spider',
    }

];

custom.forEach(cu => {
    console.log("Parsing : ",cu);
    console.log("URL : "+KB_URL+'columns-storage-engines-and-plugins/storage-engines/'+cu.id+'/'+cu.desc+'-system-variables/');
    parsePage(
        KB_URL+'columns-storage-engines-and-plugins/storage-engines/'+cu.id+'/'+cu.desc+'-system-variables/',
        (data, url)=> {
            let page = {
                url: url,
                name: cu.name+'-system-variables',
                data: data,
            };
        writeJSON(path.join(__dirname, "../", "data", "mariadb-"+page.name+".json"), page);
    });
});
