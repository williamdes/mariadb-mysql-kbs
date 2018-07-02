const jsdom = require('jsdom').JSDOM;
var dom = new jsdom(`
<div>
<h4 class="anchored_heading" id="mroonga_action_on_fulltext_query_error"><code>mroonga_action_on_fulltext_query_error</code></h4>
<ul start="1"><li><strong>Description:</strong> Action to take when encountering a Mroonga fulltext error.
<ul><li><code>ERROR</code>: Report an error without logging.
</li><li><code>ERROR_AND_LOG</code>: Report an error with logging (the default)
</li><li><code>IGNORE</code>: No logging or reporting - the error is ignored.
</li><li><code>IGNORE_AND_LOG</code>: Log the error without reporting it.
</li></ul>
</li><li><strong>Commandline:</strong> <code class="fixed" style="white-space:pre-wrap">--mroonga-action-on-fulltext-query-error=value</code>
</li><li><strong>Scope:</strong> Global, Session
</li><li><strong>Dynamic:</strong> Yes
</li><li><strong>Data Type:</strong> <code>enum</code>
</li><li><strong>Default Value:</strong> <code>ERROR_AND_LOG</code>
</li></ul>
<h4 class="anchored_heading" id="wsrep_sync_wait"><code>wsrep_sync_wait</code></h4>
<ul start="1"><li><strong>Description:</strong> Setting this variable ensures causality checks will take place before executing an operation of the type specified by the value, ensuring that the statement is executed on a fully synced node. While the check is taking place, new queries are blocked on the node to allow the server to catch up with all updates made in the cluster up to the point where the check was begun. Once reached, the original query is executed on the node. This can result in higher latency. Note that when <a href="#wsrep_dirty_reads">wsrep_dirty_reads</a> is ON, values of wsrep_sync_wait become irrelevant. Sample usage (for a critical read that must have the most up-to-date data) <code>SET SESSION wsrep_sync_wait=1; SELECT ...; SET SESSION wsrep_sync_wait=0;</code>
<ul><li><code>0</code> - Disabled (default)
</li><li><code>1</code> - READ (SELECT and BEGIN/START TRANSACTION). Up until <a href="/kb/en/mariadb-1028-release-notes/">MariaDB 10.2.8</a>, <a href="/kb/en/mariadb-10126-release-notes/">MariaDB 10.1.26</a>, <a href="/kb/en/mariadb-galera-cluster-10031-release-notes/">MariaDB Galera 10.0.31</a> and <a href="/kb/en/mariadb-galera-cluster-5556-release-notes/">MariaDB Galera 5.5.56</a>, also SHOW). This is the same as <a href="#wsrep_causal_reads">wsrep_causal_reads=1</a>.
</li><li><code>2</code> - UPDATE and DELETE; 
</li><li><code>3</code> - READ, UPDATE and DELETE;
</li><li><code>4</code> - INSERT and REPLACE;
</li><li><code>5</code> - READ, INSERT and REPLACE;
</li><li><code>6</code> - UPDATE, DELETE, INSERT and REPLACE;
</li><li><code>7</code> - READ, UPDATE, DELETE, INSERT and REPLACE;
</li><li><code>8</code> - SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>9</code> - READ and SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>10</code> - UPDATE, DELETE and SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>11</code> - READ, UPDATE, DELETE and SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>12</code> - INSERT, REPLACE and SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>13</code> - READ, INSERT, REPLACE and SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>14</code> - UPDATE, DELETE, INSERT, REPLACE and SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>15</code> - READ, UPDATE, DELETE, INSERT, REPLACE and SHOW (from <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li></ul>
</li><li><strong>Commandline:</strong> <code>--wsrep-sync-wait=</code>#
</li><li><strong>Scope:</strong> Global, Session
</li><li><strong>Dynamic:</strong> Yes
</li><li><strong>Data Type:</strong> Numeric
</li><li><strong>Default Value:</strong> <code>0</code>
</li><li><strong>Range:</strong> 
<ul start="1"><li><code>0</code> to <code>15</code> (&gt;= <a href="/kb/en/mariadb-1029-release-notes/">MariaDB 10.2.9</a>, <a href="/kb/en/mariadb-10127-release-notes/">MariaDB 10.1.27</a>, <a href="/kb/en/mariadb-galera-cluster-10032-release-notes/">MariaDB Galera 10.0.32</a>, <a href="/kb/en/mariadb-galera-cluster-5557-release-notes/">MariaDB Galera 5.5.57</a>)
</li><li><code>0</code> to <code>7</code> (&lt;= <a href="/kb/en/mariadb-1028-release-notes/">MariaDB 10.2.8</a>, <a href="/kb/en/mariadb-10126-release-notes/">MariaDB 10.1.26</a>, <a href="/kb/en/mariadb-galera-cluster-10031-release-notes/">MariaDB Galera 10.0.31</a>, <a href="/kb/en/mariadb-galera-cluster-5556-release-notes/">MariaDB Galera 5.5.56</a>)
</li></ul>
</li><li><strong>Introduced:</strong> <a href="/kb/en/mariadb-galera-cluster-10013-release-notes/">MariaDB Galera 10.0.13</a>, <a href="/kb/en/mariadb-galera-cluster-5539-release-notes/">MariaDB Galera 5.5.39</a>
</li></ul>
</div>
`);
    var window = dom.window;
    var document = window.document;
    const elements = document.getElementsByClassName('anchored_heading');
    for (let i = 0; i < elements.length; i++) {
        var element = elements[i];
        var doc = {};
        element.nextSibling.nextSibling.childNodes.forEach(liChild => {
            liChild.childNodes.forEach(elementDescr => {
                if (elementDescr.nodeName.toLocaleLowerCase() === "strong") {

                    switch (elementDescr.innerHTML.toLowerCase().trim()) {
                        case 'data type:':
                                if (elementDescr.nextSibling.nextSibling != undefined) {
                                    doc.dataType = elementDescr.nextSibling.nextSibling.textContent;
                                } else {
                                    doc.dataType = elementDescr.nextSibling.textContent;
                                }
                                doc.dataType = doc.dataType
                                .toLowerCase().trim();
                                console.log(doc.dataType);
                            break;
                        default:
                            //console.log(elementDescr.innerHTML);
                            break;
                    }

                }
            });
        });
    }