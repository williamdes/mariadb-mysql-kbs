const jsdom = require('jsdom').JSDOM;
const path = require('path');
const writeJSON = require(__dirname + '/common').writeJSON;

function parsePage(url, cbSuccess) {
  var anchors = [];
  jsdom.fromURL(url).then(dom => {
    var window = dom.window;
    var document = window.document;
    const elements = document.getElementsByClassName('informaltable');
    for (let i = 0; i < elements.length; i++) {
      let element = elements[i];
      let doc = {};
      if (element.getElementsByTagName('th')[0].textContent != 'Property') {
        continue;
      }

      doc.id = element.parentElement.getElementsByTagName('a')[0].name;
      const regexCli = /([-]{2})([0-9a-z-_]+)/i;
      doc.name = element.parentElement.getElementsByTagName('code')[0].textContent.trim();
      var cli = doc.name.match(regexCli);
      if (cli) {
        // cli format
        doc.name = cli[2].replace(/-/g, '_'); //Try to clean format
      }
      var tbody = element.getElementsByTagName('tbody')[0];

      var trs = tbody.getElementsByTagName('tr');
      for (let i = 0; i < trs.length; i++) {
        const tr = trs[i];
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
            doc.scope = value.textContent
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
          case 'type':
            doc.type = value.textContent.toLowerCase().trim();
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
      anchors.push(doc);
    }
    cbSuccess(anchors, url);
  });
}

const KB_URL = 'https://dev.mysql.com/doc/refman/8.0/en/';
const KB_URL57 = 'https://dev.mysql.com/doc/refman/5.7/en/';

parsePage(KB_URL + 'server-system-variables.html', (data, url) => {
  let page = {
    url: url,
    name: 'server-system-variables',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'innodb-parameters.html', (data, url) => {
  let page = {
    url: url,
    name: 'innodb-parameters',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'performance-schema-system-variables.html', (data, url) => {
  let page = {
    url: url,
    name: 'performance-schema-system-variables',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'x-plugin-system-variables-options.html', (data, url) => {
  let page = {
    url: url,
    name: 'x-plugin-system-variables-options',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'replication-options-binary-log.html', (data, url) => {
  let page = {
    url: url,
    name: 'replication-options-binary-log',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'pluggable-authentication-system-variables.html', (data, url) => {
  let page = {
    url: url,
    name: 'pluggable-authentication-system-variables',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'audit-log-reference.html', (data, url) => {
  let page = {
    url: url,
    name: 'audit-log-reference',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'replication-options-gtids.html', (data, url) => {
  let page = {
    url: url,
    name: 'replication-options-gtids',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'replication-options-slave.html', (data, url) => {
  let page = {
    url: url,
    name: 'replication-options-slave',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'replication-options-master.html', (data, url) => {
  let page = {
    url: url,
    name: 'replication-options-master',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL + 'replication-options.html', (data, url) => {
  let page = {
    url: url,
    name: 'replication-options',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});

parsePage(KB_URL57 + 'mysql-cluster-options-variables.html', (data, url) => {
  let page = {
    url: url,
    name: 'mysql-cluster-options-variables',
    data: data,
  };
  writeJSON(path.join(__dirname, '../', 'data', 'mysql-' + page.name + '.json'), page);
});
