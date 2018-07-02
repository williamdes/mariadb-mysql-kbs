#!/usr/bin/env node
/**
 * @see gist.github.com/sergey-shpak/40fe8d2534c5e5941b9db9e28132ca0b
 */
const { exec } = require('child_process');
const Twig = require('twig'), // Twig module
  twig = Twig.twig; // Render function

// Compile 'git log' command
const command = params =>
  `git log --pretty=format:"
    ${params.join(command.format.param)}
    ${command.format.line}"`;

const hash = Math.random() * 10e8; // A random separator ?
command.format = {
  line: hash.toString(36),
  param: +hash.toString(36),
};

const log = (schema, post = (k, v) => v) =>
  new Promise((resolve, reject) => {
    const keys = Object.keys(schema);
    const params = keys.map(key => schema[key]);
    // Execute coomand and parse result
    exec(command(params), (err, stdout) => {
      if (err) reject(err);
      else
        resolve(
          stdout
            .split(command.format.line)
            .filter(line => line.length)
            .map(line =>
              line.split(command.format.param).reduce((obj, value, idx) => {
                const key = keys[idx];
                obj[key] = post(key, value);
                return obj;
              }, {})
            )
        );
    });
  });

const tagName = /tag\:\s*(v[\d|\.]*[-\.][\w]*)\,?/g;
const isTagName = str => {
  const match = tagName.exec(str);
  return match && match[1];
};

var changelog = {};

/**
 * @see www.mikedoesweb.com/2014/javascript-object-next-and-previous-keys/
 */
oFunctions = {};
oFunctions.keys = {};

//NEXT KEY
oFunctions.keys.next = function(o, id) {
  var keys = Object.keys(o),
    idIndex = keys.indexOf(id),
    nextIndex = (idIndex += 1);
  if (nextIndex >= keys.length) {
    //we're at the end, there is no next
    return;
  }
  var nextKey = keys[nextIndex];
  return nextKey;
};

//PREVIOUS KEY
oFunctions.keys.previous = function(o, id) {
  var keys = Object.keys(o),
    idIndex = keys.indexOf(id),
    nextIndex = (idIndex -= 1);
  if (idIndex === 0) {
    //we're at the beginning, there is no previous
    return;
  }
  var nextKey = keys[nextIndex];
  return nextKey;
};

/**
 * get log info with mapped properties to log format
 * @see git-scm.com/docs/git-log#_pretty_formats
 */
log(
  { tag: '%d', note: '%N', msg: '%s', hash: '%h', longHash: '%H' },

  // replace \r\n etc from value
  (key, value) => value.replace(/\s\s/g, '')
).then(records => {
  let tag = (changelog['HEAD'] = []);
  records.forEach(record => {
    const tagName = isTagName(record.tag);
    if (tagName) tag = changelog[tagName] = [];
    tag.push(record);
  });
  links = [];
  versions = [];
  for (version in changelog) {
    const changesAdded = [];
    const changesChanged = [];
    const changesDeprecated = [];
    const changesRemoved = [];
    const changesFixed = [];
    const changesSecurity = [];
    const changesImprove = [];
    links.push({
      name: version.trim().replace('HEAD', 'Unreleased'),
      start:
        oFunctions.keys.next(changelog, version) || '4282724e1e04d6b27d3c0744e1a37a50be740237',
      end: version,
    });
    for (commit in changelog[version]) {
      let changes = [];
      let msg = changelog[version][commit].msg.trim();
      /*if (msg.match(/^v([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})$/gi)) {
        continue;
      }*/
      if (msg.match(/^added:\s/gi) || msg.match(/^add:/gi)) {
        changes = changesAdded;
      }
      if (msg.match(/^changed:/gi) || msg.match(/^update:/gi) || msg.match(/^moved:/gi)) {
        changes = changesChanged;
      }
      if (msg.match(/^deprecated:/gi)) {
        changes = changesDeprecated;
      }
      if (msg.match(/^removed:/gi) || msg.match(/^remove:/gi)) {
        changes = changesRemoved;
      }
      if (msg.match(/^fixed:/gi) || msg.match(/fix:/gi)) {
        changes = changesFixed;
      }
      if (msg.match(/^security:/gi)) {
        changes = changesSecurity;
      }
      if (msg.match(/^improve:/gi) || msg.match(/^improved:/gi)) {
        changes = changesImprove;
      }

      changes.push({
        msg: msg,
        hash: changelog[version][commit].hash.trim(),
        longHash: changelog[version][commit].longHash.trim(),
      });
    }
    versions.push({
      name: version.replace('HEAD', 'Unreleased'),
      changesAdded: changesAdded,
      changesChanged: changesChanged,
      changesDeprecated: changesDeprecated,
      changesRemoved: changesRemoved,
      changesFixed: changesFixed,
      changesSecurity: changesSecurity,
      changesImprove: changesImprove,
    });
  }
  Twig.renderFile(
    __dirname + '/CHANGELOG.twig',
    {
      versions: versions,
      links: links,
      owner: 'williamdes',
      repo: 'mariadb-mysql-kbs',
    },
    (err, html) => {
      console.log(html); // compiled string
    }
  );
});
