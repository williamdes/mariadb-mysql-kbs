use crate::data::QueryResponse;
use crate::{mariadb, mysql};
use futures::join;
use reqwest::header::{FROM, USER_AGENT};
use reqwest::Client;
use reqwest::RequestBuilder;

const UA_FROM: &str = "williamdes+mariadb-mysql-kbs@wdes.fr";
const UA: &str = "mariadb-mysql-kbs-bot (+https://github.com/williamdes/mariadb-mysql-kbs; williamdes+mariadb-mysql-kbs@wdes.fr)";

pub async fn extract() {
    println!("Run build...");
    join!(extract_mysql(), extract_mariadb());
    println!("All done.");
    println!("End !");
}

fn add_headers(rb: RequestBuilder) -> RequestBuilder {
    rb.header(FROM, UA_FROM).header(USER_AGENT, UA)
}

async fn get_html_from_url(client: Client, url: &str) -> QueryResponse {
    let mut request = client.get(url);
    request = add_headers(request);

    let response = request
        .send()
        .await
        .expect("Url should be fetched")
        .text()
        .await
        .unwrap();

    QueryResponse {
        body: response,
        url: url,
    }
}

async fn extract_mysql() {
    let client = reqwest::Client::new();
    for page in mysql::get_pages() {
        let response = get_html_from_url(
            client,
            "https://webhook.site/a2682f4d-b178-444e-81c9-d1eaf4138acd",
        )
        .await;
        println!("{}", response.body);
        break;
    }
}

async fn extract_mariadb() {
    mariadb::get_pages();
}

/*
/**
 * Sort the object keys
 * @see https://stackoverflow.com/a/48112249/5155484
 * @param {Object} obj The object
 * @param {Function} arraySorter The sorter callback
 */
const sortObject = function (obj, arraySorter) {
    if (typeof obj !== 'object') {
        return obj;
    }
    if (Array.isArray(obj)) {
        if (arraySorter) {
            obj.sort(arraySorter);
        }
        for (var i = 0; i < obj.length; i++) {
            obj[i] = sortObject(obj[i], arraySorter);
        }
        return obj;
    }
    var temp = {};
    var keys = [];
    for (var key in obj) {
        keys.push(key);
    }
    keys.sort();
    for (var index in keys) {
        temp[keys[index]] = sortObject(obj[keys[index]], arraySorter);
    }
    return temp;
};

const writeJSON = function (filename, data, cbSuccess = null) {
    fs.writeFile(filename, JSON.stringify(sortObject(data), null, 2) + '\n', function (err) {
        if (err) {
            return console.log(err);
        } else {
            if (cbSuccess !== null) {
                cbSuccess();
            }
        }
    });
};

const writePage = function (type, filePrefix, name, url, data, onWriteSuccess) {
    let pageKB = {
        url: url,
        name: name,
        data: data,
    };
    writeJSON(path.join(__dirname, '../', 'data', type, filePrefix + pageKB.name + '.json'), pageKB, onWriteSuccess);
};

const processDataExtraction = function (pages, filePrefix, parsePage) {
    return new Promise((resolve) => {
        var nbrPagesProcessed = 0;
        var crawler = new Crawler({
            maxConnections: 1,
            // This will be called for each crawled page
            callback: function (error, res, done) {
                if (error) {
                    console.log(error);
                } else {
                    console.log('URL : ' + res.options.url);
                    parsePage(res.$, (anchors) => {
                        writePage(res.options.type, filePrefix, res.options.name, res.options.url, anchors, () => {
                            nbrPagesProcessed++;
                            if (nbrPagesProcessed === pages.length) {
                                resolve();
                            }
                        });
                    });
                }
                done();
            },
        });
        crawler.queue(
            pages.map((page) => {
                return { uri: page.url, name: page.name, url: page.url, type: page.type };
            })
        );
    });
};


*/
