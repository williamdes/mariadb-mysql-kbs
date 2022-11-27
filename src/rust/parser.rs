
    suite('MariaDB parser', function () {
        test('test case 1', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_1.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: Some("--query-cache-size=#".to_string()),
                        default:
                            '1M (>= MariaDB, 10.1.7), 0 (<= MariaDB 10.1.6), (although frequently given a default value in some setups)',
                        dynamic: Some(true),
                        id: Some("query_cache_size".to_string()),
                        name: Some("query_cache_size".to_string()),
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("integer".to_string()),
                        valid_values: ['0'],
                    },
                ]);
                done();
            });
        });
        test('test case 2', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_2.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: Some("query-cache-strip-comments".to_string()),
                        default: Some("OFF".to_string()),
                        dynamic: Some(true),
                        id: Some("query_cache_strip_comments".to_string()),
                        name: Some("query_cache_strip_comments".to_string()),
                        scope: ['session', 'global'],
                        r#type: Some("boolean".to_string()),
                    },
                ]);
                done();
            });
        });
        test('test case 3', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_3.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        id: Some("ssl_accept_renegotiates".to_string()),
                        name: Some("Ssl_accept_renegotiates".to_string()),
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("integer".to_string()),
                    },
                ]);
                done();
            });
        });
        test('test case 4', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_4.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: Some("--server-audit-events=value".to_string()),
                        default: Some("Empty string".to_string()),
                        dynamic: Some(true),
                        id: Some("server_audit_events".to_string()),
                        name: Some("server_audit_events".to_string()),
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("string".to_string()),
                        valid_values: [
                            'CONNECT',
                            'QUERY',
                            'TABLE',
                            'CONNECT',
                            'QUERY',
                            'TABLE',
                            'QUERY_DDL',
                            'QUERY_DML',
                            'CONNECT',
                            'QUERY',
                            'TABLE',
                            'QUERY_DDL',
                            'QUERY_DML',
                            'QUERY_DCL',
                            'CONNECT',
                            'QUERY',
                            'TABLE',
                            'QUERY_DDL',
                            'QUERY_DML',
                            'QUERY_DCL',
                            'QUERY_DML_NO_SELECT',
                        ],
                    },
                    KbParsedEntry {
                        cli: Some("--server-audit-excl-users=value".to_string()),
                        default: Some("Empty string".to_string()),
                        dynamic: Some(true),
                        id: Some("server_audit_excl_users".to_string()),
                        name: Some("server_audit_excl_users".to_string()),
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("string".to_string()),
                    },
                ]);
                done();
            });
        });
        test('test case 5', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_5.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        dynamic: Some(false),
                        id: Some("tokudb_version".to_string()),
                        name: Some("tokudb_version".to_string()),
                        r#type: Some("string".to_string()),
                    },
                    KbParsedEntry {
                        default: Some("1000".to_string()),
                        dynamic: Some(true),
                        id: Some("tokudb_write_status_frequency".to_string()),
                        name: Some("tokudb_write_status_frequency".to_string()),
                        range: {
                            from: 0,
                            to: 4294967295,
                        },
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("integer".to_string()),
                    },
                ]);
                done();
            });
        });
        test('test case 6', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_6.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: Some("--rpl-semi-sync-slave-trace_level[=#]".to_string()),
                        default: Some("32".to_string()),
                        dynamic: Some(true),
                        id: Some("rpl_semi_sync_slave_trace_level".to_string()),
                        name: Some("rpl_semi_sync_slave_trace_level".to_string()),
                        range: {
                            from: 0,
                            to: 18446744073709552000,
                        },
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("integer".to_string()),
                    },
                    KbParsedEntry {
                        cli: Some("--rpl-semi-sync-master=value".to_string()),
                        default: Some("ON".to_string()),
                        id: Some("rpl_semi_sync_master".to_string()),
                        name: Some("rpl_semi_sync_master".to_string()),
                        r#type: Some("enumeration".to_string()),
                        valid_values: ['OFF', 'ON', 'FORCE', 'FORCE_PLUS_PERMANENT'],
                    },
                ]);
                done();
            });
        });
        test('test case 7', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_7.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: Some("--wsrep-provider=value".to_string()),
                        default: Some("None".to_string()),
                        id: Some("wsrep_provider".to_string()),
                        name: Some("wsrep_provider".to_string()),
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("string".to_string()),
                    },
                ]);
                done();
            });
        });
        test('test case 8', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_8.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: Some("--tls-version=value".to_string()),
                        default: Some("TLSv1.1,TLSv1.2,TLSv1.3".to_string()),
                        dynamic: Some(false),
                        id: Some("tls_version".to_string()),
                        name: Some("tls_version".to_string()),
                        scope: Some(vec!["global".to_string()]),
                        r#type: Some("enumeration".to_string()),
                        valid_values: ['TLSv1.0', 'TLSv1.1', 'TLSv1.2', 'TLSv1.3'],
                    },
                ]
};
