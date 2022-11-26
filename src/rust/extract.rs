use crate::data::QueryResponse;
use crate::{mariadb, mysql};
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::{FROM, USER_AGENT};

const UA_FROM: &str = "williamdes+mariadb-mysql-kbs@wdes.fr";
const UA: &str = "mariadb-mysql-kbs-bot (+https://github.com/williamdes/mariadb-mysql-kbs; williamdes+mariadb-mysql-kbs@wdes.fr)";

pub fn extract() {
    println!("Run build...");
    extract_mysql();
    extract_mariadb();
    println!("All done.");
    println!("End !");
}

fn add_headers(rb: RequestBuilder) -> RequestBuilder {
    rb.header(FROM, UA_FROM).header(USER_AGENT, UA)
}

fn get_html_from_url(client: Client, url: &str) -> QueryResponse {
    let mut request = client.get(url);
    request = add_headers(request);

    let response = request
        .send()
        .expect("Url should be fetched")
        .text()
        .unwrap();

    QueryResponse {
        body: response,
        url: url,
    }
}

fn extract_mysql() {
    let client = Client::new();
    for page in mysql::get_pages() {
        println!("URL : {}", &page.url);
        let response = get_html_from_url(client, &page.url);
        let data = mysql::extract_mysql_from_text(response);

        let serialized = serde_json::to_string(&data).unwrap();
        println!("serialized = {}", serialized);
        break;
    }
}

fn extract_mariadb() {
    mariadb::get_pages();
}

/*
fn write_json(filename: String, data) {
    fs.writeFile(filename, JSON.stringify(sortObject(data), null, 2) + '\n', function (err) {
        if (err) {
            return eprintln!(err);
        } else {
            if (cbSuccess !== null) {
                cbSuccess();
            }
        }
    });
};

fn write_page(page: PageProcess, filePrefix: String, data, onWriteSuccess) {
    let pageKB = {
        url: url,
        name: name,
        data: data,
    };
    writeJSON(path.join(__dirname, '../', 'data', page_type, filePrefix + pageKB.name + '.json'), pageKB, onWriteSuccess);
};*/

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
