'use strict';

const expect = require('chai').expect;
const MySQL = require(__dirname + '/../src/MySQL');
const MariaDB = require(__dirname + '/../src/MariaDB');
const cheerio = require('cheerio');
const fs = require('fs');

module.exports = function () {
    suite('MySQL parser', function () {
        test('test case 1', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_1.html'));
            MySQL.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: '--ndbcluster',
                        default: 'FALSE (Version: NDB 7.5-7.6)',
                        dynamic: false,
                        id: 'option_mysqld_ndbcluster',
                        name: 'ndbcluster',
                    },
                    {
                        cli: '--ndb-allow-copying-alter-table=[ON|OFF]',
                        default: 'ON (Version: NDB 7.5-7.6)',
                        dynamic: true,
                        id: 'option_mysqld_ndb-allow-copying-alter-table',
                        name: 'ndb-allow-copying-alter-table',
                        scope: ['global', 'session'],
                    },
                ]);
                done();
            });
        });
        test('test case 2', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_2.html'));
            MySQL.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: '--binlog-gtid-simple-recovery[={OFF|ON}]',
                        default: 'ON',
                        dynamic: false,
                        id: 'sysvar_binlog_gtid_simple_recovery',
                        name: 'binlog_gtid_simple_recovery',
                        scope: ['global'],
                        type: 'boolean',
                    },
                    {
                        cli: '--enforce-gtid-consistency[=value]',
                        default: 'OFF',
                        dynamic: true,
                        id: 'sysvar_enforce_gtid_consistency',
                        name: 'enforce_gtid_consistency',
                        scope: ['global'],
                        type: 'enumeration',
                        validValues: ['OFF', 'ON', 'WARN'],
                    },
                    {
                        dynamic: false,
                        id: 'sysvar_gtid_executed',
                        name: 'gtid_executed',
                        scope: ['global', 'session'],
                        type: 'string',
                    },
                    {
                        cli: '--gtid-executed-compression-period=#',
                        default: '1000',
                        dynamic: true,
                        id: 'sysvar_gtid_executed_compression_period',
                        name: 'gtid_executed_compression_period',
                        range: {
                            from: 0,
                            to: 4294967295,
                        },
                        scope: ['global'],
                        type: 'integer',
                    },
                    {
                        cli: '--gtid-mode=MODE',
                        default: 'OFF',
                        dynamic: true,
                        id: 'sysvar_gtid_mode',
                        name: 'gtid_mode',
                        scope: ['global'],
                        type: 'enumeration',
                        validValues: ['OFF', 'OFF_PERMISSIVE', 'ON_PERMISSIVE', 'ON'],
                    },
                    {
                        default: 'AUTOMATIC',
                        dynamic: true,
                        id: 'sysvar_gtid_next',
                        name: 'gtid_next',
                        scope: ['session'],
                        type: 'enumeration',
                        validValues: ['AUTOMATIC', 'ANONYMOUS', 'UUID:NUMBER'],
                    },
                    {
                        dynamic: false,
                        id: 'sysvar_gtid_owned',
                        name: 'gtid_owned',
                        scope: ['global', 'session'],
                        type: 'string',
                    },
                    {
                        dynamic: true,
                        id: 'sysvar_gtid_purged',
                        name: 'gtid_purged',
                        scope: ['global'],
                        type: 'string',
                    },
                ]);
                done();
            });
        });
        test('test case 3', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_3.html'));
            MySQL.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        default: 'TRUE (Version: 5.1.51-ndb-7.2.0)',
                        dynamic: true,
                        id: 'sysvar_ndb_join_pushdown',
                        name: 'ndb_join_pushdown',
                        scope: ['global', 'session'],
                    },
                ]);
                done();
            });
        });
        test('test case 4', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_4.html'));
            MySQL.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        id: 'option_mysqld_ndbcluster',
                        name: 'ndbcluster',
                        cli: '--ndbcluster',
                        dynamic: false,
                        type: 'boolean',
                        default: 'OFF (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-allow-copying-alter-table',
                        name: 'ndb-allow-copying-alter-table',
                        cli: '--ndb-allow-copying-alter-table=[ON|OFF]',
                        scope: ['global', 'session'],
                        dynamic: true,
                        type: 'boolean',
                        default: 'ON (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-batch-size',
                        name: 'ndb-batch-size',
                        cli: '--ndb-batch-size=#',
                        scope: ['global'],
                        dynamic: false,
                        type: 'integer',
                        default: '32768 / 0 - 31536000 (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-cluster-connection-pool',
                        name: 'ndb-cluster-connection-pool',
                        cli: '--ndb-cluster-connection-pool=#',
                        scope: ['global'],
                        dynamic: false,
                        default: '1 / 1 - 63 (Version: NDB 7.5-7.6)',
                    },
                    {
                        id: 'option_mysqld_ndb-cluster-connection-pool-nodeids',
                        name: 'ndb-cluster-connection-pool-nodeids',
                        cli: '--ndb-cluster-connection-pool-nodeids=list',
                        scope: ['global'],
                        dynamic: false,
                        type: 'set',
                        default: '/ (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-blob-read-batch-bytes',
                        name: 'ndb-blob-read-batch-bytes',
                        cli: '--ndb-blob-read-batch-bytes=bytes',
                        scope: ['global', 'session'],
                        dynamic: true,
                        type: 'integer',
                        default: '65536 / 0 - 4294967295 (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-blob-write-batch-bytes',
                        name: 'ndb-blob-write-batch-bytes',
                        cli: '--ndb-blob-write-batch-bytes=bytes',
                        scope: ['global', 'session'],
                        dynamic: true,
                        type: 'integer',
                        default: '65536 / 0 - 4294967295 (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-connectstring',
                        name: 'ndb-connectstring',
                        cli: '--ndb-connectstring=connection_string',
                        dynamic: false,
                        type: 'string',
                        default: '(Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-default-column-format',
                        name: 'ndb-default-column-format',
                        cli: '--ndb-default-column-format=[FIXED|DYNAMIC]',
                        scope: ['global'],
                        dynamic: true,
                        type: 'enumeration',
                        default: 'FIXED / FIXED, DYNAMIC (Version: 7.5.4)',
                    },
                    {
                        id: 'option_mysqld_ndb-deferred-constraints',
                        name: 'ndb-deferred-constraints',
                        cli: '--ndb-deferred-constraints=[0|1]',
                        scope: ['global', 'session'],
                        dynamic: true,
                        type: 'integer',
                        default: '0 / 0 - 1 (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-distribution',
                        name: 'ndb-distribution',
                        cli: '--ndb-distribution=[KEYHASH|LINHASH]',
                        scope: ['global'],
                        dynamic: true,
                        type: 'enumeration',
                        default: 'KEYHASH / LINHASH, KEYHASH (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-log-apply-status',
                        name: 'ndb-log-apply-status',
                        cli: '--ndb-log-apply-status',
                        scope: ['global'],
                        dynamic: false,
                        type: 'boolean',
                        default: 'OFF (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-log-empty-epochs',
                        name: 'ndb-log-empty-epochs',
                        cli: '--ndb-log-empty-epochs=[ON|OFF]',
                        scope: ['global'],
                        dynamic: true,
                        type: 'boolean',
                        default: 'OFF (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-log-empty-update',
                        name: 'ndb-log-empty-update',
                        cli: '--ndb-log-empty-update=[ON|OFF]',
                        scope: ['global'],
                        dynamic: true,
                        type: 'boolean',
                        default: 'OFF (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-log-exclusive-reads',
                        name: 'ndb-log-exclusive-reads',
                        cli: '--ndb-log-exclusive-reads=[0|1]',
                        scope: ['global', 'session'],
                        dynamic: true,
                        type: 'boolean',
                        default: '0 (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-log-orig',
                        name: 'ndb-log-orig',
                        cli: '--ndb-log-orig',
                        scope: ['global'],
                        dynamic: false,
                        type: 'boolean',
                        default: 'OFF (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-log-transaction-id',
                        name: 'ndb-log-transaction-id',
                        cli: '--ndb-log-transaction-id',
                        scope: ['global'],
                        dynamic: false,
                        type: 'boolean',
                        default: 'OFF (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-log-update-minimal',
                        name: 'ndb-log-update-minimal',
                        cli: '--ndb-log-update-minimal',
                        scope: ['global'],
                        dynamic: true,
                        type: 'boolean',
                        default: 'OFF (Version: 7.6.3)',
                    },
                    {
                        id: 'option_mysqld_ndb-mgmd-host',
                        name: 'ndb-mgmd-host',
                        cli: '--ndb-mgmd-host=host[:port]',
                        dynamic: false,
                        type: 'string',
                        default: 'localhost:1186 (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-nodeid',
                        name: 'ndb-nodeid',
                        cli: '--ndb-nodeid=#',
                        scope: ['global'],
                        dynamic: false,
                        default: '/ 1 - 255 (Version: 5.1.5)',
                    },
                    {
                        id: 'option_mysqld_ndb-optimization-delay',
                        name: 'ndb-optimization-delay',
                        cli: '--ndb-optimization-delay=milliseconds',
                        scope: ['global'],
                        dynamic: true,
                        type: 'integer',
                        default: '10 / 0 - 100000 (Version: NDB 7.5-7.6)',
                    },
                    {
                        id: 'option_mysqld_ndb-recv-thread-activation-threshold',
                        name: 'ndb-recv-thread-activation-threshold',
                        cli: '--ndb-recv-thread-activation-threshold=threshold',
                        dynamic: false,
                        type: 'integer',
                        default:
                            '8 / 0 (MIN_ACTIVATION_THRESHOLD) - 16, (MAX_ACTIVATION_THRESHOLD) (Version: NDB 7.5-7.6)',
                    },
                    {
                        id: 'option_mysqld_ndb-recv-thread-cpu-mask',
                        name: 'ndb-recv-thread-cpu-mask',
                        cli: '--ndb-recv-thread-cpu-mask=bitmask',
                        dynamic: false,
                        default: '[empty] (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-transid-mysql-connection-map',
                        name: 'ndb-transid-mysql-connection-map',
                        cli: 'ndb-transid-mysql-connection-map=state',
                        dynamic: false,
                        type: 'enumeration',
                        default: 'ON / ON, OFF, FORCE (Version: 5.7)',
                    },
                    {
                        id: 'option_mysqld_ndb-wait-connected',
                        name: 'ndb-wait-connected',
                        cli: '--ndb-wait-connected=seconds',
                        scope: ['global'],
                        dynamic: false,
                        type: 'integer',
                        default: '30 / 0 - 31536000 (Version: NDB 7.5-7.6)',
                    },
                    {
                        id: 'option_mysqld_ndb-wait-setup',
                        name: 'ndb-wait-setup',
                        cli: '--ndb-wait-setup=seconds',
                        scope: ['global'],
                        dynamic: false,
                        type: 'integer',
                        default: '30 / 0 - 31536000 (Version: NDB 7.5-7.6)',
                    },
                    {
                        id: 'option_mysqld_skip-ndbcluster',
                        name: 'skip-ndbcluster',
                        cli: '--skip-ndbcluster',
                        dynamic: false,
                    },
                ]);
                done();
            });
        });
        test('test case 5', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_5.html'));
            MySQL.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        id: 'option_mysqld_mysqlx',
                        cli: '--mysqlx[=value]',
                        type: 'enumeration',
                        default: 'ON',
                        validValues: ['ON', 'OFF', 'FORCE', 'FORCE_PLUS_PERMANENT'],
                        name: 'mysqlx',
                    },
                ]);
                done();
            });
        });
        test('test case 6', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_6.html'));
            MySQL.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: '--auto-increment-increment=#',
                        default: '1',
                        dynamic: true,
                        id: 'sysvar_auto_increment_increment',
                        name: 'auto_increment_increment',
                        range: {
                            from: 1,
                            to: 65535,
                        },
                        scope: ['global', 'session'],
                        type: 'integer',
                    },
                    {
                        cli: '--auto-increment-offset=#',
                        default: '1',
                        dynamic: true,
                        id: 'sysvar_auto_increment_offset',
                        name: 'auto_increment_offset',
                        range: {
                            from: 1,
                            to: 65535,
                        },
                        scope: ['global', 'session'],
                        type: 'integer',
                    },
                ]);
                done();
            });
        });
    });
    suite('MariaDB parser', function () {
        test('test case 1', function (done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mariadb_test_case_1.html'));
            MariaDB.parsePage($, function (resultData) {
                expect(resultData).to.deep.equal([
                    {
                        cli: '--query-cache-size=#',
                        default:
                            '1M (>= MariaDB, 10.1.7), 0 (<= MariaDB 10.1.6), (although frequently given a default value in some setups)',
                        dynamic: true,
                        id: 'query_cache_size',
                        name: 'query_cache_size',
                        scope: ['global'],
                        type: 'integer',
                        validValues: ['0'],
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
                        cli: 'query-cache-strip-comments',
                        default: 'OFF',
                        dynamic: true,
                        id: 'query_cache_strip_comments',
                        name: 'query_cache_strip_comments',
                        scope: ['session', 'global'],
                        type: 'boolean',
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
                        id: 'ssl_accept_renegotiates',
                        name: 'Ssl_accept_renegotiates',
                        scope: ['global'],
                        type: 'integer',
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
                        cli: '--server-audit-events=value',
                        default: 'Empty string',
                        dynamic: true,
                        id: 'server_audit_events',
                        name: 'server_audit_events',
                        scope: ['global'],
                        type: 'string',
                        validValues: [
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
                    {
                        cli: '--server-audit-excl-users=value',
                        default: 'Empty string',
                        dynamic: true,
                        id: 'server_audit_excl_users',
                        name: 'server_audit_excl_users',
                        scope: ['global'],
                        type: 'string',
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
                        dynamic: false,
                        id: 'tokudb_version',
                        name: 'tokudb_version',
                        type: 'string',
                    },
                    {
                        default: '1000',
                        dynamic: true,
                        id: 'tokudb_write_status_frequency',
                        name: 'tokudb_write_status_frequency',
                        range: {
                            from: 0,
                            to: 4294967295,
                        },
                        scope: ['global'],
                        type: 'integer',
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
                        cli: '--rpl-semi-sync-slave-trace_level[=#]',
                        default: '32',
                        dynamic: true,
                        id: 'rpl_semi_sync_slave_trace_level',
                        name: 'rpl_semi_sync_slave_trace_level',
                        range: {
                            from: 0,
                            to: 18446744073709552000,
                        },
                        scope: ['global'],
                        type: 'integer',
                    },
                    {
                        cli: '--rpl-semi-sync-master=value',
                        default: 'ON',
                        id: 'rpl_semi_sync_master',
                        name: 'rpl_semi_sync_master',
                        type: 'enumeration',
                        validValues: ['OFF', 'ON', 'FORCE', 'FORCE_PLUS_PERMANENT'],
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
                        cli: '--wsrep-provider=value',
                        default: 'None',
                        id: 'wsrep_provider',
                        name: 'wsrep_provider',
                        scope: ['global'],
                        type: 'string',
                    },
                ]);
                done();
            });
        });
    });
};
