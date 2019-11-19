'use strict';

const expect = require('chai').expect;
const MySQL = require(__dirname + '/../src/MySQL');
const cheerio = require('cheerio');
const fs = require('fs');

module.exports = function() {
    suite('parser', function() {
        test('test case 1', function(done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_1.html'));
            MySQL.parsePage($, function(resultData) {
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
        test('test case 2', function(done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_2.html'));
            MySQL.parsePage($, function(resultData) {
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
        test('test case 3', function(done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_3.html'));
            MySQL.parsePage($, function(resultData) {
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
        test('test case 4', function(done) {
            const $ = cheerio.load(fs.readFileSync(__dirname + '/data/mysql_test_case_4.html'));
            MySQL.parsePage($, function(resultData) {
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
    });
};
