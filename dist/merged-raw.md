# Variables and options
## gssapi_keytab_path
|name|value|
|----|-----|
|Name|`gssapi_keytab_path`|
|Command line|`--gssapi-keytab-path`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`''`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[gssapi_keytab_path](https://mariadb.com/kb/en/authentication-plugin-gssapi/#gssapi_keytab_path)|

## gssapi_principal_name
|name|value|
|----|-----|
|Name|`gssapi_principal_name`|
|Command line|`--gssapi-principal-name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`''`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[gssapi_principal_name](https://mariadb.com/kb/en/authentication-plugin-gssapi/#gssapi_principal_name)|

## gssapi_mech_name
|name|value|
|----|-----|
|Name|`gssapi_mech_name`|
|Command line|`--gssapi-mech-name`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`Negotiate`|
|Dynamic|`false`|
|Valid value(s)|`Kerberos`, `Negotiate`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[gssapi_mech_name](https://mariadb.com/kb/en/authentication-plugin-gssapi/#gssapi_mech_name)|

## gssapi
|name|value|
|----|-----|
|Name|`gssapi`|
|Command line|`--gssapi=value`|
|Type of variable|`enumeration`|
|Default value|`ON`|
|Valid value(s)|`OFF`, `ON`, `FORCE`, `FORCE_PLUS_PERMANENT`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[gssapi](https://mariadb.com/kb/en/authentication-plugin-gssapi/#gssapi)|

## Server_audit_active
|name|value|
|----|-----|
|Name|`Server_audit_active`|
|Type of variable|`boolean`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[server_audit_active](https://mariadb.com/kb/en/mariadb-audit-plugin-status-variables/#server_audit_active)|

## Server_audit_current_log
|name|value|
|----|-----|
|Name|`Server_audit_current_log`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[server_audit_current_log](https://mariadb.com/kb/en/mariadb-audit-plugin-status-variables/#server_audit_current_log)|

## Server_audit_last_error
|name|value|
|----|-----|
|Name|`Server_audit_last_error`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[server_audit_last_error](https://mariadb.com/kb/en/mariadb-audit-plugin-status-variables/#server_audit_last_error)|

## Server_audit_writes_failed
|name|value|
|----|-----|
|Name|`Server_audit_writes_failed`|
|Type of variable|`integer`|
|Default value|`0`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[server_audit_writes_failed](https://mariadb.com/kb/en/mariadb-audit-plugin-status-variables/#server_audit_writes_failed)|

## column_compression_threshold
|name|value|
|----|-----|
|Name|`column_compression_threshold`|
|Command line|`--column-compression-threshold=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`100`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[column_compression_threshold](https://mariadb.com/kb/en/storage-engine-independent-column-compression/#column_compression_threshold)|

## column_compression_zlib_level
|name|value|
|----|-----|
|Name|`column_compression_zlib_level`|
|Command line|`--column-compression-zlib-level=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`6`|
|Dynamic|`true`|
|Range|from: `1` to: `9`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[column_compression_zlib_level](https://mariadb.com/kb/en/storage-engine-independent-column-compression/#column_compression_zlib_level)|

## column_compression_zlib_strategy
|name|value|
|----|-----|
|Name|`column_compression_zlib_strategy`|
|Command line|`--column-compression-zlib-strategy=#`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`DEFAULT_STRATEGY`|
|Dynamic|`true`|
|Valid value(s)|`DEFAULT_STRATEGY`, `FILTERED`, `HUFFMAN_ONLY`, `RLE`, `FIXED`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[column_compression_zlib_strategy](https://mariadb.com/kb/en/storage-engine-independent-column-compression/#column_compression_zlib_strategy)|

## column_compression_zlib_wrap
|name|value|
|----|-----|
|Name|`column_compression_zlib_wrap`|
|Command line|`--column-compression-zlib-wrap{=0|1}`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[column_compression_zlib_wrap](https://mariadb.com/kb/en/storage-engine-independent-column-compression/#column_compression_zlib_wrap)|

## Column_compressions
|name|value|
|----|-----|
|Name|`Column_compressions`|
|Type of variable|`integer`|
|Scope|`global`, `session`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[column_compressions](https://mariadb.com/kb/en/storage-engine-independent-column-compression/#column_compressions)|

## Column_decompressions
|name|value|
|----|-----|
|Name|`Column_decompressions`|
|Type of variable|`integer`|
|Scope|`global`, `session`|

### Documentation(s)
|source|anchor name|
|------|----|
|mariadb.com|[column_decompressions](https://mariadb.com/kb/en/storage-engine-independent-column-compression/#column_decompressions)|

## audit_log
|name|value|
|----|-----|
|Name|`audit_log`|
|Command line|`--audit-log[=value]`|
|Type of variable|`enumeration`|
|Default value|`ON`|
|Valid value(s)|`ON`, `OFF`, `FORCE`, `FORCE_PLUS_PERMANENT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_audit-log](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#option_mysqld_audit-log)|

## audit_log_buffer_size
|name|value|
|----|-----|
|Name|`audit_log_buffer_size`|
|Command line|`--audit-log-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1048576`|
|Dynamic|`false`|
|Range|from: `4096`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_buffer_size)|

## audit_log_compression
|name|value|
|----|-----|
|Name|`audit_log_compression`|
|Command line|`--audit-log-compression=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`NONE`|
|Dynamic|`false`|
|Valid value(s)|`NONE`, `GZIP`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_compression](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_compression)|

## audit_log_connection_policy
|name|value|
|----|-----|
|Name|`audit_log_connection_policy`|
|Command line|`--audit-log-connection-policy=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`ALL`|
|Dynamic|`true`|
|Valid value(s)|`ALL`, `ERRORS`, `NONE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_connection_policy](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_connection_policy)|

## audit_log_current_session
|name|value|
|----|-----|
|Name|`audit_log_current_session`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`depends on filtering policy`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_current_session](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_current_session)|

## audit_log_database
|name|value|
|----|-----|
|Name|`audit_log_database`|
|Command line|`--audit-log-database=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`mysql`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_database](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_database)|

## audit_log_disable
|name|value|
|----|-----|
|Name|`audit_log_disable`|
|Command line|`--audit-log-disable[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_disable](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_disable)|

## audit_log_encryption
|name|value|
|----|-----|
|Name|`audit_log_encryption`|
|Command line|`--audit-log-encryption=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`NONE`|
|Dynamic|`false`|
|Valid value(s)|`NONE`, `AES`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_encryption](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_encryption)|

## audit_log_exclude_accounts
|name|value|
|----|-----|
|Name|`audit_log_exclude_accounts`|
|Command line|`--audit-log-exclude-accounts=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_exclude_accounts](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_exclude_accounts)|

## audit_log_file
|name|value|
|----|-----|
|Name|`audit_log_file`|
|Command line|`--audit-log-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`audit.log`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_file](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_file)|

## audit_log_filter_id
|name|value|
|----|-----|
|Name|`audit_log_filter_id`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`false`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_filter_id](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_filter_id)|

## audit_log_flush
|name|value|
|----|-----|
|Name|`audit_log_flush`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_flush](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_flush)|

## audit_log_flush_interval_seconds
|name|value|
|----|-----|
|Name|`audit_log_flush_interval_seconds`|
|Command line|`--audit-log-flush-interval-seconds[=value]`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_flush_interval_seconds](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_flush_interval_seconds)|

## audit_log_format
|name|value|
|----|-----|
|Name|`audit_log_format`|
|Command line|`--audit-log-format=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`NEW`|
|Dynamic|`false`|
|Valid value(s)|`OLD`, `NEW`, `JSON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_format](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_format)|

## audit_log_format_unix_timestamp
|name|value|
|----|-----|
|Name|`audit_log_format_unix_timestamp`|
|Command line|`--audit-log-format-unix-timestamp[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_format_unix_timestamp](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_format_unix_timestamp)|

## audit_log_include_accounts
|name|value|
|----|-----|
|Name|`audit_log_include_accounts`|
|Command line|`--audit-log-include-accounts=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_include_accounts](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_include_accounts)|

## audit_log_password_history_keep_days
|name|value|
|----|-----|
|Name|`audit_log_password_history_keep_days`|
|Command line|`--audit-log-password-history-keep-days=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_password_history_keep_days](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_password_history_keep_days)|

## audit_log_policy
|name|value|
|----|-----|
|Name|`audit_log_policy`|
|Command line|`--audit-log-policy=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`ALL`|
|Dynamic|`false`|
|Valid value(s)|`ALL`, `LOGINS`, `QUERIES`, `NONE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_policy](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_policy)|

## audit_log_prune_seconds
|name|value|
|----|-----|
|Name|`audit_log_prune_seconds`|
|Command line|`--audit-log-prune-seconds=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_prune_seconds](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_prune_seconds)|

## audit_log_read_buffer_size
|name|value|
|----|-----|
|Name|`audit_log_read_buffer_size`|
|Command line|`--audit-log-read-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`32768`|
|Dynamic|`true`|
|Range|from: `32768` to: `4194304`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_read_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_read_buffer_size)|

## audit_log_rotate_on_size
|name|value|
|----|-----|
|Name|`audit_log_rotate_on_size`|
|Command line|`--audit-log-rotate-on-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_rotate_on_size](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_rotate_on_size)|

## audit_log_statement_policy
|name|value|
|----|-----|
|Name|`audit_log_statement_policy`|
|Command line|`--audit-log-statement-policy=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`ALL`|
|Dynamic|`true`|
|Valid value(s)|`ALL`, `ERRORS`, `NONE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_statement_policy](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_statement_policy)|

## audit_log_strategy
|name|value|
|----|-----|
|Name|`audit_log_strategy`|
|Command line|`--audit-log-strategy=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`ASYNCHRONOUS`|
|Dynamic|`false`|
|Valid value(s)|`ASYNCHRONOUS`, `PERFORMANCE`, `SEMISYNCHRONOUS`, `SYNCHRONOUS`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_strategy](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_strategy)|

## audit_log_max_size
|name|value|
|----|-----|
|Name|`audit_log_max_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_audit_log_max_size](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#sysvar_audit_log_max_size)|

## Audit_log_current_size
|name|value|
|----|-----|
|Name|`Audit_log_current_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_current_size](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_current_size)|

## Audit_log_event_max_drop_size
|name|value|
|----|-----|
|Name|`Audit_log_event_max_drop_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_event_max_drop_size](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_event_max_drop_size)|

## Audit_log_events
|name|value|
|----|-----|
|Name|`Audit_log_events`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_events](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_events)|

## Audit_log_events_filtered
|name|value|
|----|-----|
|Name|`Audit_log_events_filtered`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_events_filtered](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_events_filtered)|

## Audit_log_events_lost
|name|value|
|----|-----|
|Name|`Audit_log_events_lost`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_events_lost](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_events_lost)|

## Audit_log_events_written
|name|value|
|----|-----|
|Name|`Audit_log_events_written`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_events_written](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_events_written)|

## Audit_log_total_size
|name|value|
|----|-----|
|Name|`Audit_log_total_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_total_size](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_total_size)|

## Audit_log_write_waits
|name|value|
|----|-----|
|Name|`Audit_log_write_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Audit_log_write_waits](https://dev.mysql.com/doc/refman/8.0/en/audit-log-reference.html#statvar_Audit_log_write_waits)|

## AuroraDb_commits
|name|value|
|----|-----|
|Name|`AuroraDb_commits`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## AuroraDb_commit_latency
|name|value|
|----|-----|
|Name|`AuroraDb_commit_latency`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## AuroraDb_ddl_stmt_duration
|name|value|
|----|-----|
|Name|`AuroraDb_ddl_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## AuroraDb_select_stmt_duration
|name|value|
|----|-----|
|Name|`AuroraDb_select_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## AuroraDb_insert_stmt_duration
|name|value|
|----|-----|
|Name|`AuroraDb_insert_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## AuroraDb_update_stmt_duration
|name|value|
|----|-----|
|Name|`AuroraDb_update_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## AuroraDb_delete_stmt_duration
|name|value|
|----|-----|
|Name|`AuroraDb_delete_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_binlog_io_cache_allocated
|name|value|
|----|-----|
|Name|`Aurora_binlog_io_cache_allocated`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_binlog_io_cache_read_requests
|name|value|
|----|-----|
|Name|`Aurora_binlog_io_cache_read_requests`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_binlog_io_cache_reads
|name|value|
|----|-----|
|Name|`Aurora_binlog_io_cache_reads`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_enhanced_binlog
|name|value|
|----|-----|
|Name|`Aurora_enhanced_binlog`|
|Type of variable|`boolean`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_external_connection_count
|name|value|
|----|-----|
|Name|`Aurora_external_connection_count`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fast_insert_cache_hits
|name|value|
|----|-----|
|Name|`Aurora_fast_insert_cache_hits`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fast_insert_cache_misses
|name|value|
|----|-----|
|Name|`Aurora_fast_insert_cache_misses`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fts_cache_memory_used
|name|value|
|----|-----|
|Name|`Aurora_fts_cache_memory_used`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_dml_stmt_count
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_dml_stmt_count`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_dml_stmt_duration
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_dml_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_errors_rpc_timeout
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_errors_rpc_timeout`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_errors_session_limit
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_errors_session_limit`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_errors_session_timeout
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_errors_session_timeout`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_open_sessions
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_open_sessions`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_select_stmt_count
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_select_stmt_count`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_master_select_stmt_duration
|name|value|
|----|-----|
|Name|`Aurora_fwd_master_select_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_dml_stmt_count
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_dml_stmt_count`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_dml_stmt_duration
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_dml_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_errors_rpc_timeout
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_errors_rpc_timeout`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_errors_session_limit
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_errors_session_limit`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_errors_session_timeout
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_errors_session_timeout`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_open_sessions
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_open_sessions`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_select_stmt_count
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_select_stmt_count`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_fwd_writer_select_stmt_duration
|name|value|
|----|-----|
|Name|`Aurora_fwd_writer_select_stmt_duration`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_lockmgr_buffer_pool_memory_used
|name|value|
|----|-----|
|Name|`Aurora_lockmgr_buffer_pool_memory_used`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_lockmgr_memory_used
|name|value|
|----|-----|
|Name|`Aurora_lockmgr_memory_used`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_ml_actual_request_cnt
|name|value|
|----|-----|
|Name|`Aurora_ml_actual_request_cnt`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_ml_actual_response_cnt
|name|value|
|----|-----|
|Name|`Aurora_ml_actual_response_cnt`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_ml_cache_hit_cnt
|name|value|
|----|-----|
|Name|`Aurora_ml_cache_hit_cnt`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_ml_logical_request_cnt
|name|value|
|----|-----|
|Name|`Aurora_ml_logical_request_cnt`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_ml_logical_response_cnt
|name|value|
|----|-----|
|Name|`Aurora_ml_logical_response_cnt`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_ml_retry_request_cnt
|name|value|
|----|-----|
|Name|`Aurora_ml_retry_request_cnt`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_ml_single_request_cnt
|name|value|
|----|-----|
|Name|`Aurora_ml_single_request_cnt`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## aurora_oom_avoidance_recovery_state
|name|value|
|----|-----|
|Name|`aurora_oom_avoidance_recovery_state`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## aurora_oom_reserved_mem_enter_kb
|name|value|
|----|-----|
|Name|`aurora_oom_reserved_mem_enter_kb`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## aurora_oom_reserved_mem_exit_kb
|name|value|
|----|-----|
|Name|`aurora_oom_reserved_mem_exit_kb`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## aurora_oom_status
|name|value|
|----|-----|
|Name|`aurora_oom_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_bytes_returned
|name|value|
|----|-----|
|Name|`Aurora_pq_bytes_returned`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_max_concurrent_requests
|name|value|
|----|-----|
|Name|`Aurora_pq_max_concurrent_requests`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_pages_pushed_down
|name|value|
|----|-----|
|Name|`Aurora_pq_pages_pushed_down`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_attempted
|name|value|
|----|-----|
|Name|`Aurora_pq_request_attempted`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_executed
|name|value|
|----|-----|
|Name|`Aurora_pq_request_executed`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_failed
|name|value|
|----|-----|
|Name|`Aurora_pq_request_failed`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_in_progress
|name|value|
|----|-----|
|Name|`Aurora_pq_request_in_progress`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_below_min_rows
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_below_min_rows`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_column_bit
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_column_bit`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_column_geometry
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_column_geometry`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_column_lob
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_column_lob`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_column_virtual
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_column_virtual`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_custom_charset
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_custom_charset`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_fast_ddl
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_fast_ddl`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_few_pages_outside_buffer_pool
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_few_pages_outside_buffer_pool`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_full_text_index
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_full_text_index`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_high_buffer_pool_pct
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_high_buffer_pool_pct`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_index_hint
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_index_hint`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_innodb_table_format
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_innodb_table_format`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_long_trx
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_long_trx`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_no_where_clause
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_no_where_clause`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_range_scan
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_range_scan`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_row_length_too_long
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_row_length_too_long`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_small_table
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_small_table`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_temporary_table
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_temporary_table`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_tx_isolation
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_tx_isolation`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_update_delete_stmts
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_update_delete_stmts`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_unsupported_access
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_unsupported_access`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_not_chosen_unsupported_storage_type
|name|value|
|----|-----|
|Name|`Aurora_pq_request_not_chosen_unsupported_storage_type`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_pq_request_throttled
|name|value|
|----|-----|
|Name|`Aurora_pq_request_throttled`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_repl_bytes_received
|name|value|
|----|-----|
|Name|`Aurora_repl_bytes_received`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_reserved_mem_exceeded_incidents
|name|value|
|----|-----|
|Name|`Aurora_reserved_mem_exceeded_incidents`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## aurora_temptable_max_ram_allocation
|name|value|
|----|-----|
|Name|`aurora_temptable_max_ram_allocation`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## aurora_temptable_ram_allocation
|name|value|
|----|-----|
|Name|`aurora_temptable_ram_allocation`|
|Type of variable|`byte`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_in_memory_relaylog_status
|name|value|
|----|-----|
|Name|`Aurora_in_memory_relaylog_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_in_memory_relaylog_disabled_reason
|name|value|
|----|-----|
|Name|`Aurora_in_memory_relaylog_disabled_reason`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_in_memory_relaylog_fallback_count
|name|value|
|----|-----|
|Name|`Aurora_in_memory_relaylog_fallback_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_in_memory_relaylog_recovery_count
|name|value|
|----|-----|
|Name|`Aurora_in_memory_relaylog_recovery_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_thread_pool_thread_count
|name|value|
|----|-----|
|Name|`Aurora_thread_pool_thread_count`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_tmz_version
|name|value|
|----|-----|
|Name|`Aurora_tmz_version`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## Aurora_zdr_oom_threshold
|name|value|
|----|-----|
|Name|`Aurora_zdr_oom_threshold`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## server_aurora_das_running
|name|value|
|----|-----|
|Name|`server_aurora_das_running`|
|Type of variable|`boolean`|

### Documentation(s)
|source|anchor name|
|------|----|
|docs.aws.amazon.com|[](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Reference.GlobalStatusVars.html#)|

## innodb
|name|value|
|----|-----|
|Name|`innodb`|
|Command line|`--innodb[=value]`|
|Type of variable|`enumeration`|
|Default value|`ON`|
|Valid value(s)|`OFF`, `ON`, `FORCE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_innodb](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#option_mysqld_innodb)|

## innodb_dedicated_server
|name|value|
|----|-----|
|Name|`innodb_dedicated_server`|
|Command line|`--innodb-dedicated-server[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_innodb-dedicated-server](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#option_mysqld_innodb-dedicated-server)|

## innodb_status_file
|name|value|
|----|-----|
|Name|`innodb_status_file`|
|Command line|`--innodb-status-file[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_innodb-status-file](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#option_mysqld_innodb-status-file)|

## daemon_memcached_enable_binlog
|name|value|
|----|-----|
|Name|`daemon_memcached_enable_binlog`|
|Command line|`--daemon-memcached-enable-binlog[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_daemon_memcached_enable_binlog](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_daemon_memcached_enable_binlog)|

## daemon_memcached_engine_lib_name
|name|value|
|----|-----|
|Name|`daemon_memcached_engine_lib_name`|
|Command line|`--daemon-memcached-engine-lib-name=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`innodb_engine.so`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_daemon_memcached_engine_lib_name](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_daemon_memcached_engine_lib_name)|

## daemon_memcached_engine_lib_path
|name|value|
|----|-----|
|Name|`daemon_memcached_engine_lib_path`|
|Command line|`--daemon-memcached-engine-lib-path=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_daemon_memcached_engine_lib_path](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_daemon_memcached_engine_lib_path)|

## daemon_memcached_option
|name|value|
|----|-----|
|Name|`daemon_memcached_option`|
|Command line|`--daemon-memcached-option=options`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|``|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_daemon_memcached_option](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_daemon_memcached_option)|

## daemon_memcached_r_batch_size
|name|value|
|----|-----|
|Name|`daemon_memcached_r_batch_size`|
|Command line|`--daemon-memcached-r-batch-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`false`|
|Range|from: `1` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_daemon_memcached_r_batch_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_daemon_memcached_r_batch_size)|

## daemon_memcached_w_batch_size
|name|value|
|----|-----|
|Name|`daemon_memcached_w_batch_size`|
|Command line|`--daemon-memcached-w-batch-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`false`|
|Range|from: `1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_daemon_memcached_w_batch_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_daemon_memcached_w_batch_size)|

## innodb_adaptive_flushing
|name|value|
|----|-----|
|Name|`innodb_adaptive_flushing`|
|Command line|`--innodb-adaptive-flushing[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_adaptive_flushing](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_adaptive_flushing)|

## innodb_adaptive_flushing_lwm
|name|value|
|----|-----|
|Name|`innodb_adaptive_flushing_lwm`|
|Command line|`--innodb-adaptive-flushing-lwm=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `70`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_adaptive_flushing_lwm](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_adaptive_flushing_lwm)|

## innodb_adaptive_hash_index
|name|value|
|----|-----|
|Name|`innodb_adaptive_hash_index`|
|Command line|`--innodb-adaptive-hash-index[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_adaptive_hash_index](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_adaptive_hash_index)|

## innodb_adaptive_hash_index_parts
|name|value|
|----|-----|
|Name|`innodb_adaptive_hash_index_parts`|
|Command line|`--innodb-adaptive-hash-index-parts=#`|
|Type of variable|`numeric`|
|Scope|`global`|
|Default value|`8`|
|Dynamic|`false`|
|Range|from: `1` to: `512`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_adaptive_hash_index_parts](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_adaptive_hash_index_parts)|

## innodb_adaptive_max_sleep_delay
|name|value|
|----|-----|
|Name|`innodb_adaptive_max_sleep_delay`|
|Command line|`--innodb-adaptive-max-sleep-delay=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`150000`|
|Dynamic|`true`|
|Range|from: `0` to: `1000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_adaptive_max_sleep_delay](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_adaptive_max_sleep_delay)|

## innodb_api_bk_commit_interval
|name|value|
|----|-----|
|Name|`innodb_api_bk_commit_interval`|
|Command line|`--innodb-api-bk-commit-interval=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`5`|
|Dynamic|`true`|
|Range|from: `1` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_api_bk_commit_interval](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_api_bk_commit_interval)|

## innodb_api_disable_rowlock
|name|value|
|----|-----|
|Name|`innodb_api_disable_rowlock`|
|Command line|`--innodb-api-disable-rowlock[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_api_disable_rowlock](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_api_disable_rowlock)|

## innodb_api_enable_binlog
|name|value|
|----|-----|
|Name|`innodb_api_enable_binlog`|
|Command line|`--innodb-api-enable-binlog[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_api_enable_binlog](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_api_enable_binlog)|

## innodb_api_enable_mdl
|name|value|
|----|-----|
|Name|`innodb_api_enable_mdl`|
|Command line|`--innodb-api-enable-mdl[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_api_enable_mdl](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_api_enable_mdl)|

## innodb_api_trx_level
|name|value|
|----|-----|
|Name|`innodb_api_trx_level`|
|Command line|`--innodb-api-trx-level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `3`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_api_trx_level](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_api_trx_level)|

## innodb_autoextend_increment
|name|value|
|----|-----|
|Name|`innodb_autoextend_increment`|
|Command line|`--innodb-autoextend-increment=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`64`|
|Dynamic|`true`|
|Range|from: `1` to: `1000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_autoextend_increment](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_autoextend_increment)|

## innodb_autoinc_lock_mode
|name|value|
|----|-----|
|Name|`innodb_autoinc_lock_mode`|
|Command line|`--innodb-autoinc-lock-mode=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`false`|
|Valid value(s)|`0`, `1`, `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_autoinc_lock_mode](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_autoinc_lock_mode)|

## innodb_background_drop_list_empty
|name|value|
|----|-----|
|Name|`innodb_background_drop_list_empty`|
|Command line|`--innodb-background-drop-list-empty[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_background_drop_list_empty](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_background_drop_list_empty)|

## innodb_buffer_pool_chunk_size
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_chunk_size`|
|Command line|`--innodb-buffer-pool-chunk-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`134217728`|
|Dynamic|`false`|
|Range|from: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_chunk_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_chunk_size)|

## innodb_buffer_pool_debug
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_debug`|
|Command line|`--innodb-buffer-pool-debug[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_debug)|

## innodb_buffer_pool_dump_at_shutdown
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_dump_at_shutdown`|
|Command line|`--innodb-buffer-pool-dump-at-shutdown[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_dump_at_shutdown](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_dump_at_shutdown)|

## innodb_buffer_pool_dump_now
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_dump_now`|
|Command line|`--innodb-buffer-pool-dump-now[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_dump_now](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_dump_now)|

## innodb_buffer_pool_dump_pct
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_dump_pct`|
|Command line|`--innodb-buffer-pool-dump-pct=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`25`|
|Dynamic|`true`|
|Range|from: `1` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_dump_pct](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_dump_pct)|

## innodb_buffer_pool_filename
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_filename`|
|Command line|`--innodb-buffer-pool-filename=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`ib_buffer_pool`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_filename](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_filename)|

## innodb_buffer_pool_in_core_file
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_in_core_file`|
|Command line|`--innodb-buffer-pool-in-core-file[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_in_core_file](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_in_core_file)|

## innodb_buffer_pool_instances
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_instances`|
|Command line|`--innodb-buffer-pool-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`false`|
|Range|from: `1` to: `64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_instances](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_instances)|

## innodb_buffer_pool_load_abort
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_load_abort`|
|Command line|`--innodb-buffer-pool-load-abort[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_load_abort](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_load_abort)|

## innodb_buffer_pool_load_at_startup
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_load_at_startup`|
|Command line|`--innodb-buffer-pool-load-at-startup[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_load_at_startup](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_load_at_startup)|

## innodb_buffer_pool_load_now
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_load_now`|
|Command line|`--innodb-buffer-pool-load-now[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_load_now](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_load_now)|

## innodb_buffer_pool_size
|name|value|
|----|-----|
|Name|`innodb_buffer_pool_size`|
|Command line|`--innodb-buffer-pool-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`134217728`|
|Dynamic|`true`|
|Range|from: `5242880`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_buffer_pool_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_buffer_pool_size)|

## innodb_change_buffer_max_size
|name|value|
|----|-----|
|Name|`innodb_change_buffer_max_size`|
|Command line|`--innodb-change-buffer-max-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`25`|
|Dynamic|`true`|
|Range|from: `0` to: `50`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_change_buffer_max_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_change_buffer_max_size)|

## innodb_change_buffering
|name|value|
|----|-----|
|Name|`innodb_change_buffering`|
|Command line|`--innodb-change-buffering=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`all`|
|Dynamic|`true`|
|Valid value(s)|`none`, `inserts`, `deletes`, `changes`, `purges`, `all`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_change_buffering](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_change_buffering)|

## innodb_change_buffering_debug
|name|value|
|----|-----|
|Name|`innodb_change_buffering_debug`|
|Command line|`--innodb-change-buffering-debug=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_change_buffering_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_change_buffering_debug)|

## innodb_checkpoint_disabled
|name|value|
|----|-----|
|Name|`innodb_checkpoint_disabled`|
|Command line|`--innodb-checkpoint-disabled[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_checkpoint_disabled](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_checkpoint_disabled)|

## innodb_checksum_algorithm
|name|value|
|----|-----|
|Name|`innodb_checksum_algorithm`|
|Command line|`--innodb-checksum-algorithm=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`crc32`|
|Dynamic|`true`|
|Valid value(s)|`crc32`, `strict_crc32`, `innodb`, `strict_innodb`, `none`, `strict_none`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_checksum_algorithm](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_checksum_algorithm)|

## innodb_cmp_per_index_enabled
|name|value|
|----|-----|
|Name|`innodb_cmp_per_index_enabled`|
|Command line|`--innodb-cmp-per-index-enabled[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_cmp_per_index_enabled](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_cmp_per_index_enabled)|

## innodb_commit_concurrency
|name|value|
|----|-----|
|Name|`innodb_commit_concurrency`|
|Command line|`--innodb-commit-concurrency=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_commit_concurrency](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_commit_concurrency)|

## innodb_compress_debug
|name|value|
|----|-----|
|Name|`innodb_compress_debug`|
|Command line|`--innodb-compress-debug=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`none`|
|Dynamic|`true`|
|Valid value(s)|`none`, `zlib`, `lz4`, `lz4hc`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_compress_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_compress_debug)|

## innodb_compression_failure_threshold_pct
|name|value|
|----|-----|
|Name|`innodb_compression_failure_threshold_pct`|
|Command line|`--innodb-compression-failure-threshold-pct=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`5`|
|Dynamic|`true`|
|Range|from: `0` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_compression_failure_threshold_pct](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_compression_failure_threshold_pct)|

## innodb_compression_level
|name|value|
|----|-----|
|Name|`innodb_compression_level`|
|Command line|`--innodb-compression-level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`6`|
|Dynamic|`true`|
|Range|from: `0` to: `9`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_compression_level](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_compression_level)|

## innodb_compression_pad_pct_max
|name|value|
|----|-----|
|Name|`innodb_compression_pad_pct_max`|
|Command line|`--innodb-compression-pad-pct-max=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`50`|
|Dynamic|`true`|
|Range|from: `0` to: `75`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_compression_pad_pct_max](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_compression_pad_pct_max)|

## innodb_concurrency_tickets
|name|value|
|----|-----|
|Name|`innodb_concurrency_tickets`|
|Command line|`--innodb-concurrency-tickets=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`5000`|
|Dynamic|`true`|
|Range|from: `1` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_concurrency_tickets](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_concurrency_tickets)|

## innodb_data_file_path
|name|value|
|----|-----|
|Name|`innodb_data_file_path`|
|Command line|`--innodb-data-file-path=file_name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`ibdata1:12M:autoextend`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_data_file_path](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_data_file_path)|

## innodb_data_home_dir
|name|value|
|----|-----|
|Name|`innodb_data_home_dir`|
|Command line|`--innodb-data-home-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_data_home_dir](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_data_home_dir)|

## innodb_ddl_buffer_size
|name|value|
|----|-----|
|Name|`innodb_ddl_buffer_size`|
|Command line|`--innodb-ddl-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`1048576`|
|Dynamic|`true`|
|Range|from: `65536` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ddl_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ddl_buffer_size)|

## innodb_ddl_log_crash_reset_debug
|name|value|
|----|-----|
|Name|`innodb_ddl_log_crash_reset_debug`|
|Command line|`--innodb-ddl-log-crash-reset-debug[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ddl_log_crash_reset_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ddl_log_crash_reset_debug)|

## innodb_ddl_threads
|name|value|
|----|-----|
|Name|`innodb_ddl_threads`|
|Command line|`--innodb-ddl-threads=#`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`4`|
|Dynamic|`true`|
|Range|from: `1` to: `64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ddl_threads](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ddl_threads)|

## innodb_deadlock_detect
|name|value|
|----|-----|
|Name|`innodb_deadlock_detect`|
|Command line|`--innodb-deadlock-detect[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_deadlock_detect](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_deadlock_detect)|

## innodb_default_row_format
|name|value|
|----|-----|
|Name|`innodb_default_row_format`|
|Command line|`--innodb-default-row-format=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`DYNAMIC`|
|Dynamic|`true`|
|Valid value(s)|`REDUNDANT`, `COMPACT`, `DYNAMIC`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_default_row_format](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_default_row_format)|

## innodb_directories
|name|value|
|----|-----|
|Name|`innodb_directories`|
|Command line|`--innodb-directories=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_directories](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_directories)|

## innodb_disable_sort_file_cache
|name|value|
|----|-----|
|Name|`innodb_disable_sort_file_cache`|
|Command line|`--innodb-disable-sort-file-cache[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_disable_sort_file_cache](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_disable_sort_file_cache)|

## innodb_doublewrite
|name|value|
|----|-----|
|Name|`innodb_doublewrite`|
|Command line|`--innodb-doublewrite=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|
|Valid value(s)|`ON`, `OFF`, `DETECT_AND_RECOVER`, `DETECT_ONLY`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_doublewrite](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_doublewrite)|

## innodb_doublewrite_batch_size
|name|value|
|----|-----|
|Name|`innodb_doublewrite_batch_size`|
|Command line|`--innodb-doublewrite-batch-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`false`|
|Range|from: `0` to: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_doublewrite_batch_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_doublewrite_batch_size)|

## innodb_doublewrite_dir
|name|value|
|----|-----|
|Name|`innodb_doublewrite_dir`|
|Command line|`--innodb-doublewrite-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_doublewrite_dir](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_doublewrite_dir)|

## innodb_doublewrite_files
|name|value|
|----|-----|
|Name|`innodb_doublewrite_files`|
|Command line|`--innodb-doublewrite-files=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`innodb_buffer_pool_instances * 2`|
|Dynamic|`false`|
|Range|from: `1` to: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_doublewrite_files](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_doublewrite_files)|

## innodb_doublewrite_pages
|name|value|
|----|-----|
|Name|`innodb_doublewrite_pages`|
|Command line|`--innodb-doublewrite-pages=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`innodb_write_io_threads value`|
|Dynamic|`false`|
|Range|to: `512`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_doublewrite_pages](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_doublewrite_pages)|

## innodb_extend_and_initialize
|name|value|
|----|-----|
|Name|`innodb_extend_and_initialize`|
|Command line|`--innodb=extend-and-initialize[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_extend_and_initialize](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_extend_and_initialize)|

## innodb_fast_shutdown
|name|value|
|----|-----|
|Name|`innodb_fast_shutdown`|
|Command line|`--innodb-fast-shutdown=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Valid value(s)|`0`, `1`, `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_fast_shutdown](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_fast_shutdown)|

## innodb_fil_make_page_dirty_debug
|name|value|
|----|-----|
|Name|`innodb_fil_make_page_dirty_debug`|
|Command line|`--innodb-fil-make-page-dirty-debug=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_fil_make_page_dirty_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_fil_make_page_dirty_debug)|

## innodb_file_per_table
|name|value|
|----|-----|
|Name|`innodb_file_per_table`|
|Command line|`--innodb-file-per-table[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_file_per_table](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_file_per_table)|

## innodb_fill_factor
|name|value|
|----|-----|
|Name|`innodb_fill_factor`|
|Command line|`--innodb-fill-factor=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`true`|
|Range|from: `10` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_fill_factor](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_fill_factor)|

## innodb_flush_log_at_timeout
|name|value|
|----|-----|
|Name|`innodb_flush_log_at_timeout`|
|Command line|`--innodb-flush-log-at-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `2700`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_flush_log_at_timeout](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_flush_log_at_timeout)|

## innodb_flush_log_at_trx_commit
|name|value|
|----|-----|
|Name|`innodb_flush_log_at_trx_commit`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_flush_log_at_trx_commit](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_flush_log_at_trx_commit)|
|dev.mysql.com|[sysvar_innodb_flush_log_at_trx_commit](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_innodb_flush_log_at_trx_commit)|
|dev.mysql.com|[sysvar_innodb_flush_log_at_trx_commit](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_innodb_flush_log_at_trx_commit)|

## innodb_flush_method
|name|value|
|----|-----|
|Name|`innodb_flush_method`|
|Command line|`--innodb-flush-method=value`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_flush_method](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_flush_method)|

## innodb_flush_neighbors
|name|value|
|----|-----|
|Name|`innodb_flush_neighbors`|
|Command line|`--innodb-flush-neighbors=#`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Valid value(s)|`0`, `1`, `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_flush_neighbors](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_flush_neighbors)|

## innodb_flush_sync
|name|value|
|----|-----|
|Name|`innodb_flush_sync`|
|Command line|`--innodb-flush-sync[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_flush_sync](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_flush_sync)|

## innodb_flushing_avg_loops
|name|value|
|----|-----|
|Name|`innodb_flushing_avg_loops`|
|Command line|`--innodb-flushing-avg-loops=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`30`|
|Dynamic|`true`|
|Range|from: `1` to: `1000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_flushing_avg_loops](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_flushing_avg_loops)|

## innodb_force_load_corrupted
|name|value|
|----|-----|
|Name|`innodb_force_load_corrupted`|
|Command line|`--innodb-force-load-corrupted[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_force_load_corrupted](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_force_load_corrupted)|

## innodb_force_recovery
|name|value|
|----|-----|
|Name|`innodb_force_recovery`|
|Command line|`--innodb-force-recovery=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`false`|
|Range|from: `0` to: `6`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_force_recovery](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_force_recovery)|

## innodb_fsync_threshold
|name|value|
|----|-----|
|Name|`innodb_fsync_threshold`|
|Command line|`--innodb-fsync-threshold=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_fsync_threshold](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_fsync_threshold)|

## innodb_ft_aux_table
|name|value|
|----|-----|
|Name|`innodb_ft_aux_table`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_aux_table](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_aux_table)|

## innodb_ft_cache_size
|name|value|
|----|-----|
|Name|`innodb_ft_cache_size`|
|Command line|`--innodb-ft-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8000000`|
|Dynamic|`false`|
|Range|from: `1600000` to: `80000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_cache_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_cache_size)|

## innodb_ft_enable_diag_print
|name|value|
|----|-----|
|Name|`innodb_ft_enable_diag_print`|
|Command line|`--innodb-ft-enable-diag-print[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_enable_diag_print](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_enable_diag_print)|

## innodb_ft_enable_stopword
|name|value|
|----|-----|
|Name|`innodb_ft_enable_stopword`|
|Command line|`--innodb-ft-enable-stopword[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_enable_stopword](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_enable_stopword)|

## innodb_ft_max_token_size
|name|value|
|----|-----|
|Name|`innodb_ft_max_token_size`|
|Command line|`--innodb-ft-max-token-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`84`|
|Dynamic|`false`|
|Range|from: `10` to: `84`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_max_token_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_max_token_size)|

## innodb_ft_min_token_size
|name|value|
|----|-----|
|Name|`innodb_ft_min_token_size`|
|Command line|`--innodb-ft-min-token-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`3`|
|Dynamic|`false`|
|Range|from: `0` to: `16`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_min_token_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_min_token_size)|

## innodb_ft_num_word_optimize
|name|value|
|----|-----|
|Name|`innodb_ft_num_word_optimize`|
|Command line|`--innodb-ft-num-word-optimize=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2000`|
|Dynamic|`true`|
|Range|from: `1000` to: `10000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_num_word_optimize](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_num_word_optimize)|

## innodb_ft_result_cache_limit
|name|value|
|----|-----|
|Name|`innodb_ft_result_cache_limit`|
|Command line|`--innodb-ft-result-cache-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2000000000`|
|Dynamic|`true`|
|Range|from: `1000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_result_cache_limit](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_result_cache_limit)|

## innodb_ft_server_stopword_table
|name|value|
|----|-----|
|Name|`innodb_ft_server_stopword_table`|
|Command line|`--innodb-ft-server-stopword-table=db_name/table_name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_server_stopword_table](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_server_stopword_table)|

## innodb_ft_sort_pll_degree
|name|value|
|----|-----|
|Name|`innodb_ft_sort_pll_degree`|
|Command line|`--innodb-ft-sort-pll-degree=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`false`|
|Range|from: `1` to: `16`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_sort_pll_degree](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_sort_pll_degree)|

## innodb_ft_total_cache_size
|name|value|
|----|-----|
|Name|`innodb_ft_total_cache_size`|
|Command line|`--innodb-ft-total-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`640000000`|
|Dynamic|`false`|
|Range|from: `32000000` to: `1600000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_total_cache_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_total_cache_size)|

## innodb_ft_user_stopword_table
|name|value|
|----|-----|
|Name|`innodb_ft_user_stopword_table`|
|Command line|`--innodb-ft-user-stopword-table=db_name/table_name`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_ft_user_stopword_table](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_ft_user_stopword_table)|

## innodb_idle_flush_pct
|name|value|
|----|-----|
|Name|`innodb_idle_flush_pct`|
|Command line|`--innodb-idle-flush-pct=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`true`|
|Range|from: `0` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_idle_flush_pct](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_idle_flush_pct)|

## innodb_io_capacity
|name|value|
|----|-----|
|Name|`innodb_io_capacity`|
|Command line|`--innodb-io-capacity=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`200`|
|Dynamic|`true`|
|Range|from: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_io_capacity](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_io_capacity)|

## innodb_io_capacity_max
|name|value|
|----|-----|
|Name|`innodb_io_capacity_max`|
|Command line|`--innodb-io-capacity-max=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2 * innodb_io_capacity, min of 2000`|
|Dynamic|`true`|
|Range|from: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_io_capacity_max](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_io_capacity_max)|

## innodb_limit_optimistic_insert_debug
|name|value|
|----|-----|
|Name|`innodb_limit_optimistic_insert_debug`|
|Command line|`--innodb-limit-optimistic-insert-debug=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_limit_optimistic_insert_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_limit_optimistic_insert_debug)|

## innodb_lock_wait_timeout
|name|value|
|----|-----|
|Name|`innodb_lock_wait_timeout`|
|Command line|`--innodb-lock-wait-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`50`|
|Dynamic|`true`|
|Range|from: `1` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_lock_wait_timeout](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_lock_wait_timeout)|

## innodb_log_buffer_size
|name|value|
|----|-----|
|Name|`innodb_log_buffer_size`|
|Command line|`--innodb-log-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`16777216`|
|Dynamic|`true`|
|Range|from: `1048576` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_buffer_size)|

## innodb_log_checkpoint_fuzzy_now
|name|value|
|----|-----|
|Name|`innodb_log_checkpoint_fuzzy_now`|
|Command line|`--innodb-log-checkpoint-fuzzy-now[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_checkpoint_fuzzy_now](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_checkpoint_fuzzy_now)|

## innodb_log_checkpoint_now
|name|value|
|----|-----|
|Name|`innodb_log_checkpoint_now`|
|Command line|`--innodb-log-checkpoint-now[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_checkpoint_now](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_checkpoint_now)|

## innodb_log_checksums
|name|value|
|----|-----|
|Name|`innodb_log_checksums`|
|Command line|`--innodb-log-checksums[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_checksums](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_checksums)|

## innodb_log_compressed_pages
|name|value|
|----|-----|
|Name|`innodb_log_compressed_pages`|
|Command line|`--innodb-log-compressed-pages[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_compressed_pages](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_compressed_pages)|

## innodb_log_file_size
|name|value|
|----|-----|
|Name|`innodb_log_file_size`|
|Command line|`--innodb-log-file-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`50331648`|
|Dynamic|`false`|
|Range|from: `4194304`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_file_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_file_size)|

## innodb_log_files_in_group
|name|value|
|----|-----|
|Name|`innodb_log_files_in_group`|
|Command line|`--innodb-log-files-in-group=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`false`|
|Range|from: `2` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_files_in_group](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_files_in_group)|

## innodb_log_group_home_dir
|name|value|
|----|-----|
|Name|`innodb_log_group_home_dir`|
|Command line|`--innodb-log-group-home-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_group_home_dir](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_group_home_dir)|

## innodb_log_spin_cpu_abs_lwm
|name|value|
|----|-----|
|Name|`innodb_log_spin_cpu_abs_lwm`|
|Command line|`--innodb-log-spin-cpu-abs-lwm=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`80`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_spin_cpu_abs_lwm](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_spin_cpu_abs_lwm)|

## innodb_log_spin_cpu_pct_hwm
|name|value|
|----|-----|
|Name|`innodb_log_spin_cpu_pct_hwm`|
|Command line|`--innodb-log-spin-cpu-pct-hwm=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`50`|
|Dynamic|`true`|
|Range|from: `0` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_spin_cpu_pct_hwm](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_spin_cpu_pct_hwm)|

## innodb_log_wait_for_flush_spin_hwm
|name|value|
|----|-----|
|Name|`innodb_log_wait_for_flush_spin_hwm`|
|Command line|`--innodb-log-wait-for-flush-spin-hwm=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`400`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_wait_for_flush_spin_hwm](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_wait_for_flush_spin_hwm)|

## innodb_log_write_ahead_size
|name|value|
|----|-----|
|Name|`innodb_log_write_ahead_size`|
|Command line|`--innodb-log-write-ahead-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8192`|
|Dynamic|`true`|
|Range|from: `512`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_write_ahead_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_write_ahead_size)|

## innodb_log_writer_threads
|name|value|
|----|-----|
|Name|`innodb_log_writer_threads`|
|Command line|`--innodb-log-writer-threads[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_log_writer_threads](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_log_writer_threads)|

## innodb_lru_scan_depth
|name|value|
|----|-----|
|Name|`innodb_lru_scan_depth`|
|Command line|`--innodb-lru-scan-depth=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1024`|
|Dynamic|`true`|
|Range|from: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_lru_scan_depth](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_lru_scan_depth)|

## innodb_max_dirty_pages_pct
|name|value|
|----|-----|
|Name|`innodb_max_dirty_pages_pct`|
|Command line|`--innodb-max-dirty-pages-pct=#`|
|Type of variable|`numeric`|
|Scope|`global`|
|Default value|`90`|
|Dynamic|`true`|
|Range|from: `0` to: `99.999`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_max_dirty_pages_pct](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_max_dirty_pages_pct)|

## innodb_max_dirty_pages_pct_lwm
|name|value|
|----|-----|
|Name|`innodb_max_dirty_pages_pct_lwm`|
|Command line|`--innodb-max-dirty-pages-pct-lwm=#`|
|Type of variable|`numeric`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `99.999`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_max_dirty_pages_pct_lwm](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_max_dirty_pages_pct_lwm)|

## innodb_max_purge_lag
|name|value|
|----|-----|
|Name|`innodb_max_purge_lag`|
|Command line|`--innodb-max-purge-lag=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_max_purge_lag](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_max_purge_lag)|

## innodb_max_purge_lag_delay
|name|value|
|----|-----|
|Name|`innodb_max_purge_lag_delay`|
|Command line|`--innodb-max-purge-lag-delay=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `10000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_max_purge_lag_delay](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_max_purge_lag_delay)|

## innodb_max_undo_log_size
|name|value|
|----|-----|
|Name|`innodb_max_undo_log_size`|
|Command line|`--innodb-max-undo-log-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1073741824`|
|Dynamic|`true`|
|Range|from: `10485760`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_max_undo_log_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_max_undo_log_size)|

## innodb_merge_threshold_set_all_debug
|name|value|
|----|-----|
|Name|`innodb_merge_threshold_set_all_debug`|
|Command line|`--innodb-merge-threshold-set-all-debug=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`50`|
|Dynamic|`true`|
|Range|from: `1` to: `50`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_merge_threshold_set_all_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_merge_threshold_set_all_debug)|

## innodb_monitor_disable
|name|value|
|----|-----|
|Name|`innodb_monitor_disable`|
|Command line|`--innodb-monitor-disable={counter|module|pattern|all}`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_monitor_disable](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_monitor_disable)|

## innodb_monitor_enable
|name|value|
|----|-----|
|Name|`innodb_monitor_enable`|
|Command line|`--innodb-monitor-enable={counter|module|pattern|all}`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_monitor_enable](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_monitor_enable)|

## innodb_monitor_reset
|name|value|
|----|-----|
|Name|`innodb_monitor_reset`|
|Command line|`--innodb-monitor-reset={counter|module|pattern|all}`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|
|Valid value(s)|`counter`, `module`, `pattern`, `all`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_monitor_reset](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_monitor_reset)|

## innodb_monitor_reset_all
|name|value|
|----|-----|
|Name|`innodb_monitor_reset_all`|
|Command line|`--innodb-monitor-reset-all={counter|module|pattern|all}`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|
|Valid value(s)|`counter`, `module`, `pattern`, `all`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_monitor_reset_all](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_monitor_reset_all)|

## innodb_numa_interleave
|name|value|
|----|-----|
|Name|`innodb_numa_interleave`|
|Command line|`--innodb-numa-interleave[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_numa_interleave](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_numa_interleave)|

## innodb_old_blocks_pct
|name|value|
|----|-----|
|Name|`innodb_old_blocks_pct`|
|Command line|`--innodb-old-blocks-pct=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`37`|
|Dynamic|`true`|
|Range|from: `5` to: `95`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_old_blocks_pct](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_old_blocks_pct)|

## innodb_old_blocks_time
|name|value|
|----|-----|
|Name|`innodb_old_blocks_time`|
|Command line|`--innodb-old-blocks-time=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1000`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_old_blocks_time](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_old_blocks_time)|

## innodb_online_alter_log_max_size
|name|value|
|----|-----|
|Name|`innodb_online_alter_log_max_size`|
|Command line|`--innodb-online-alter-log-max-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`134217728`|
|Dynamic|`true`|
|Range|from: `65536`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_online_alter_log_max_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_online_alter_log_max_size)|

## innodb_open_files
|name|value|
|----|-----|
|Name|`innodb_open_files`|
|Command line|`--innodb-open-files=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`true`|
|Range|from: `10` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_open_files](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_open_files)|

## innodb_optimize_fulltext_only
|name|value|
|----|-----|
|Name|`innodb_optimize_fulltext_only`|
|Command line|`--innodb-optimize-fulltext-only[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_optimize_fulltext_only](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_optimize_fulltext_only)|

## innodb_page_cleaners
|name|value|
|----|-----|
|Name|`innodb_page_cleaners`|
|Command line|`--innodb-page-cleaners=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4`|
|Dynamic|`false`|
|Range|from: `1` to: `64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_page_cleaners](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_page_cleaners)|

## innodb_page_size
|name|value|
|----|-----|
|Name|`innodb_page_size`|
|Command line|`--innodb-page-size=#`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`16384`|
|Dynamic|`false`|
|Valid value(s)|`4096`, `8192`, `16384`, `32768`, `65536`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_page_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_page_size)|

## innodb_parallel_read_threads
|name|value|
|----|-----|
|Name|`innodb_parallel_read_threads`|
|Command line|`--innodb-parallel-read-threads=#`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`4`|
|Dynamic|`true`|
|Range|from: `1` to: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_parallel_read_threads](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_parallel_read_threads)|

## innodb_print_all_deadlocks
|name|value|
|----|-----|
|Name|`innodb_print_all_deadlocks`|
|Command line|`--innodb-print-all-deadlocks[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_print_all_deadlocks](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_print_all_deadlocks)|

## innodb_print_ddl_logs
|name|value|
|----|-----|
|Name|`innodb_print_ddl_logs`|
|Command line|`--innodb-print-ddl-logs[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_print_ddl_logs](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_print_ddl_logs)|

## innodb_purge_batch_size
|name|value|
|----|-----|
|Name|`innodb_purge_batch_size`|
|Command line|`--innodb-purge-batch-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`300`|
|Dynamic|`true`|
|Range|from: `1` to: `5000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_purge_batch_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_purge_batch_size)|

## innodb_purge_threads
|name|value|
|----|-----|
|Name|`innodb_purge_threads`|
|Command line|`--innodb-purge-threads=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4`|
|Dynamic|`false`|
|Range|from: `1` to: `32`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_purge_threads](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_purge_threads)|

## innodb_purge_rseg_truncate_frequency
|name|value|
|----|-----|
|Name|`innodb_purge_rseg_truncate_frequency`|
|Command line|`--innodb-purge-rseg-truncate-frequency=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`128`|
|Dynamic|`true`|
|Range|from: `1` to: `128`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_purge_rseg_truncate_frequency](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_purge_rseg_truncate_frequency)|

## innodb_random_read_ahead
|name|value|
|----|-----|
|Name|`innodb_random_read_ahead`|
|Command line|`--innodb-random-read-ahead[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_random_read_ahead](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_random_read_ahead)|

## innodb_read_ahead_threshold
|name|value|
|----|-----|
|Name|`innodb_read_ahead_threshold`|
|Command line|`--innodb-read-ahead-threshold=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`56`|
|Dynamic|`true`|
|Range|from: `0` to: `64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_read_ahead_threshold](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_read_ahead_threshold)|

## innodb_read_io_threads
|name|value|
|----|-----|
|Name|`innodb_read_io_threads`|
|Command line|`--innodb-read-io-threads=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4`|
|Dynamic|`false`|
|Range|from: `1` to: `64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_read_io_threads](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_read_io_threads)|

## innodb_read_only
|name|value|
|----|-----|
|Name|`innodb_read_only`|
|Command line|`--innodb-read-only[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_read_only](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_read_only)|

## innodb_redo_log_archive_dirs
|name|value|
|----|-----|
|Name|`innodb_redo_log_archive_dirs`|
|Command line|`--innodb-redo-log-archive-dirs`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_redo_log_archive_dirs](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_redo_log_archive_dirs)|

## innodb_redo_log_capacity
|name|value|
|----|-----|
|Name|`innodb_redo_log_capacity`|
|Command line|`--innodb-redo-log-capacity=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`104857600`|
|Dynamic|`true`|
|Range|from: `8388608` to: `549755813888`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_redo_log_capacity](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_redo_log_capacity)|

## innodb_redo_log_encrypt
|name|value|
|----|-----|
|Name|`innodb_redo_log_encrypt`|
|Command line|`--innodb-redo-log-encrypt[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_redo_log_encrypt](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_redo_log_encrypt)|

## innodb_replication_delay
|name|value|
|----|-----|
|Name|`innodb_replication_delay`|
|Command line|`--innodb-replication-delay=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_replication_delay](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_replication_delay)|

## innodb_rollback_on_timeout
|name|value|
|----|-----|
|Name|`innodb_rollback_on_timeout`|
|Command line|`--innodb-rollback-on-timeout[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_rollback_on_timeout](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_rollback_on_timeout)|

## innodb_rollback_segments
|name|value|
|----|-----|
|Name|`innodb_rollback_segments`|
|Command line|`--innodb-rollback-segments=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`128`|
|Dynamic|`true`|
|Range|from: `1` to: `128`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_rollback_segments](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_rollback_segments)|

## innodb_saved_page_number_debug
|name|value|
|----|-----|
|Name|`innodb_saved_page_number_debug`|
|Command line|`--innodb-saved-page-number-debug=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_saved_page_number_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_saved_page_number_debug)|

## innodb_segment_reserve_factor
|name|value|
|----|-----|
|Name|`innodb_segment_reserve_factor`|
|Command line|`--innodb-segment-reserve-factor=#`|
|Type of variable|`numeric`|
|Scope|`global`|
|Default value|`12.5`|
|Dynamic|`true`|
|Range|from: `0.03` to: `40`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_segment_reserve_factor](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_segment_reserve_factor)|

## innodb_sort_buffer_size
|name|value|
|----|-----|
|Name|`innodb_sort_buffer_size`|
|Command line|`--innodb-sort-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1048576`|
|Dynamic|`false`|
|Range|from: `65536` to: `67108864`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_sort_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_sort_buffer_size)|

## innodb_spin_wait_delay
|name|value|
|----|-----|
|Name|`innodb_spin_wait_delay`|
|Command line|`--innodb-spin-wait-delay=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`6`|
|Dynamic|`true`|
|Range|from: `0` to: `1000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_spin_wait_delay](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_spin_wait_delay)|

## innodb_spin_wait_pause_multiplier
|name|value|
|----|-----|
|Name|`innodb_spin_wait_pause_multiplier`|
|Command line|`--innodb-spin-wait-pause-multiplier=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`50`|
|Dynamic|`true`|
|Range|from: `0` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_spin_wait_pause_multiplier](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_spin_wait_pause_multiplier)|

## innodb_stats_auto_recalc
|name|value|
|----|-----|
|Name|`innodb_stats_auto_recalc`|
|Command line|`--innodb-stats-auto-recalc[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_stats_auto_recalc](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_stats_auto_recalc)|

## innodb_stats_include_delete_marked
|name|value|
|----|-----|
|Name|`innodb_stats_include_delete_marked`|
|Command line|`--innodb-stats-include-delete-marked[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_stats_include_delete_marked](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_stats_include_delete_marked)|

## innodb_stats_method
|name|value|
|----|-----|
|Name|`innodb_stats_method`|
|Command line|`--innodb-stats-method=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`nulls_equal`|
|Dynamic|`true`|
|Valid value(s)|`nulls_equal`, `nulls_unequal`, `nulls_ignored`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_stats_method](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_stats_method)|

## innodb_stats_on_metadata
|name|value|
|----|-----|
|Name|`innodb_stats_on_metadata`|
|Command line|`--innodb-stats-on-metadata[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_stats_on_metadata](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_stats_on_metadata)|

## innodb_stats_persistent
|name|value|
|----|-----|
|Name|`innodb_stats_persistent`|
|Command line|`--innodb-stats-persistent[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_stats_persistent](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_stats_persistent)|

## innodb_stats_persistent_sample_pages
|name|value|
|----|-----|
|Name|`innodb_stats_persistent_sample_pages`|
|Command line|`--innodb-stats-persistent-sample-pages=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`20`|
|Dynamic|`true`|
|Range|from: `1` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_stats_persistent_sample_pages](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_stats_persistent_sample_pages)|

## innodb_stats_transient_sample_pages
|name|value|
|----|-----|
|Name|`innodb_stats_transient_sample_pages`|
|Command line|`--innodb-stats-transient-sample-pages=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8`|
|Dynamic|`true`|
|Range|from: `1` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_stats_transient_sample_pages](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_stats_transient_sample_pages)|

## innodb_status_output
|name|value|
|----|-----|
|Name|`innodb_status_output`|
|Command line|`--innodb-status-output[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_status_output](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_status_output)|

## innodb_status_output_locks
|name|value|
|----|-----|
|Name|`innodb_status_output_locks`|
|Command line|`--innodb-status-output-locks[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_status_output_locks](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_status_output_locks)|

## innodb_strict_mode
|name|value|
|----|-----|
|Name|`innodb_strict_mode`|
|Command line|`--innodb-strict-mode[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_strict_mode](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_strict_mode)|

## innodb_sync_array_size
|name|value|
|----|-----|
|Name|`innodb_sync_array_size`|
|Command line|`--innodb-sync-array-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`false`|
|Range|from: `1` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_sync_array_size](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_sync_array_size)|

## innodb_sync_spin_loops
|name|value|
|----|-----|
|Name|`innodb_sync_spin_loops`|
|Command line|`--innodb-sync-spin-loops=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`30`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_sync_spin_loops](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_sync_spin_loops)|

## innodb_sync_debug
|name|value|
|----|-----|
|Name|`innodb_sync_debug`|
|Command line|`--innodb-sync-debug[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_sync_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_sync_debug)|

## innodb_table_locks
|name|value|
|----|-----|
|Name|`innodb_table_locks`|
|Command line|`--innodb-table-locks[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_table_locks](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_table_locks)|

## innodb_temp_data_file_path
|name|value|
|----|-----|
|Name|`innodb_temp_data_file_path`|
|Command line|`--innodb-temp-data-file-path=file_name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`ibtmp1:12M:autoextend`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_temp_data_file_path](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_temp_data_file_path)|

## innodb_temp_tablespaces_dir
|name|value|
|----|-----|
|Name|`innodb_temp_tablespaces_dir`|
|Command line|`--innodb-temp-tablespaces-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`#innodb_temp`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_temp_tablespaces_dir](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_temp_tablespaces_dir)|

## innodb_thread_concurrency
|name|value|
|----|-----|
|Name|`innodb_thread_concurrency`|
|Command line|`--innodb-thread-concurrency=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_thread_concurrency](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_thread_concurrency)|

## innodb_thread_sleep_delay
|name|value|
|----|-----|
|Name|`innodb_thread_sleep_delay`|
|Command line|`--innodb-thread-sleep-delay=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10000`|
|Dynamic|`true`|
|Range|from: `0` to: `1000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_thread_sleep_delay](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_thread_sleep_delay)|

## innodb_tmpdir
|name|value|
|----|-----|
|Name|`innodb_tmpdir`|
|Command line|`--innodb-tmpdir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`, `session`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_tmpdir](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_tmpdir)|

## innodb_trx_purge_view_update_only_debug
|name|value|
|----|-----|
|Name|`innodb_trx_purge_view_update_only_debug`|
|Command line|`--innodb-trx-purge-view-update-only-debug[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_trx_purge_view_update_only_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_trx_purge_view_update_only_debug)|

## innodb_trx_rseg_n_slots_debug
|name|value|
|----|-----|
|Name|`innodb_trx_rseg_n_slots_debug`|
|Command line|`--innodb-trx-rseg-n-slots-debug=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_trx_rseg_n_slots_debug](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_trx_rseg_n_slots_debug)|

## innodb_undo_directory
|name|value|
|----|-----|
|Name|`innodb_undo_directory`|
|Command line|`--innodb-undo-directory=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_undo_directory](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_undo_directory)|

## innodb_undo_log_encrypt
|name|value|
|----|-----|
|Name|`innodb_undo_log_encrypt`|
|Command line|`--innodb-undo-log-encrypt[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_undo_log_encrypt](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_undo_log_encrypt)|

## innodb_undo_log_truncate
|name|value|
|----|-----|
|Name|`innodb_undo_log_truncate`|
|Command line|`--innodb-undo-log-truncate[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_undo_log_truncate](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_undo_log_truncate)|

## innodb_undo_tablespaces
|name|value|
|----|-----|
|Name|`innodb_undo_tablespaces`|
|Command line|`--innodb-undo-tablespaces=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`true`|
|Range|from: `2` to: `127`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_undo_tablespaces](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_undo_tablespaces)|

## innodb_use_fdatasync
|name|value|
|----|-----|
|Name|`innodb_use_fdatasync`|
|Command line|`--innodb-use-fdatasync[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_use_fdatasync](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_use_fdatasync)|

## innodb_use_native_aio
|name|value|
|----|-----|
|Name|`innodb_use_native_aio`|
|Command line|`--innodb-use-native-aio[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_use_native_aio](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_use_native_aio)|

## innodb_validate_tablespace_paths
|name|value|
|----|-----|
|Name|`innodb_validate_tablespace_paths`|
|Command line|`--innodb-validate-tablespace-paths[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_validate_tablespace_paths](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_validate_tablespace_paths)|

## innodb_write_io_threads
|name|value|
|----|-----|
|Name|`innodb_write_io_threads`|
|Command line|`--innodb-write-io-threads=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4`|
|Dynamic|`false`|
|Range|from: `1` to: `64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_write_io_threads](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_write_io_threads)|

## innodb_version
|name|value|
|----|-----|
|Name|`innodb_version`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_innodb_version](https://dev.mysql.com/doc/refman/8.0/en/innodb-parameters.html#sysvar_innodb_version)|

## keyring_aws_cmk_id
|name|value|
|----|-----|
|Name|`keyring_aws_cmk_id`|
|Command line|`--keyring-aws-cmk-id=value`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_aws_cmk_id](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_aws_cmk_id)|

## keyring_aws_conf_file
|name|value|
|----|-----|
|Name|`keyring_aws_conf_file`|
|Command line|`--keyring-aws-conf-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`platform specific`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_aws_conf_file](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_aws_conf_file)|

## keyring_aws_data_file
|name|value|
|----|-----|
|Name|`keyring_aws_data_file`|
|Command line|`--keyring-aws-data-file`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`platform specific`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_aws_data_file](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_aws_data_file)|

## keyring_aws_region
|name|value|
|----|-----|
|Name|`keyring_aws_region`|
|Command line|`--keyring-aws-region=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`us-east-1`|
|Dynamic|`true`|
|Valid value(s)|`af-south-1`, `ap-east-1`, `ap-northeast-1`, `ap-northeast-2`, `ap-northeast-3`, `ap-south-1`, `ap-southeast-1`, `ap-southeast-2`, `ca-central-1`, `cn-north-1`, `cn-northwest-1`, `eu-central-1`, `eu-north-1`, `eu-south-1`, `eu-west-1`, `eu-west-2`, `eu-west-3`, `me-south-1`, `sa-east-1`, `us-east-1`, `us-east-2`, `us-gov-east-1`, `us-iso-east-1`, `us-iso-west-1`, `us-isob-east-1`, `us-west-1`, `us-west-2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_aws_region](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_aws_region)|

## keyring_hashicorp_auth_path
|name|value|
|----|-----|
|Name|`keyring_hashicorp_auth_path`|
|Command line|`--keyring-hashicorp-auth-path=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`/v1/auth/approle/login`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_auth_path](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_auth_path)|

## keyring_hashicorp_ca_path
|name|value|
|----|-----|
|Name|`keyring_hashicorp_ca_path`|
|Command line|`--keyring-hashicorp-ca-path=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_ca_path](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_ca_path)|

## keyring_hashicorp_caching
|name|value|
|----|-----|
|Name|`keyring_hashicorp_caching`|
|Command line|`--keyring-hashicorp-caching[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_caching](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_caching)|

## keyring_hashicorp_commit_auth_path
|name|value|
|----|-----|
|Name|`keyring_hashicorp_commit_auth_path`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_commit_auth_path](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_commit_auth_path)|

## keyring_hashicorp_commit_ca_path
|name|value|
|----|-----|
|Name|`keyring_hashicorp_commit_ca_path`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_commit_ca_path](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_commit_ca_path)|

## keyring_hashicorp_commit_caching
|name|value|
|----|-----|
|Name|`keyring_hashicorp_commit_caching`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_commit_caching](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_commit_caching)|

## keyring_hashicorp_commit_role_id
|name|value|
|----|-----|
|Name|`keyring_hashicorp_commit_role_id`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_commit_role_id](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_commit_role_id)|

## keyring_hashicorp_commit_server_url
|name|value|
|----|-----|
|Name|`keyring_hashicorp_commit_server_url`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_commit_server_url](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_commit_server_url)|

## keyring_hashicorp_commit_store_path
|name|value|
|----|-----|
|Name|`keyring_hashicorp_commit_store_path`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_commit_store_path](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_commit_store_path)|

## keyring_hashicorp_role_id
|name|value|
|----|-----|
|Name|`keyring_hashicorp_role_id`|
|Command line|`--keyring-hashicorp-role-id=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_role_id](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_role_id)|

## keyring_hashicorp_secret_id
|name|value|
|----|-----|
|Name|`keyring_hashicorp_secret_id`|
|Command line|`--keyring-hashicorp-secret-id=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_secret_id](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_secret_id)|

## keyring_hashicorp_server_url
|name|value|
|----|-----|
|Name|`keyring_hashicorp_server_url`|
|Command line|`--keyring-hashicorp-server-url=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`https://127.0.0.1:8200`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_server_url](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_server_url)|

## keyring_hashicorp_store_path
|name|value|
|----|-----|
|Name|`keyring_hashicorp_store_path`|
|Command line|`--keyring-hashicorp-store-path=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_hashicorp_store_path](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_hashicorp_store_path)|

## keyring_okv_conf_dir
|name|value|
|----|-----|
|Name|`keyring_okv_conf_dir`|
|Command line|`--keyring-okv-conf-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_okv_conf_dir](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_okv_conf_dir)|

## keyring_operations
|name|value|
|----|-----|
|Name|`keyring_operations`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keyring_operations](https://dev.mysql.com/doc/refman/8.4/en/keyring-system-variables.html#sysvar_keyring_operations)|

## ndbcluster
|name|value|
|----|-----|
|Name|`ndbcluster`|
|Command line|`--ndbcluster[=value]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndbcluster](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndbcluster)|

## ndb_allow_copying_alter_table
|name|value|
|----|-----|
|Name|`ndb_allow_copying_alter_table`|
|Command line|`--ndb-allow-copying-alter-table[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-allow-copying-alter-table](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-allow-copying-alter-table)|

## ndb_batch_size
|name|value|
|----|-----|
|Name|`ndb_batch_size`|
|Command line|`--ndb-batch-size`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`32768`|
|Dynamic|`true`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-batch-size](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-batch-size)|

## ndb_cluster_connection_pool
|name|value|
|----|-----|
|Name|`ndb_cluster_connection_pool`|
|Command line|`--ndb-cluster-connection-pool`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`false`|
|Range|from: `1` to: `63`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-cluster-connection-pool](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-cluster-connection-pool)|

## ndb_cluster_connection_pool_nodeids
|name|value|
|----|-----|
|Name|`ndb_cluster_connection_pool_nodeids`|
|Command line|`--ndb-cluster-connection-pool-nodeids`|
|Type of variable|`set`|
|Scope|`global`|
|Default value|``|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-cluster-connection-pool-nodeids](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-cluster-connection-pool-nodeids)|

## ndb_blob_read_batch_bytes
|name|value|
|----|-----|
|Name|`ndb_blob_read_batch_bytes`|
|Command line|`--ndb-blob-read-batch-bytes`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`65536`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-blob-read-batch-bytes](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-blob-read-batch-bytes)|

## ndb_blob_write_batch_bytes
|name|value|
|----|-----|
|Name|`ndb_blob_write_batch_bytes`|
|Command line|`--ndb-blob-write-batch-bytes`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`65536`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-blob-write-batch-bytes](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-blob-write-batch-bytes)|

## ndb_connectstring
|name|value|
|----|-----|
|Name|`ndb_connectstring`|
|Command line|`--ndb-connectstring`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-connectstring](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-connectstring)|

## ndb_default_column_format
|name|value|
|----|-----|
|Name|`ndb_default_column_format`|
|Command line|`--ndb-default-column-format={FIXED|DYNAMIC}`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`FIXED`|
|Dynamic|`true`|
|Valid value(s)|`FIXED`, `DYNAMIC`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-default-column-format](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-default-column-format)|

## ndb_deferred_constraints
|name|value|
|----|-----|
|Name|`ndb_deferred_constraints`|
|Command line|`--ndb-deferred-constraints`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-deferred-constraints](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-deferred-constraints)|

## ndb_distribution
|name|value|
|----|-----|
|Name|`ndb_distribution`|
|Command line|`--ndb-distribution={KEYHASH|LINHASH}`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`KEYHASH`|
|Dynamic|`true`|
|Valid value(s)|`LINHASH`, `KEYHASH`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-distribution](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-distribution)|

## ndb_log_apply_status
|name|value|
|----|-----|
|Name|`ndb_log_apply_status`|
|Command line|`--ndb-log-apply-status[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-apply-status](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-apply-status)|

## ndb_log_empty_epochs
|name|value|
|----|-----|
|Name|`ndb_log_empty_epochs`|
|Command line|`--ndb-log-empty-epochs[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-empty-epochs](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-empty-epochs)|

## ndb_log_empty_update
|name|value|
|----|-----|
|Name|`ndb_log_empty_update`|
|Command line|`--ndb-log-empty-update[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-empty-update](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-empty-update)|

## ndb_log_exclusive_reads
|name|value|
|----|-----|
|Name|`ndb_log_exclusive_reads`|
|Command line|`--ndb-log-exclusive-reads[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-exclusive-reads](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-exclusive-reads)|

## ndb_log_fail_terminate
|name|value|
|----|-----|
|Name|`ndb_log_fail_terminate`|
|Command line|`--ndb-log-fail-terminate`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`FALSE`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-fail-terminate](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-fail-terminate)|

## ndb_log_orig
|name|value|
|----|-----|
|Name|`ndb_log_orig`|
|Command line|`--ndb-log-orig[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-orig](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-orig)|

## ndb_log_transaction_id
|name|value|
|----|-----|
|Name|`ndb_log_transaction_id`|
|Command line|`--ndb-log-transaction-id[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-transaction-id](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-transaction-id)|

## ndb_log_update_as_write
|name|value|
|----|-----|
|Name|`ndb_log_update_as_write`|
|Command line|`--ndb-log-update-as-write[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-update-as-write](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-update-as-write)|

## ndb_log_updated_only
|name|value|
|----|-----|
|Name|`ndb_log_updated_only`|
|Command line|`--ndb-log-updated-only[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-updated-only](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-updated-only)|

## ndb_log_update_minimal
|name|value|
|----|-----|
|Name|`ndb_log_update_minimal`|
|Command line|`--ndb-log-update-minimal[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-log-update-minimal](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-update-minimal)|

## ndb_mgmd_host
|name|value|
|----|-----|
|Name|`ndb_mgmd_host`|
|Command line|`--ndb-mgmd-host=host_name[:port_num]`|
|Type of variable|`string`|
|Default value|`localhost:1186`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-mgmd-host](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-mgmd-host)|

## ndb_nodeid
|name|value|
|----|-----|
|Name|`ndb_nodeid`|
|Command line|`--ndb-nodeid=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`N/A`|
|Dynamic|`false`|
|Range|from: `1` to: `63`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-nodeid](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-nodeid)|

## ndb_optimization_delay
|name|value|
|----|-----|
|Name|`ndb_optimization_delay`|
|Command line|`--ndb-optimization-delay=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `100000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-optimization-delay](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-optimization-delay)|

## ndb_optimized_node_selection
|name|value|
|----|-----|
|Name|`ndb_optimized_node_selection`|
|Command line|`--ndb-optimized-node-selection`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`3`|
|Dynamic|`true`|
|Range|from: `0` to: `3`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-optimized-node-selection](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-optimized-node-selection)|

## ndb_transid_mysql_connection_map
|name|value|
|----|-----|
|Name|`ndb_transid_mysql_connection_map`|
|Command line|`--ndb-transid-mysql-connection-map[=state]`|
|Type of variable|`enumeration`|
|Default value|`ON`|
|Valid value(s)|`ON`, `OFF`, `FORCE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-transid-mysql-connection-map](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-transid-mysql-connection-map)|

## ndb_wait_connected
|name|value|
|----|-----|
|Name|`ndb_wait_connected`|
|Command line|`--ndb-wait-connected=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`30`|
|Dynamic|`false`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-wait-connected](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-wait-connected)|

## ndb_wait_setup
|name|value|
|----|-----|
|Name|`ndb_wait_setup`|
|Command line|`--ndb-wait-setup=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`15`|
|Dynamic|`false`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ndb-wait-setup](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-wait-setup)|

## skip_ndbcluster
|name|value|
|----|-----|
|Name|`skip_ndbcluster`|
|Command line|`--skip-ndbcluster`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-ndbcluster](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_skip-ndbcluster)|

## ndb_autoincrement_prefetch_sz
|name|value|
|----|-----|
|Name|`ndb_autoincrement_prefetch_sz`|
|Command line|`--ndb-autoincrement-prefetch-sz=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `65536`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_autoincrement_prefetch_sz](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_autoincrement_prefetch_sz)|

## ndb_cache_check_time
|name|value|
|----|-----|
|Name|`ndb_cache_check_time`|
|Command line|`--ndb-cache-check-time=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_cache_check_time](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_cache_check_time)|

## ndb_clear_apply_status
|name|value|
|----|-----|
|Name|`ndb_clear_apply_status`|
|Command line|`--ndb-clear-apply-status[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_clear_apply_status](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_clear_apply_status)|

## ndb_data_node_neighbour
|name|value|
|----|-----|
|Name|`ndb_data_node_neighbour`|
|Command line|`--ndb-data-node-neighbour=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `255`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_data_node_neighbour](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_data_node_neighbour)|

## ndb_eventbuffer_free_percent
|name|value|
|----|-----|
|Name|`ndb_eventbuffer_free_percent`|
|Command line|`--ndb-eventbuffer-free-percent=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`20`|
|Dynamic|`true`|
|Range|from: `1` to: `99`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_eventbuffer_free_percent](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_eventbuffer_free_percent)|

## ndb_eventbuffer_max_alloc
|name|value|
|----|-----|
|Name|`ndb_eventbuffer_max_alloc`|
|Command line|`--ndb-eventbuffer-max-alloc=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_eventbuffer_max_alloc](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_eventbuffer_max_alloc)|

## ndb_extra_logging
|name|value|
|----|-----|
|Name|`ndb_extra_logging`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `0` to: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_extra_logging](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_extra_logging)|

## ndb_force_send
|name|value|
|----|-----|
|Name|`ndb_force_send`|
|Command line|`--ndb-force-send[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_force_send](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_force_send)|

## ndb_fully_replicated
|name|value|
|----|-----|
|Name|`ndb_fully_replicated`|
|Command line|`--ndb-fully-replicated[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_fully_replicated](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_fully_replicated)|

## ndb_index_stat_enable
|name|value|
|----|-----|
|Name|`ndb_index_stat_enable`|
|Command line|`--ndb-index-stat-enable[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_index_stat_enable](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_index_stat_enable)|

## ndb_index_stat_option
|name|value|
|----|-----|
|Name|`ndb_index_stat_option`|
|Command line|`--ndb-index-stat-option=value`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`loop_checkon=1000ms,loop_idle=1000ms,loop_busy=100ms,         update_batch=1,read_batch=4,idle_batch=32,check_batch=32,         check_delay=1m,delete_batch=8,clean_delay=0,error_batch=4,         error_delay=1m,evict_batch=8,evict_delay=1m,cache_limit=32M,         cache_lowpct=90`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_index_stat_option](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_index_stat_option)|

## ndb_join_pushdown
|name|value|
|----|-----|
|Name|`ndb_join_pushdown`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_join_pushdown](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_join_pushdown)|

## ndb_log_bin
|name|value|
|----|-----|
|Name|`ndb_log_bin`|
|Command line|`--ndb-log-bin[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_log_bin](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_log_bin)|

## ndb_log_binlog_index
|name|value|
|----|-----|
|Name|`ndb_log_binlog_index`|
|Command line|`--ndb-log-binlog-index[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_log_binlog_index](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_log_binlog_index)|

## ndb_read_backup
|name|value|
|----|-----|
|Name|`ndb_read_backup`|
|Command line|`--ndb-read-backup[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_read_backup](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_read_backup)|

## ndb_recv_thread_activation_threshold
|name|value|
|----|-----|
|Name|`ndb_recv_thread_activation_threshold`|
|Command line|`--ndb-recv-thread-activation-threshold=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8`|
|Dynamic|`true`|
|Range|from: `0` to: `16`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_recv_thread_activation_threshold](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_recv_thread_activation_threshold)|

## ndb_recv_thread_cpu_mask
|name|value|
|----|-----|
|Name|`ndb_recv_thread_cpu_mask`|
|Command line|`--ndb-recv-thread-cpu-mask=mask`|
|Scope|`global`|
|Default value|`[empty]`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_recv_thread_cpu_mask](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_recv_thread_cpu_mask)|

## ndb_report_thresh_binlog_epoch_slip
|name|value|
|----|-----|
|Name|`ndb_report_thresh_binlog_epoch_slip`|
|Command line|`--ndb-report-thresh-binlog-epoch-slip=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_report_thresh_binlog_epoch_slip](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_report_thresh_binlog_epoch_slip)|

## ndb_report_thresh_binlog_mem_usage
|name|value|
|----|-----|
|Name|`ndb_report_thresh_binlog_mem_usage`|
|Command line|`--ndb-report-thresh-binlog-mem-usage=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `10`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_report_thresh_binlog_mem_usage](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_report_thresh_binlog_mem_usage)|

## ndb_row_checksum
|name|value|
|----|-----|
|Name|`ndb_row_checksum`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `0` to: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_row_checksum](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_row_checksum)|

## ndb_show_foreign_key_mock_tables
|name|value|
|----|-----|
|Name|`ndb_show_foreign_key_mock_tables`|
|Command line|`--ndb-show-foreign-key-mock-tables[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_show_foreign_key_mock_tables](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_show_foreign_key_mock_tables)|

## ndb_slave_conflict_role
|name|value|
|----|-----|
|Name|`ndb_slave_conflict_role`|
|Command line|`--ndb-slave-conflict-role=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`NONE`|
|Dynamic|`true`|
|Valid value(s)|`NONE`, `PRIMARY`, `SECONDARY`, `PASS`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_slave_conflict_role](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_slave_conflict_role)|

## ndb_table_no_logging
|name|value|
|----|-----|
|Name|`ndb_table_no_logging`|
|Type of variable|`boolean`|
|Scope|`session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_table_no_logging](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_table_no_logging)|

## ndb_table_temporary
|name|value|
|----|-----|
|Name|`ndb_table_temporary`|
|Type of variable|`boolean`|
|Scope|`session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_table_temporary](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_table_temporary)|

## ndb_use_copying_alter_table
|name|value|
|----|-----|
|Name|`ndb_use_copying_alter_table`|
|Scope|`global`, `session`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_use_copying_alter_table](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_use_copying_alter_table)|

## ndb_use_exact_count
|name|value|
|----|-----|
|Name|`ndb_use_exact_count`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_use_exact_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_use_exact_count)|

## ndb_use_transactions
|name|value|
|----|-----|
|Name|`ndb_use_transactions`|
|Command line|`--ndb-use-transactions[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_use_transactions](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_use_transactions)|

## ndb_version
|name|value|
|----|-----|
|Name|`ndb_version`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|``|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_version](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_version)|

## ndb_version_string
|name|value|
|----|-----|
|Name|`ndb_version_string`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|``|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndb_version_string](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndb_version_string)|

## server_id_bits
|name|value|
|----|-----|
|Name|`server_id_bits`|
|Command line|`--server-id-bits=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32`|
|Dynamic|`false`|
|Range|from: `7` to: `32`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_server_id_bits](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_server_id_bits)|

## slave_allow_batching
|name|value|
|----|-----|
|Name|`slave_allow_batching`|
|Command line|`--slave-allow-batching[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_allow_batching](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_slave_allow_batching)|

## transaction_allow_batching
|name|value|
|----|-----|
|Name|`transaction_allow_batching`|
|Type of variable|`boolean`|
|Scope|`session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_transaction_allow_batching](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_transaction_allow_batching)|

## ndbinfo_database
|name|value|
|----|-----|
|Name|`ndbinfo_database`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`ndbinfo`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndbinfo_database](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndbinfo_database)|

## ndbinfo_max_bytes
|name|value|
|----|-----|
|Name|`ndbinfo_max_bytes`|
|Command line|`--ndbinfo-max-bytes=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndbinfo_max_bytes](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndbinfo_max_bytes)|

## ndbinfo_max_rows
|name|value|
|----|-----|
|Name|`ndbinfo_max_rows`|
|Command line|`--ndbinfo-max-rows=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `1` to: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndbinfo_max_rows](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndbinfo_max_rows)|

## ndbinfo_offline
|name|value|
|----|-----|
|Name|`ndbinfo_offline`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndbinfo_offline](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndbinfo_offline)|

## ndbinfo_show_hidden
|name|value|
|----|-----|
|Name|`ndbinfo_show_hidden`|
|Command line|`--ndbinfo-show-hidden[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|
|Valid value(s)|`ON`, `OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndbinfo_show_hidden](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndbinfo_show_hidden)|

## ndbinfo_table_prefix
|name|value|
|----|-----|
|Name|`ndbinfo_table_prefix`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`ndb$`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndbinfo_table_prefix](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndbinfo_table_prefix)|

## ndbinfo_version
|name|value|
|----|-----|
|Name|`ndbinfo_version`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|``|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ndbinfo_version](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#sysvar_ndbinfo_version)|

## Ndb_pushed_queries_defined
|name|value|
|----|-----|
|Name|`Ndb_pushed_queries_defined`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_pushed_queries_defined](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_pushed_queries_defined)|

## Handler_discover
|name|value|
|----|-----|
|Name|`Handler_discover`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_discover](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Handler_discover)|

## Ndb_api_adaptive_send_deferred_count
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_deferred_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_deferred_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_deferred_count)|

## Ndb_api_adaptive_send_deferred_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_deferred_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_deferred_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_deferred_count_session)|

## Ndb_api_adaptive_send_deferred_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_deferred_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_deferred_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_deferred_count_slave)|

## Ndb_api_adaptive_send_forced_count
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_forced_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_forced_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_forced_count)|

## Ndb_api_adaptive_send_forced_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_forced_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_forced_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_forced_count_session)|

## Ndb_api_adaptive_send_forced_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_forced_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_forced_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_forced_count_slave)|

## Ndb_api_adaptive_send_unforced_count
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_unforced_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_unforced_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_unforced_count)|

## Ndb_api_adaptive_send_unforced_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_unforced_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_unforced_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_unforced_count_session)|

## Ndb_api_adaptive_send_unforced_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_adaptive_send_unforced_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_adaptive_send_unforced_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_adaptive_send_unforced_count_slave)|

## Ndb_api_bytes_sent_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_bytes_sent_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_bytes_sent_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_bytes_sent_count_session)|

## Ndb_api_bytes_sent_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_bytes_sent_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_bytes_sent_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_bytes_sent_count_slave)|

## Ndb_api_bytes_sent_count
|name|value|
|----|-----|
|Name|`Ndb_api_bytes_sent_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_bytes_sent_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_bytes_sent_count)|

## Ndb_api_bytes_received_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_bytes_received_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_bytes_received_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_bytes_received_count_session)|

## Ndb_api_bytes_received_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_bytes_received_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_bytes_received_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_bytes_received_count_slave)|

## Ndb_api_bytes_received_count
|name|value|
|----|-----|
|Name|`Ndb_api_bytes_received_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_bytes_received_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_bytes_received_count)|

## Ndb_api_event_data_count_injector
|name|value|
|----|-----|
|Name|`Ndb_api_event_data_count_injector`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_event_data_count_injector](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_event_data_count_injector)|

## Ndb_api_event_data_count
|name|value|
|----|-----|
|Name|`Ndb_api_event_data_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_event_data_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_event_data_count)|

## Ndb_api_event_nondata_count_injector
|name|value|
|----|-----|
|Name|`Ndb_api_event_nondata_count_injector`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_event_nondata_count_injector](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_event_nondata_count_injector)|

## Ndb_api_event_nondata_count
|name|value|
|----|-----|
|Name|`Ndb_api_event_nondata_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_event_nondata_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_event_nondata_count)|

## Ndb_api_event_bytes_count_injector
|name|value|
|----|-----|
|Name|`Ndb_api_event_bytes_count_injector`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_event_bytes_count_injector](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_event_bytes_count_injector)|

## Ndb_api_event_bytes_count
|name|value|
|----|-----|
|Name|`Ndb_api_event_bytes_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_event_bytes_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_event_bytes_count)|

## Ndb_api_pk_op_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_pk_op_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_pk_op_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_pk_op_count_session)|

## Ndb_api_pk_op_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_pk_op_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_pk_op_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_pk_op_count_slave)|

## Ndb_api_pk_op_count
|name|value|
|----|-----|
|Name|`Ndb_api_pk_op_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_pk_op_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_pk_op_count)|

## Ndb_api_pruned_scan_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_pruned_scan_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_pruned_scan_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_pruned_scan_count_session)|

## Ndb_api_pruned_scan_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_pruned_scan_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_pruned_scan_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_pruned_scan_count_slave)|

## Ndb_api_pruned_scan_count
|name|value|
|----|-----|
|Name|`Ndb_api_pruned_scan_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_pruned_scan_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_pruned_scan_count)|

## Ndb_api_range_scan_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_range_scan_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_range_scan_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_range_scan_count_session)|

## Ndb_api_range_scan_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_range_scan_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_range_scan_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_range_scan_count_slave)|

## Ndb_api_range_scan_count
|name|value|
|----|-----|
|Name|`Ndb_api_range_scan_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_range_scan_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_range_scan_count)|

## Ndb_api_read_row_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_read_row_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_read_row_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_read_row_count_session)|

## Ndb_api_read_row_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_read_row_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_read_row_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_read_row_count_slave)|

## Ndb_api_read_row_count
|name|value|
|----|-----|
|Name|`Ndb_api_read_row_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_read_row_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_read_row_count)|

## Ndb_api_scan_batch_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_scan_batch_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_scan_batch_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_scan_batch_count_session)|

## Ndb_api_scan_batch_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_scan_batch_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_scan_batch_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_scan_batch_count_slave)|

## Ndb_api_scan_batch_count
|name|value|
|----|-----|
|Name|`Ndb_api_scan_batch_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_scan_batch_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_scan_batch_count)|

## Ndb_api_table_scan_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_table_scan_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_table_scan_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_table_scan_count_session)|

## Ndb_api_table_scan_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_table_scan_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_table_scan_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_table_scan_count_slave)|

## Ndb_api_table_scan_count
|name|value|
|----|-----|
|Name|`Ndb_api_table_scan_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_table_scan_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_table_scan_count)|

## Ndb_api_trans_abort_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_trans_abort_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_abort_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_abort_count_session)|

## Ndb_api_trans_abort_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_trans_abort_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_abort_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_abort_count_slave)|

## Ndb_api_trans_abort_count
|name|value|
|----|-----|
|Name|`Ndb_api_trans_abort_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_abort_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_abort_count)|

## Ndb_api_trans_close_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_trans_close_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_close_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_close_count_session)|

## Ndb_api_trans_close_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_trans_close_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_close_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_close_count_slave)|

## Ndb_api_trans_close_count
|name|value|
|----|-----|
|Name|`Ndb_api_trans_close_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_close_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_close_count)|

## Ndb_api_trans_commit_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_trans_commit_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_commit_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_commit_count_session)|

## Ndb_api_trans_commit_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_trans_commit_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_commit_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_commit_count_slave)|

## Ndb_api_trans_commit_count
|name|value|
|----|-----|
|Name|`Ndb_api_trans_commit_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_commit_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_commit_count)|

## Ndb_api_trans_local_read_row_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_trans_local_read_row_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_local_read_row_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_local_read_row_count_session)|

## Ndb_api_trans_local_read_row_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_trans_local_read_row_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_local_read_row_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_local_read_row_count_slave)|

## Ndb_api_trans_local_read_row_count
|name|value|
|----|-----|
|Name|`Ndb_api_trans_local_read_row_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_local_read_row_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_local_read_row_count)|

## Ndb_api_trans_start_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_trans_start_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_start_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_start_count_session)|

## Ndb_api_trans_start_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_trans_start_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_start_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_start_count_slave)|

## Ndb_api_trans_start_count
|name|value|
|----|-----|
|Name|`Ndb_api_trans_start_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_trans_start_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_trans_start_count)|

## Ndb_api_uk_op_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_uk_op_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_uk_op_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_uk_op_count_session)|

## Ndb_api_uk_op_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_uk_op_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_uk_op_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_uk_op_count_slave)|

## Ndb_api_uk_op_count
|name|value|
|----|-----|
|Name|`Ndb_api_uk_op_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_uk_op_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_uk_op_count)|

## Ndb_api_wait_exec_complete_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_wait_exec_complete_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_exec_complete_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_exec_complete_count_session)|

## Ndb_api_wait_exec_complete_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_wait_exec_complete_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_exec_complete_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_exec_complete_count_slave)|

## Ndb_api_wait_exec_complete_count
|name|value|
|----|-----|
|Name|`Ndb_api_wait_exec_complete_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_exec_complete_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_exec_complete_count)|

## Ndb_api_wait_meta_request_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_wait_meta_request_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_meta_request_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_meta_request_count_session)|

## Ndb_api_wait_meta_request_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_wait_meta_request_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_meta_request_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_meta_request_count_slave)|

## Ndb_api_wait_meta_request_count
|name|value|
|----|-----|
|Name|`Ndb_api_wait_meta_request_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_meta_request_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_meta_request_count)|

## Ndb_api_wait_nanos_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_wait_nanos_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_nanos_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_nanos_count_session)|

## Ndb_api_wait_nanos_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_wait_nanos_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_nanos_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_nanos_count_slave)|

## Ndb_api_wait_nanos_count
|name|value|
|----|-----|
|Name|`Ndb_api_wait_nanos_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_nanos_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_nanos_count)|

## Ndb_api_wait_scan_result_count_session
|name|value|
|----|-----|
|Name|`Ndb_api_wait_scan_result_count_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_scan_result_count_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_scan_result_count_session)|

## Ndb_api_wait_scan_result_count_slave
|name|value|
|----|-----|
|Name|`Ndb_api_wait_scan_result_count_slave`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_scan_result_count_slave](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_scan_result_count_slave)|

## Ndb_api_wait_scan_result_count
|name|value|
|----|-----|
|Name|`Ndb_api_wait_scan_result_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_api_wait_scan_result_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_api_wait_scan_result_count)|

## Ndb_cluster_node_id
|name|value|
|----|-----|
|Name|`Ndb_cluster_node_id`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_cluster_node_id](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_cluster_node_id)|

## Ndb_config_from_host
|name|value|
|----|-----|
|Name|`Ndb_config_from_host`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_config_from_host](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_config_from_host)|

## Ndb_config_from_port
|name|value|
|----|-----|
|Name|`Ndb_config_from_port`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_config_from_port](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_config_from_port)|

## Ndb_conflict_fn_epoch
|name|value|
|----|-----|
|Name|`Ndb_conflict_fn_epoch`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_fn_epoch](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_fn_epoch)|

## Ndb_conflict_fn_epoch_trans
|name|value|
|----|-----|
|Name|`Ndb_conflict_fn_epoch_trans`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_fn_epoch_trans](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_fn_epoch_trans)|

## Ndb_conflict_fn_epoch2
|name|value|
|----|-----|
|Name|`Ndb_conflict_fn_epoch2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_fn_epoch2](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_fn_epoch2)|

## Ndb_conflict_fn_epoch2_trans
|name|value|
|----|-----|
|Name|`Ndb_conflict_fn_epoch2_trans`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_fn_epoch2_trans](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_fn_epoch2_trans)|

## Ndb_conflict_fn_max
|name|value|
|----|-----|
|Name|`Ndb_conflict_fn_max`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_fn_max](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_fn_max)|

## Ndb_conflict_fn_max_del_win
|name|value|
|----|-----|
|Name|`Ndb_conflict_fn_max_del_win`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_fn_max_del_win](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_fn_max_del_win)|

## Ndb_conflict_fn_old
|name|value|
|----|-----|
|Name|`Ndb_conflict_fn_old`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_fn_old](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_fn_old)|

## Ndb_conflict_last_conflict_epoch
|name|value|
|----|-----|
|Name|`Ndb_conflict_last_conflict_epoch`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_last_conflict_epoch](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_last_conflict_epoch)|

## Ndb_conflict_reflected_op_discard_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_reflected_op_discard_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_reflected_op_discard_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_reflected_op_discard_count)|

## Ndb_conflict_reflected_op_prepare_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_reflected_op_prepare_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_reflected_op_prepare_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_reflected_op_prepare_count)|

## Ndb_conflict_refresh_op_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_refresh_op_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_refresh_op_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_refresh_op_count)|

## Ndb_conflict_last_stable_epoch
|name|value|
|----|-----|
|Name|`Ndb_conflict_last_stable_epoch`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_last_stable_epoch](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_last_stable_epoch)|

## Ndb_conflict_trans_row_conflict_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_trans_row_conflict_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_trans_row_conflict_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_trans_row_conflict_count)|

## Ndb_conflict_trans_row_reject_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_trans_row_reject_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_trans_row_reject_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_trans_row_reject_count)|

## Ndb_conflict_trans_reject_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_trans_reject_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_trans_reject_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_trans_reject_count)|

## Ndb_conflict_trans_detect_iter_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_trans_detect_iter_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_trans_detect_iter_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_trans_detect_iter_count)|

## Ndb_conflict_trans_conflict_commit_count
|name|value|
|----|-----|
|Name|`Ndb_conflict_trans_conflict_commit_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_conflict_trans_conflict_commit_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_conflict_trans_conflict_commit_count)|

## Ndb_epoch_delete_delete_count
|name|value|
|----|-----|
|Name|`Ndb_epoch_delete_delete_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_epoch_delete_delete_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_epoch_delete_delete_count)|

## Ndb_execute_count
|name|value|
|----|-----|
|Name|`Ndb_execute_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_execute_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_execute_count)|

## Ndb_last_commit_epoch_server
|name|value|
|----|-----|
|Name|`Ndb_last_commit_epoch_server`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_last_commit_epoch_server](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_last_commit_epoch_server)|

## Ndb_last_commit_epoch_session
|name|value|
|----|-----|
|Name|`Ndb_last_commit_epoch_session`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_last_commit_epoch_session](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_last_commit_epoch_session)|

## Ndb_number_of_data_nodes
|name|value|
|----|-----|
|Name|`Ndb_number_of_data_nodes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_number_of_data_nodes](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_number_of_data_nodes)|

## Ndb_pushed_queries_dropped
|name|value|
|----|-----|
|Name|`Ndb_pushed_queries_dropped`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_pushed_queries_dropped](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_pushed_queries_dropped)|

## Ndb_pushed_queries_executed
|name|value|
|----|-----|
|Name|`Ndb_pushed_queries_executed`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_pushed_queries_executed](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_pushed_queries_executed)|

## Ndb_pushed_reads
|name|value|
|----|-----|
|Name|`Ndb_pushed_reads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_pushed_reads](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_pushed_reads)|

## Ndb_pruned_scan_count
|name|value|
|----|-----|
|Name|`Ndb_pruned_scan_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_pruned_scan_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_pruned_scan_count)|

## Ndb_scan_count
|name|value|
|----|-----|
|Name|`Ndb_scan_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_scan_count](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_scan_count)|

## Ndb_slave_max_replicated_epoch
|name|value|
|----|-----|
|Name|`Ndb_slave_max_replicated_epoch`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_slave_max_replicated_epoch](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_slave_max_replicated_epoch)|

## Ndb_system_name
|name|value|
|----|-----|
|Name|`Ndb_system_name`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ndb_system_name](https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#statvar_Ndb_system_name)|

## performance_schema
|name|value|
|----|-----|
|Name|`performance_schema`|
|Command line|`--performance-schema[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema)|

## performance_schema_accounts_size
|name|value|
|----|-----|
|Name|`performance_schema_accounts_size`|
|Command line|`--performance-schema-accounts-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_accounts_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_accounts_size)|

## performance_schema_digests_size
|name|value|
|----|-----|
|Name|`performance_schema_digests_size`|
|Command line|`--performance-schema-digests-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_digests_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_digests_size)|

## performance_schema_error_size
|name|value|
|----|-----|
|Name|`performance_schema_error_size`|
|Command line|`--performance-schema-error-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`number of server error codes`|
|Dynamic|`false`|
|Range|from: `0` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_error_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_error_size)|

## performance_schema_events_stages_history_long_size
|name|value|
|----|-----|
|Name|`performance_schema_events_stages_history_long_size`|
|Command line|`--performance-schema-events-stages-history-long-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_stages_history_long_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_stages_history_long_size)|

## performance_schema_events_stages_history_size
|name|value|
|----|-----|
|Name|`performance_schema_events_stages_history_size`|
|Command line|`--performance-schema-events-stages-history-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_stages_history_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_stages_history_size)|

## performance_schema_events_statements_history_long_size
|name|value|
|----|-----|
|Name|`performance_schema_events_statements_history_long_size`|
|Command line|`--performance-schema-events-statements-history-long-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_statements_history_long_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_statements_history_long_size)|

## performance_schema_events_statements_history_size
|name|value|
|----|-----|
|Name|`performance_schema_events_statements_history_size`|
|Command line|`--performance-schema-events-statements-history-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_statements_history_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_statements_history_size)|

## performance_schema_events_transactions_history_long_size
|name|value|
|----|-----|
|Name|`performance_schema_events_transactions_history_long_size`|
|Command line|`--performance-schema-events-transactions-history-long-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_transactions_history_long_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_transactions_history_long_size)|

## performance_schema_events_transactions_history_size
|name|value|
|----|-----|
|Name|`performance_schema_events_transactions_history_size`|
|Command line|`--performance-schema-events-transactions-history-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_transactions_history_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_transactions_history_size)|

## performance_schema_events_waits_history_long_size
|name|value|
|----|-----|
|Name|`performance_schema_events_waits_history_long_size`|
|Command line|`--performance-schema-events-waits-history-long-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_waits_history_long_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_waits_history_long_size)|

## performance_schema_events_waits_history_size
|name|value|
|----|-----|
|Name|`performance_schema_events_waits_history_size`|
|Command line|`--performance-schema-events-waits-history-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_events_waits_history_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_events_waits_history_size)|

## performance_schema_hosts_size
|name|value|
|----|-----|
|Name|`performance_schema_hosts_size`|
|Command line|`--performance-schema-hosts-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_hosts_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_hosts_size)|

## performance_schema_max_cond_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_cond_classes`|
|Command line|`--performance-schema-max-cond-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`150`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_cond_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_cond_classes)|

## performance_schema_max_cond_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_cond_instances`|
|Command line|`--performance-schema-max-cond-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_cond_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_cond_instances)|

## performance_schema_max_digest_length
|name|value|
|----|-----|
|Name|`performance_schema_max_digest_length`|
|Command line|`--performance-schema-max-digest-length=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1024`|
|Dynamic|`false`|
|Range|from: `0` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_digest_length](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_digest_length)|

## performance_schema_max_digest_sample_age
|name|value|
|----|-----|
|Name|`performance_schema_max_digest_sample_age`|
|Command line|`--performance-schema-max-digest-sample-age=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`60`|
|Dynamic|`true`|
|Range|from: `0` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_digest_sample_age](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_digest_sample_age)|

## performance_schema_max_file_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_file_classes`|
|Command line|`--performance-schema-max-file-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`80`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_file_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_file_classes)|

## performance_schema_max_file_handles
|name|value|
|----|-----|
|Name|`performance_schema_max_file_handles`|
|Command line|`--performance-schema-max-file-handles=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32768`|
|Dynamic|`false`|
|Range|from: `0` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_file_handles](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_file_handles)|

## performance_schema_max_file_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_file_instances`|
|Command line|`--performance-schema-max-file-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_file_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_file_instances)|

## performance_schema_max_index_stat
|name|value|
|----|-----|
|Name|`performance_schema_max_index_stat`|
|Command line|`--performance-schema-max-index-stat=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_index_stat](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_index_stat)|

## performance_schema_max_memory_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_memory_classes`|
|Command line|`--performance-schema-max-memory-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`450`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_memory_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_memory_classes)|

## performance_schema_max_metadata_locks
|name|value|
|----|-----|
|Name|`performance_schema_max_metadata_locks`|
|Command line|`--performance-schema-max-metadata-locks=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `10485760`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_metadata_locks](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_metadata_locks)|

## performance_schema_max_mutex_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_mutex_classes`|
|Command line|`--performance-schema-max-mutex-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`350`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_mutex_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_mutex_classes)|

## performance_schema_max_mutex_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_mutex_instances`|
|Command line|`--performance-schema-max-mutex-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `104857600`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_mutex_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_mutex_instances)|

## performance_schema_max_prepared_statements_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_prepared_statements_instances`|
|Command line|`--performance-schema-max-prepared-statements-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `4194304`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_prepared_statements_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_prepared_statements_instances)|

## performance_schema_max_rwlock_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_rwlock_classes`|
|Command line|`--performance-schema-max-rwlock-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_rwlock_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_rwlock_classes)|

## performance_schema_max_program_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_program_instances`|
|Command line|`--performance-schema-max-program-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_program_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_program_instances)|

## performance_schema_max_rwlock_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_rwlock_instances`|
|Command line|`--performance-schema-max-rwlock-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `104857600`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_rwlock_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_rwlock_instances)|

## performance_schema_max_socket_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_socket_classes`|
|Command line|`--performance-schema-max-socket-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_socket_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_socket_classes)|

## performance_schema_max_socket_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_socket_instances`|
|Command line|`--performance-schema-max-socket-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_socket_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_socket_instances)|

## performance_schema_max_sql_text_length
|name|value|
|----|-----|
|Name|`performance_schema_max_sql_text_length`|
|Command line|`--performance-schema-max-sql-text-length=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1024`|
|Dynamic|`false`|
|Range|from: `0` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_sql_text_length](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_sql_text_length)|

## performance_schema_max_stage_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_stage_classes`|
|Command line|`--performance-schema-max-stage-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`175`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_stage_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_stage_classes)|

## performance_schema_max_statement_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_statement_classes`|
|Command line|`--performance-schema-max-statement-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`false`|
|Range|from: `0` to: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_statement_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_statement_classes)|

## performance_schema_max_statement_stack
|name|value|
|----|-----|
|Name|`performance_schema_max_statement_stack`|
|Command line|`--performance-schema-max-statement-stack=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`false`|
|Range|from: `1` to: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_statement_stack](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_statement_stack)|

## performance_schema_max_table_handles
|name|value|
|----|-----|
|Name|`performance_schema_max_table_handles`|
|Command line|`--performance-schema-max-table-handles=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_table_handles](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_table_handles)|

## performance_schema_max_table_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_table_instances`|
|Command line|`--performance-schema-max-table-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_table_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_table_instances)|

## performance_schema_max_table_lock_stat
|name|value|
|----|-----|
|Name|`performance_schema_max_table_lock_stat`|
|Command line|`--performance-schema-max-table-lock-stat=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_table_lock_stat](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_table_lock_stat)|

## performance_schema_max_thread_classes
|name|value|
|----|-----|
|Name|`performance_schema_max_thread_classes`|
|Command line|`--performance-schema-max-thread-classes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`false`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_thread_classes](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_thread_classes)|

## performance_schema_max_thread_instances
|name|value|
|----|-----|
|Name|`performance_schema_max_thread_instances`|
|Command line|`--performance-schema-max-thread-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_max_thread_instances](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_max_thread_instances)|

## performance_schema_session_connect_attrs_size
|name|value|
|----|-----|
|Name|`performance_schema_session_connect_attrs_size`|
|Command line|`--performance-schema-session-connect-attrs-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_session_connect_attrs_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_session_connect_attrs_size)|

## performance_schema_setup_actors_size
|name|value|
|----|-----|
|Name|`performance_schema_setup_actors_size`|
|Command line|`--performance-schema-setup-actors-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_setup_actors_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_setup_actors_size)|

## performance_schema_setup_objects_size
|name|value|
|----|-----|
|Name|`performance_schema_setup_objects_size`|
|Command line|`--performance-schema-setup-objects-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_setup_objects_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_setup_objects_size)|

## performance_schema_show_processlist
|name|value|
|----|-----|
|Name|`performance_schema_show_processlist`|
|Command line|`--performance-schema-show-processlist[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_show_processlist](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_show_processlist)|

## performance_schema_users_size
|name|value|
|----|-----|
|Name|`performance_schema_users_size`|
|Command line|`--performance-schema-users-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autoscaling; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `-1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_performance_schema_users_size](https://dev.mysql.com/doc/refman/8.0/en/performance-schema-system-variables.html#sysvar_performance_schema_users_size)|

## authentication_fido_rp_id
|name|value|
|----|-----|
|Name|`authentication_fido_rp_id`|
|Command line|`--authentication-fido-rp-id=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`MySQL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_fido_rp_id](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_fido_rp_id)|

## authentication_kerberos_service_key_tab
|name|value|
|----|-----|
|Name|`authentication_kerberos_service_key_tab`|
|Command line|`--authentication-kerberos-service-key-tab=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`datadir/mysql.keytab`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_kerberos_service_key_tab](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_kerberos_service_key_tab)|

## authentication_kerberos_service_principal
|name|value|
|----|-----|
|Name|`authentication_kerberos_service_principal`|
|Command line|`--authentication-kerberos-service-principal=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`mysql/host_name@realm_name`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_kerberos_service_principal](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_kerberos_service_principal)|

## authentication_ldap_sasl_auth_method_name
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_auth_method_name`|
|Command line|`--authentication-ldap-sasl-auth-method-name=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`SCRAM-SHA-1`|
|Dynamic|`true`|
|Valid value(s)|`SCRAM-SHA-1`, `SCRAM-SHA-256`, `GSSAPI`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_auth_method_name](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_auth_method_name)|

## authentication_ldap_sasl_bind_base_dn
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_bind_base_dn`|
|Command line|`--authentication-ldap-sasl-bind-base-dn=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_bind_base_dn](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_bind_base_dn)|

## authentication_ldap_sasl_bind_root_dn
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_bind_root_dn`|
|Command line|`--authentication-ldap-sasl-bind-root-dn=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_bind_root_dn](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_bind_root_dn)|

## authentication_ldap_sasl_bind_root_pwd
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_bind_root_pwd`|
|Command line|`--authentication-ldap-sasl-bind-root-pwd=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_bind_root_pwd](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_bind_root_pwd)|

## authentication_ldap_sasl_ca_path
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_ca_path`|
|Command line|`--authentication-ldap-sasl-ca-path=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_ca_path](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_ca_path)|

## authentication_ldap_sasl_group_search_attr
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_group_search_attr`|
|Command line|`--authentication-ldap-sasl-group-search-attr=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`cn`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_group_search_attr](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_group_search_attr)|

## authentication_ldap_sasl_group_search_filter
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_group_search_filter`|
|Command line|`--authentication-ldap-sasl-group-search-filter=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`(|(&(objectClass=posixGroup)(memberUid=%s))(&(objectClass=group)(member=%s)))`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_group_search_filter](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_group_search_filter)|

## authentication_ldap_sasl_init_pool_size
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_init_pool_size`|
|Command line|`--authentication-ldap-sasl-init-pool-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `32767`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_init_pool_size](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_init_pool_size)|

## authentication_ldap_sasl_log_status
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_log_status`|
|Command line|`--authentication-ldap-sasl-log-status=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `6`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_log_status](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_log_status)|

## authentication_ldap_sasl_max_pool_size
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_max_pool_size`|
|Command line|`--authentication-ldap-sasl-max-pool-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1000`|
|Dynamic|`true`|
|Range|from: `0` to: `32767`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_max_pool_size](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_max_pool_size)|

## authentication_ldap_sasl_referral
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_referral`|
|Command line|`--authentication-ldap-sasl-referral[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_referral](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_referral)|

## authentication_ldap_sasl_server_host
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_server_host`|
|Command line|`--authentication-ldap-sasl-server-host=host_name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_server_host](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_server_host)|

## authentication_ldap_sasl_server_port
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_server_port`|
|Command line|`--authentication-ldap-sasl-server-port=port_num`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`389`|
|Dynamic|`true`|
|Range|from: `1` to: `32376`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_server_port](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_server_port)|

## authentication_ldap_sasl_tls
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_tls`|
|Command line|`--authentication-ldap-sasl-tls[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_tls](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_tls)|

## authentication_ldap_sasl_user_search_attr
|name|value|
|----|-----|
|Name|`authentication_ldap_sasl_user_search_attr`|
|Command line|`--authentication-ldap-sasl-user-search-attr=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`uid`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_sasl_user_search_attr](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_sasl_user_search_attr)|

## authentication_ldap_simple_auth_method_name
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_auth_method_name`|
|Command line|`--authentication-ldap-simple-auth-method-name=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`SIMPLE`|
|Dynamic|`true`|
|Valid value(s)|`SIMPLE`, `AD-FOREST`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_auth_method_name](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_auth_method_name)|

## authentication_ldap_simple_bind_base_dn
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_bind_base_dn`|
|Command line|`--authentication-ldap-simple-bind-base-dn=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_bind_base_dn](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_bind_base_dn)|

## authentication_ldap_simple_bind_root_dn
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_bind_root_dn`|
|Command line|`--authentication-ldap-simple-bind-root-dn=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_bind_root_dn](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_bind_root_dn)|

## authentication_ldap_simple_bind_root_pwd
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_bind_root_pwd`|
|Command line|`--authentication-ldap-simple-bind-root-pwd=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_bind_root_pwd](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_bind_root_pwd)|

## authentication_ldap_simple_ca_path
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_ca_path`|
|Command line|`--authentication-ldap-simple-ca-path=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_ca_path](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_ca_path)|

## authentication_ldap_simple_group_search_attr
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_group_search_attr`|
|Command line|`--authentication-ldap-simple-group-search-attr=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`cn`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_group_search_attr](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_group_search_attr)|

## authentication_ldap_simple_group_search_filter
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_group_search_filter`|
|Command line|`--authentication-ldap-simple-group-search-filter=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`(|(&(objectClass=posixGroup)(memberUid=%s))(&(objectClass=group)(member=%s)))`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_group_search_filter](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_group_search_filter)|

## authentication_ldap_simple_init_pool_size
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_init_pool_size`|
|Command line|`--authentication-ldap-simple-init-pool-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `32767`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_init_pool_size](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_init_pool_size)|

## authentication_ldap_simple_log_status
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_log_status`|
|Command line|`--authentication-ldap-simple-log-status=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `6`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_log_status](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_log_status)|

## authentication_ldap_simple_max_pool_size
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_max_pool_size`|
|Command line|`--authentication-ldap-simple-max-pool-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1000`|
|Dynamic|`true`|
|Range|from: `0` to: `32767`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_max_pool_size](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_max_pool_size)|

## authentication_ldap_simple_referral
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_referral`|
|Command line|`--authentication-ldap-simple-referral[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_referral](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_referral)|

## authentication_ldap_simple_server_host
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_server_host`|
|Command line|`--authentication-ldap-simple-server-host=host_name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_server_host](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_server_host)|

## authentication_ldap_simple_server_port
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_server_port`|
|Command line|`--authentication-ldap-simple-server-port=port_num`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`389`|
|Dynamic|`true`|
|Range|from: `1` to: `32376`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_server_port](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_server_port)|

## authentication_ldap_simple_tls
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_tls`|
|Command line|`--authentication-ldap-simple-tls[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_tls](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_tls)|

## authentication_ldap_simple_user_search_attr
|name|value|
|----|-----|
|Name|`authentication_ldap_simple_user_search_attr`|
|Command line|`--authentication-ldap-simple-user-search-attr=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`uid`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_ldap_simple_user_search_attr](https://dev.mysql.com/doc/refman/8.0/en/pluggable-authentication-system-variables.html#sysvar_authentication_ldap_simple_user_search_attr)|

## binlog_row_event_max_size
|name|value|
|----|-----|
|Name|`binlog_row_event_max_size`|
|Command line|`--binlog-row-event-max-size=#`|
|Type of variable|`integer`|
|Default value|`8192`|
|Range|from: `256`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_binlog-row-event-max-size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_binlog-row-event-max-size)|
|dev.mysql.com|[option_mysqld_binlog-row-event-max-size](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_binlog-row-event-max-size)|

## log_bin
|name|value|
|----|-----|
|Name|`log_bin`|
|Command line|`--log-bin=file_name`|
|Type of variable|`file name`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-bin](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_log-bin)|
|dev.mysql.com|[option_mysqld_log-bin](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_log-bin)|

## log_bin_index
|name|value|
|----|-----|
|Name|`log_bin_index`|
|Command line|`--log-bin-index=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-bin-index](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_log-bin-index)|
|dev.mysql.com|[option_mysqld_log-bin-index](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_log-bin-index)|

## binlog_do_db
|name|value|
|----|-----|
|Name|`binlog_do_db`|
|Command line|`--binlog-do-db=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_binlog-do-db](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_binlog-do-db)|
|dev.mysql.com|[option_mysqld_binlog-do-db](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_binlog-do-db)|

## binlog_ignore_db
|name|value|
|----|-----|
|Name|`binlog_ignore_db`|
|Command line|`--binlog-ignore-db=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_binlog-ignore-db](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_binlog-ignore-db)|
|dev.mysql.com|[option_mysqld_binlog-ignore-db](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_binlog-ignore-db)|

## binlog_checksum
|name|value|
|----|-----|
|Name|`binlog_checksum`|
|Command line|`--binlog-checksum=type`|
|Type of variable|`string`|
|Default value|`CRC32`|
|Valid value(s)|`NONE`, `CRC32`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_binlog-checksum](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_binlog-checksum)|
|dev.mysql.com|[option_mysqld_binlog-checksum](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_binlog-checksum)|

## max_binlog_dump_events
|name|value|
|----|-----|
|Name|`max_binlog_dump_events`|
|Command line|`--max-binlog-dump-events=#`|
|Type of variable|`integer`|
|Default value|`0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_max-binlog-dump-events](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_max-binlog-dump-events)|
|dev.mysql.com|[option_mysqld_max-binlog-dump-events](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_max-binlog-dump-events)|

## sporadic_binlog_dump_fail
|name|value|
|----|-----|
|Name|`sporadic_binlog_dump_fail`|
|Command line|`--sporadic-binlog-dump-fail[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_sporadic-binlog-dump-fail](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#option_mysqld_sporadic-binlog-dump-fail)|
|dev.mysql.com|[option_mysqld_sporadic-binlog-dump-fail](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#option_mysqld_sporadic-binlog-dump-fail)|

## binlog_cache_size
|name|value|
|----|-----|
|Name|`binlog_cache_size`|
|Command line|`--binlog-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32768`|
|Dynamic|`true`|
|Range|from: `4096`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_cache_size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_cache_size)|
|dev.mysql.com|[sysvar_binlog_cache_size](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_cache_size)|

## binlog_direct_non_transactional_updates
|name|value|
|----|-----|
|Name|`binlog_direct_non_transactional_updates`|
|Command line|`--binlog-direct-non-transactional-updates[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_direct_non_transactional_updates](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_direct_non_transactional_updates)|
|dev.mysql.com|[sysvar_binlog_direct_non_transactional_updates](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_direct_non_transactional_updates)|

## binlog_encryption
|name|value|
|----|-----|
|Name|`binlog_encryption`|
|Command line|`--binlog-encryption[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_encryption](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_encryption)|

## binlog_error_action
|name|value|
|----|-----|
|Name|`binlog_error_action`|
|Command line|`--binlog-error-action[=value]`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`ABORT_SERVER`|
|Dynamic|`true`|
|Valid value(s)|`IGNORE_ERROR`, `ABORT_SERVER`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_error_action](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_error_action)|
|dev.mysql.com|[sysvar_binlog_error_action](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_error_action)|

## binlog_expire_logs_seconds
|name|value|
|----|-----|
|Name|`binlog_expire_logs_seconds`|
|Command line|`--binlog-expire-logs-seconds=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2592000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_expire_logs_seconds](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_expire_logs_seconds)|

## binlog_expire_logs_auto_purge
|name|value|
|----|-----|
|Name|`binlog_expire_logs_auto_purge`|
|Command line|`--binlog-expire-logs-auto-purge={ON|OFF}`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_expire_logs_auto_purge](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_expire_logs_auto_purge)|

## binlog_format
|name|value|
|----|-----|
|Name|`binlog_format`|
|Command line|`--binlog-format=format`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`ROW`|
|Dynamic|`true`|
|Valid value(s)|`MIXED`, `STATEMENT`, `ROW`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_format](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_format)|
|dev.mysql.com|[sysvar_binlog_format](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_format)|

## binlog_group_commit_sync_delay
|name|value|
|----|-----|
|Name|`binlog_group_commit_sync_delay`|
|Command line|`--binlog-group-commit-sync-delay=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_group_commit_sync_delay](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_group_commit_sync_delay)|
|dev.mysql.com|[sysvar_binlog_group_commit_sync_delay](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_group_commit_sync_delay)|

## binlog_group_commit_sync_no_delay_count
|name|value|
|----|-----|
|Name|`binlog_group_commit_sync_no_delay_count`|
|Command line|`--binlog-group-commit-sync-no-delay-count=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `100000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_group_commit_sync_no_delay_count](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_group_commit_sync_no_delay_count)|
|dev.mysql.com|[sysvar_binlog_group_commit_sync_no_delay_count](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_group_commit_sync_no_delay_count)|

## binlog_max_flush_queue_time
|name|value|
|----|-----|
|Name|`binlog_max_flush_queue_time`|
|Command line|`--binlog-max-flush-queue-time=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `100000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_max_flush_queue_time](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_max_flush_queue_time)|
|dev.mysql.com|[sysvar_binlog_max_flush_queue_time](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_max_flush_queue_time)|

## binlog_order_commits
|name|value|
|----|-----|
|Name|`binlog_order_commits`|
|Command line|`--binlog-order-commits[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_order_commits](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_order_commits)|
|dev.mysql.com|[sysvar_binlog_order_commits](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_order_commits)|

## binlog_rotate_encryption_master_key_at_startup
|name|value|
|----|-----|
|Name|`binlog_rotate_encryption_master_key_at_startup`|
|Command line|`--binlog-rotate-encryption-master-key-at-startup[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_rotate_encryption_master_key_at_startup](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_rotate_encryption_master_key_at_startup)|

## binlog_row_image
|name|value|
|----|-----|
|Name|`binlog_row_image`|
|Command line|`--binlog-row-image=image_type`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`full`|
|Dynamic|`true`|
|Valid value(s)|`full`, `minimal`, `noblob`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_row_image](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_row_image)|
|dev.mysql.com|[sysvar_binlog_row_image](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_row_image)|

## binlog_row_metadata
|name|value|
|----|-----|
|Name|`binlog_row_metadata`|
|Command line|`--binlog-row-metadata=metadata_type`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`MINIMAL`|
|Dynamic|`true`|
|Valid value(s)|`FULL`, `MINIMAL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_row_metadata](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_row_metadata)|

## binlog_row_value_options
|name|value|
|----|-----|
|Name|`binlog_row_value_options`|
|Command line|`--binlog-row-value-options=#`|
|Type of variable|`set`|
|Scope|`global`, `session`|
|Default value|``|
|Dynamic|`true`|
|Valid value(s)|`PARTIAL_JSON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_row_value_options](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_row_value_options)|

## binlog_rows_query_log_events
|name|value|
|----|-----|
|Name|`binlog_rows_query_log_events`|
|Command line|`--binlog-rows-query-log-events[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_rows_query_log_events](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_rows_query_log_events)|
|dev.mysql.com|[sysvar_binlog_rows_query_log_events](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_rows_query_log_events)|

## binlog_stmt_cache_size
|name|value|
|----|-----|
|Name|`binlog_stmt_cache_size`|
|Command line|`--binlog-stmt-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32768`|
|Dynamic|`true`|
|Range|from: `4096`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_stmt_cache_size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_stmt_cache_size)|
|dev.mysql.com|[sysvar_binlog_stmt_cache_size](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_stmt_cache_size)|

## binlog_transaction_compression
|name|value|
|----|-----|
|Name|`binlog_transaction_compression`|
|Command line|`--binlog-transaction-compression[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_transaction_compression](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_transaction_compression)|

## binlog_transaction_compression_level_zstd
|name|value|
|----|-----|
|Name|`binlog_transaction_compression_level_zstd`|
|Command line|`--binlog-transaction-compression-level-zstd=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`3`|
|Dynamic|`true`|
|Range|from: `1` to: `22`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_transaction_compression_level_zstd](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_transaction_compression_level_zstd)|

## binlog_transaction_dependency_tracking
|name|value|
|----|-----|
|Name|`binlog_transaction_dependency_tracking`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_transaction_dependency_tracking](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_transaction_dependency_tracking)|
|dev.mysql.com|[sysvar_binlog_transaction_dependency_tracking](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_transaction_dependency_tracking)|
|dev.mysql.com|[sysvar_binlog_transaction_dependency_tracking](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_binlog_transaction_dependency_tracking)|

## binlog_transaction_dependency_history_size
|name|value|
|----|-----|
|Name|`binlog_transaction_dependency_history_size`|
|Command line|`--binlog-transaction-dependency-history-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`25000`|
|Dynamic|`true`|
|Range|from: `1` to: `1000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_transaction_dependency_history_size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_transaction_dependency_history_size)|
|dev.mysql.com|[sysvar_binlog_transaction_dependency_history_size](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_binlog_transaction_dependency_history_size)|

## expire_logs_days
|name|value|
|----|-----|
|Name|`expire_logs_days`|
|Command line|`--expire-logs-days=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `99`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_expire_logs_days](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_expire_logs_days)|
|dev.mysql.com|[sysvar_expire_logs_days](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_expire_logs_days)|

## log_bin_basename
|name|value|
|----|-----|
|Name|`log_bin_basename`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_bin_basename](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_log_bin_basename)|
|dev.mysql.com|[sysvar_log_bin_basename](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_log_bin_basename)|

## log_bin_trust_function_creators
|name|value|
|----|-----|
|Name|`log_bin_trust_function_creators`|
|Command line|`--log-bin-trust-function-creators[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_bin_trust_function_creators](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_log_bin_trust_function_creators)|
|dev.mysql.com|[sysvar_log_bin_trust_function_creators](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_log_bin_trust_function_creators)|

## log_bin_use_v1_row_events
|name|value|
|----|-----|
|Name|`log_bin_use_v1_row_events`|
|Command line|`--log-bin-use-v1-row-events[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_bin_use_v1_row_events](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_log_bin_use_v1_row_events)|
|dev.mysql.com|[sysvar_log_bin_use_v1_row_events](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_log_bin_use_v1_row_events)|

## log_replica_updates
|name|value|
|----|-----|
|Name|`log_replica_updates`|
|Command line|`--log-replica-updates[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_replica_updates](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_log_replica_updates)|

## log_slave_updates
|name|value|
|----|-----|
|Name|`log_slave_updates`|
|Command line|`--log-slave-updates[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_slave_updates](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_log_slave_updates)|
|dev.mysql.com|[sysvar_log_slave_updates](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_log_slave_updates)|

## log_statements_unsafe_for_binlog
|name|value|
|----|-----|
|Name|`log_statements_unsafe_for_binlog`|
|Command line|`--log-statements-unsafe-for-binlog[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_statements_unsafe_for_binlog](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_log_statements_unsafe_for_binlog)|
|dev.mysql.com|[sysvar_log_statements_unsafe_for_binlog](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_log_statements_unsafe_for_binlog)|

## master_verify_checksum
|name|value|
|----|-----|
|Name|`master_verify_checksum`|
|Command line|`--master-verify-checksum[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_master_verify_checksum](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_master_verify_checksum)|
|dev.mysql.com|[sysvar_master_verify_checksum](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_master_verify_checksum)|

## max_binlog_cache_size
|name|value|
|----|-----|
|Name|`max_binlog_cache_size`|
|Command line|`--max-binlog-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`true`|
|Range|from: `4096`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_binlog_cache_size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_max_binlog_cache_size)|
|dev.mysql.com|[sysvar_max_binlog_cache_size](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_max_binlog_cache_size)|

## max_binlog_size
|name|value|
|----|-----|
|Name|`max_binlog_size`|
|Command line|`--max-binlog-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1073741824`|
|Dynamic|`true`|
|Range|from: `4096` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_binlog_size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_max_binlog_size)|
|dev.mysql.com|[sysvar_max_binlog_size](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_max_binlog_size)|

## max_binlog_stmt_cache_size
|name|value|
|----|-----|
|Name|`max_binlog_stmt_cache_size`|
|Command line|`--max-binlog-stmt-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`18446744073709547520`|
|Dynamic|`true`|
|Range|from: `4096` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_binlog_stmt_cache_size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_max_binlog_stmt_cache_size)|
|dev.mysql.com|[sysvar_max_binlog_stmt_cache_size](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_max_binlog_stmt_cache_size)|

## original_commit_timestamp
|name|value|
|----|-----|
|Name|`original_commit_timestamp`|
|Type of variable|`numeric`|
|Scope|`session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_original_commit_timestamp](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_original_commit_timestamp)|

## source_verify_checksum
|name|value|
|----|-----|
|Name|`source_verify_checksum`|
|Command line|`--source-verify-checksum[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_source_verify_checksum](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_source_verify_checksum)|

## sql_log_bin
|name|value|
|----|-----|
|Name|`sql_log_bin`|
|Type of variable|`boolean`|
|Scope|`session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_log_bin](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_sql_log_bin)|
|dev.mysql.com|[sysvar_sql_log_bin](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_sql_log_bin)|

## sync_binlog
|name|value|
|----|-----|
|Name|`sync_binlog`|
|Command line|`--sync-binlog=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sync_binlog](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_sync_binlog)|
|dev.mysql.com|[sysvar_sync_binlog](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_sync_binlog)|

## transaction_write_set_extraction
|name|value|
|----|-----|
|Name|`transaction_write_set_extraction`|
|Command line|`--transaction-write-set-extraction[=value]`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `MURMUR32`, `XXHASH64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_transaction_write_set_extraction](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_transaction_write_set_extraction)|
|dev.mysql.com|[sysvar_transaction_write_set_extraction](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_transaction_write_set_extraction)|

## max_connections
|name|value|
|----|-----|
|Name|`max_connections`|
|Command line|`--max-connections=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`151`|
|Dynamic|`true`|
|Range|from: `1` to: `100000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_connections](https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_max_connections)|
|dev.mysql.com|[sysvar_max_connections](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_max_connections)|
|dev.mysql.com|[sysvar_max_connections](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_connections)|
|dev.mysql.com|[sysvar_max_connections](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_connections)|

## log_builtin_as_identified_by_password
|name|value|
|----|-----|
|Name|`log_builtin_as_identified_by_password`|
|Command line|`--log-builtin-as-identified-by-password[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_builtin_as_identified_by_password](https://dev.mysql.com/doc/refman/5.7/en/replication-options-binary-log.html#sysvar_log_builtin_as_identified_by_password)|

## binlog_gtid_simple_recovery
|name|value|
|----|-----|
|Name|`binlog_gtid_simple_recovery`|
|Command line|`--binlog-gtid-simple-recovery[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_binlog_gtid_simple_recovery](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_binlog_gtid_simple_recovery)|

## enforce_gtid_consistency
|name|value|
|----|-----|
|Name|`enforce_gtid_consistency`|
|Command line|`--enforce-gtid-consistency[=value]`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `ON`, `WARN`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_enforce_gtid_consistency](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_enforce_gtid_consistency)|

## gtid_executed
|name|value|
|----|-----|
|Name|`gtid_executed`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_gtid_executed](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_gtid_executed)|
|dev.mysql.com|[sysvar_gtid_executed](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_gtid_executed)|
|dev.mysql.com|[sysvar_gtid_executed](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_gtid_executed)|

## gtid_executed_compression_period
|name|value|
|----|-----|
|Name|`gtid_executed_compression_period`|
|Command line|`--gtid-executed-compression-period=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_gtid_executed_compression_period](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_gtid_executed_compression_period)|

## gtid_mode
|name|value|
|----|-----|
|Name|`gtid_mode`|
|Command line|`--gtid-mode=MODE`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `OFF_PERMISSIVE`, `ON_PERMISSIVE`, `ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_gtid_mode](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_gtid_mode)|

## gtid_next
|name|value|
|----|-----|
|Name|`gtid_next`|
|Type of variable|`enumeration`|
|Scope|`session`|
|Default value|`AUTOMATIC`|
|Dynamic|`true`|
|Valid value(s)|`AUTOMATIC`, `ANONYMOUS`, `<UUID>:<NUMBER>`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_gtid_next](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_gtid_next)|

## gtid_owned
|name|value|
|----|-----|
|Name|`gtid_owned`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_gtid_owned](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_gtid_owned)|

## gtid_purged
|name|value|
|----|-----|
|Name|`gtid_purged`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_gtid_purged](https://dev.mysql.com/doc/refman/8.0/en/replication-options-gtids.html#sysvar_gtid_purged)|

## master_info_file
|name|value|
|----|-----|
|Name|`master_info_file`|
|Command line|`--master-info-file=file_name`|
|Type of variable|`file name`|
|Default value|`master.info`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_master-info-file](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_master-info-file)|

## master_retry_count
|name|value|
|----|-----|
|Name|`master_retry_count`|
|Command line|`--master-retry-count=#`|
|Type of variable|`integer`|
|Default value|`86400`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_master-retry-count](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_master-retry-count)|

## max_relay_log_size
|name|value|
|----|-----|
|Name|`max_relay_log_size`|
|Command line|`--max-relay-log-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_max-relay-log-size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_max-relay-log-size)|

## relay_log_purge
|name|value|
|----|-----|
|Name|`relay_log_purge`|
|Command line|`--relay-log-purge[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_relay-log-purge](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_relay-log-purge)|

## relay_log_space_limit
|name|value|
|----|-----|
|Name|`relay_log_space_limit`|
|Command line|`--relay-log-space-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`false`|
|Range|from: `0` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_relay-log-space-limit](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_relay-log-space-limit)|

## replicate_do_db
|name|value|
|----|-----|
|Name|`replicate_do_db`|
|Command line|`--replicate-do-db=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-do-db](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-do-db)|

## replicate_ignore_db
|name|value|
|----|-----|
|Name|`replicate_ignore_db`|
|Command line|`--replicate-ignore-db=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-ignore-db](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-ignore-db)|

## replicate_do_table
|name|value|
|----|-----|
|Name|`replicate_do_table`|
|Command line|`--replicate-do-table=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-do-table](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-do-table)|

## replicate_ignore_table
|name|value|
|----|-----|
|Name|`replicate_ignore_table`|
|Command line|`--replicate-ignore-table=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-ignore-table](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-ignore-table)|

## replicate_rewrite_db
|name|value|
|----|-----|
|Name|`replicate_rewrite_db`|
|Command line|`--replicate-rewrite-db=old_name->new_name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-rewrite-db](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-rewrite-db)|

## replicate_same_server_id
|name|value|
|----|-----|
|Name|`replicate_same_server_id`|
|Command line|`--replicate-same-server-id[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-same-server-id](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-same-server-id)|

## replicate_wild_do_table
|name|value|
|----|-----|
|Name|`replicate_wild_do_table`|
|Command line|`--replicate-wild-do-table=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-wild-do-table](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-wild-do-table)|

## replicate_wild_ignore_table
|name|value|
|----|-----|
|Name|`replicate_wild_ignore_table`|
|Command line|`--replicate-wild-ignore-table=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_replicate-wild-ignore-table](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_replicate-wild-ignore-table)|

## skip_replica_start
|name|value|
|----|-----|
|Name|`skip_replica_start`|
|Command line|`--skip-replica-start[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-replica-start](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_skip-replica-start)|

## skip_slave_start
|name|value|
|----|-----|
|Name|`skip_slave_start`|
|Command line|`--skip-slave-start[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-slave-start](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_skip-slave-start)|

## slave_skip_errors
|name|value|
|----|-----|
|Name|`slave_skip_errors`|
|Command line|`--slave-skip-errors=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|
|Valid value(s)|`OFF`, `[list of error codes]`, `all`, `ddl_exist_errors`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_slave-skip-errors](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_slave-skip-errors)|

## slave_sql_verify_checksum
|name|value|
|----|-----|
|Name|`slave_sql_verify_checksum`|
|Command line|`--slave-sql-verify-checksum[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_slave-sql-verify-checksum](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_slave-sql-verify-checksum)|

## abort_slave_event_count
|name|value|
|----|-----|
|Name|`abort_slave_event_count`|
|Command line|`--abort-slave-event-count=#`|
|Type of variable|`integer`|
|Default value|`0`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_abort-slave-event-count](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_abort-slave-event-count)|

## disconnect_slave_event_count
|name|value|
|----|-----|
|Name|`disconnect_slave_event_count`|
|Command line|`--disconnect-slave-event-count=#`|
|Type of variable|`integer`|
|Default value|`0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_disconnect-slave-event-count](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#option_mysqld_disconnect-slave-event-count)|

## init_replica
|name|value|
|----|-----|
|Name|`init_replica`|
|Command line|`--init-replica=name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_init_replica](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_init_replica)|

## init_slave
|name|value|
|----|-----|
|Name|`init_slave`|
|Command line|`--init-slave=name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_init_slave](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_init_slave)|

## log_slow_replica_statements
|name|value|
|----|-----|
|Name|`log_slow_replica_statements`|
|Command line|`--log-slow-replica-statements[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_slow_replica_statements](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_log_slow_replica_statements)|

## log_slow_slave_statements
|name|value|
|----|-----|
|Name|`log_slow_slave_statements`|
|Command line|`--log-slow-slave-statements[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_slow_slave_statements](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_log_slow_slave_statements)|

## master_info_repository
|name|value|
|----|-----|
|Name|`master_info_repository`|
|Command line|`--master-info-repository={FILE|TABLE}`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`TABLE`|
|Dynamic|`true`|
|Valid value(s)|`FILE`, `TABLE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_master_info_repository](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_master_info_repository)|

## relay_log
|name|value|
|----|-----|
|Name|`relay_log`|
|Command line|`--relay-log=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_relay_log](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_relay_log)|

## relay_log_basename
|name|value|
|----|-----|
|Name|`relay_log_basename`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`datadir + '/' + hostname + '-relay-bin'`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_relay_log_basename](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_relay_log_basename)|

## relay_log_index
|name|value|
|----|-----|
|Name|`relay_log_index`|
|Command line|`--relay-log-index=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`*host_name*-relay-bin.index`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_relay_log_index](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_relay_log_index)|

## relay_log_info_file
|name|value|
|----|-----|
|Name|`relay_log_info_file`|
|Command line|`--relay-log-info-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`relay-log.info`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_relay_log_info_file](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_relay_log_info_file)|

## relay_log_info_repository
|name|value|
|----|-----|
|Name|`relay_log_info_repository`|
|Command line|`--relay-log-info-repository=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`TABLE`|
|Dynamic|`true`|
|Valid value(s)|`FILE`, `TABLE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_relay_log_info_repository](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_relay_log_info_repository)|

## relay_log_recovery
|name|value|
|----|-----|
|Name|`relay_log_recovery`|
|Command line|`--relay-log-recovery[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_relay_log_recovery](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_relay_log_recovery)|

## replica_checkpoint_group
|name|value|
|----|-----|
|Name|`replica_checkpoint_group`|
|Command line|`--replica-checkpoint-group=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`512`|
|Dynamic|`true`|
|Range|from: `32` to: `524280`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_checkpoint_group](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_checkpoint_group)|

## replica_checkpoint_period
|name|value|
|----|-----|
|Name|`replica_checkpoint_period`|
|Command line|`--replica-checkpoint-period=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`300`|
|Dynamic|`true`|
|Range|from: `1` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_checkpoint_period](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_checkpoint_period)|

## replica_compressed_protocol
|name|value|
|----|-----|
|Name|`replica_compressed_protocol`|
|Command line|`--replica-compressed-protocol[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_compressed_protocol](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_compressed_protocol)|

## replica_exec_mode
|name|value|
|----|-----|
|Name|`replica_exec_mode`|
|Command line|`--replica-exec-mode=mode`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`IDEMPOTENT (NDB)STRICT (Other)`|
|Dynamic|`true`|
|Valid value(s)|`STRICT`, `IDEMPOTENT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_exec_mode](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_exec_mode)|

## replica_load_tmpdir
|name|value|
|----|-----|
|Name|`replica_load_tmpdir`|
|Command line|`--replica-load-tmpdir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`Value of --tmpdir`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_load_tmpdir](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_load_tmpdir)|

## replica_max_allowed_packet
|name|value|
|----|-----|
|Name|`replica_max_allowed_packet`|
|Command line|`--replica-max-allowed-packet=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1073741824`|
|Dynamic|`true`|
|Range|from: `1024` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_max_allowed_packet](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_max_allowed_packet)|

## replica_net_timeout
|name|value|
|----|-----|
|Name|`replica_net_timeout`|
|Command line|`--replica-net-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`60`|
|Dynamic|`true`|
|Range|from: `1` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_net_timeout](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_net_timeout)|

## replica_parallel_type
|name|value|
|----|-----|
|Name|`replica_parallel_type`|
|Command line|`--replica-parallel-type=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`LOGICAL_CLOCK`|
|Dynamic|`true`|
|Valid value(s)|`DATABASE`, `LOGICAL_CLOCK`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_parallel_type](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_parallel_type)|

## replica_parallel_workers
|name|value|
|----|-----|
|Name|`replica_parallel_workers`|
|Command line|`--replica-parallel-workers=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4`|
|Dynamic|`true`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_parallel_workers](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_parallel_workers)|

## replica_pending_jobs_size_max
|name|value|
|----|-----|
|Name|`replica_pending_jobs_size_max`|
|Command line|`--replica-pending-jobs-size-max=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`128M`|
|Dynamic|`true`|
|Range|from: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_pending_jobs_size_max](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_pending_jobs_size_max)|

## replica_preserve_commit_order
|name|value|
|----|-----|
|Name|`replica_preserve_commit_order`|
|Command line|`--replica-preserve-commit-order[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_preserve_commit_order](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_preserve_commit_order)|

## replica_sql_verify_checksum
|name|value|
|----|-----|
|Name|`replica_sql_verify_checksum`|
|Command line|`--replica-sql-verify-checksum[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_sql_verify_checksum](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_sql_verify_checksum)|

## replica_transaction_retries
|name|value|
|----|-----|
|Name|`replica_transaction_retries`|
|Command line|`--replica-transaction-retries=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_transaction_retries](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_transaction_retries)|

## replica_type_conversions
|name|value|
|----|-----|
|Name|`replica_type_conversions`|
|Command line|`--replica-type-conversions=set`|
|Type of variable|`set`|
|Scope|`global`|
|Default value|``|
|Dynamic|`true`|
|Valid value(s)|`ALL_LOSSY`, `ALL_NON_LOSSY`, `ALL_SIGNED`, `ALL_UNSIGNED`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_type_conversions](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_type_conversions)|

## replication_optimize_for_static_plugin_config
|name|value|
|----|-----|
|Name|`replication_optimize_for_static_plugin_config`|
|Command line|`--replication-optimize-for-static-plugin-config[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replication_optimize_for_static_plugin_config](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replication_optimize_for_static_plugin_config)|

## replication_sender_observe_commit_only
|name|value|
|----|-----|
|Name|`replication_sender_observe_commit_only`|
|Command line|`--replication-sender-observe-commit-only[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replication_sender_observe_commit_only](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replication_sender_observe_commit_only)|

## report_host
|name|value|
|----|-----|
|Name|`report_host`|
|Command line|`--report-host=host_name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_report_host](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_report_host)|

## report_password
|name|value|
|----|-----|
|Name|`report_password`|
|Command line|`--report-password=name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_report_password](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_report_password)|

## report_port
|name|value|
|----|-----|
|Name|`report_port`|
|Command line|`--report-port=port_num`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`[slave_port]`|
|Dynamic|`false`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_report_port](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_report_port)|

## report_user
|name|value|
|----|-----|
|Name|`report_user`|
|Command line|`--report-user=name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_report_user](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_report_user)|

## rpl_read_size
|name|value|
|----|-----|
|Name|`rpl_read_size`|
|Command line|`--rpl-read-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8192`|
|Dynamic|`true`|
|Range|from: `8192` to: `4294959104`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_read_size](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_rpl_read_size)|

## rpl_semi_sync_replica_enabled
|name|value|
|----|-----|
|Name|`rpl_semi_sync_replica_enabled`|
|Command line|`--rpl-semi-sync-replica-enabled[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_replica_enabled](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_rpl_semi_sync_replica_enabled)|

## rpl_semi_sync_replica_trace_level
|name|value|
|----|-----|
|Name|`rpl_semi_sync_replica_trace_level`|
|Command line|`--rpl-semi-sync-replica-trace-level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_replica_trace_level](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_rpl_semi_sync_replica_trace_level)|

## rpl_semi_sync_slave_enabled
|name|value|
|----|-----|
|Name|`rpl_semi_sync_slave_enabled`|
|Command line|`--rpl-semi-sync-slave-enabled[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_slave_enabled](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_rpl_semi_sync_slave_enabled)|

## rpl_semi_sync_slave_trace_level
|name|value|
|----|-----|
|Name|`rpl_semi_sync_slave_trace_level`|
|Command line|`--rpl-semi-sync-slave-trace-level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_slave_trace_level](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_rpl_semi_sync_slave_trace_level)|

## rpl_stop_replica_timeout
|name|value|
|----|-----|
|Name|`rpl_stop_replica_timeout`|
|Command line|`--rpl-stop-replica-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`31536000`|
|Dynamic|`true`|
|Range|from: `2` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_stop_replica_timeout](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_rpl_stop_replica_timeout)|

## rpl_stop_slave_timeout
|name|value|
|----|-----|
|Name|`rpl_stop_slave_timeout`|
|Command line|`--rpl-stop-slave-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`31536000`|
|Dynamic|`true`|
|Range|from: `2` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_stop_slave_timeout](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_rpl_stop_slave_timeout)|

## slave_checkpoint_group
|name|value|
|----|-----|
|Name|`slave_checkpoint_group`|
|Command line|`--slave-checkpoint-group=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`512`|
|Dynamic|`true`|
|Range|from: `32` to: `524280`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_checkpoint_group](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_checkpoint_group)|

## slave_checkpoint_period
|name|value|
|----|-----|
|Name|`slave_checkpoint_period`|
|Command line|`--slave-checkpoint-period=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`300`|
|Dynamic|`true`|
|Range|from: `1` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_checkpoint_period](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_checkpoint_period)|

## slave_compressed_protocol
|name|value|
|----|-----|
|Name|`slave_compressed_protocol`|
|Command line|`--slave-compressed-protocol[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_compressed_protocol](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_compressed_protocol)|

## slave_exec_mode
|name|value|
|----|-----|
|Name|`slave_exec_mode`|
|Command line|`--slave-exec-mode=mode`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`IDEMPOTENT (NDB)STRICT (Other)`|
|Dynamic|`true`|
|Valid value(s)|`STRICT`, `IDEMPOTENT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_exec_mode](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_exec_mode)|

## slave_load_tmpdir
|name|value|
|----|-----|
|Name|`slave_load_tmpdir`|
|Command line|`--slave-load-tmpdir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`Value of --tmpdir`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_load_tmpdir](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_load_tmpdir)|

## slave_max_allowed_packet
|name|value|
|----|-----|
|Name|`slave_max_allowed_packet`|
|Command line|`--slave-max-allowed-packet=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1073741824`|
|Dynamic|`true`|
|Range|from: `1024` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_max_allowed_packet](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_max_allowed_packet)|

## slave_net_timeout
|name|value|
|----|-----|
|Name|`slave_net_timeout`|
|Command line|`--slave-net-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`60`|
|Dynamic|`true`|
|Range|from: `1` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_net_timeout](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_net_timeout)|

## slave_parallel_type
|name|value|
|----|-----|
|Name|`slave_parallel_type`|
|Command line|`--slave-parallel-type=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`LOGICAL_CLOCK`|
|Dynamic|`true`|
|Valid value(s)|`DATABASE`, `LOGICAL_CLOCK`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_parallel_type](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_parallel_type)|

## slave_parallel_workers
|name|value|
|----|-----|
|Name|`slave_parallel_workers`|
|Command line|`--slave-parallel-workers=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4`|
|Dynamic|`true`|
|Range|from: `0` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_parallel_workers](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_parallel_workers)|

## slave_pending_jobs_size_max
|name|value|
|----|-----|
|Name|`slave_pending_jobs_size_max`|
|Command line|`--slave-pending-jobs-size-max=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`128M`|
|Dynamic|`true`|
|Range|from: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_pending_jobs_size_max](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_pending_jobs_size_max)|

## slave_preserve_commit_order
|name|value|
|----|-----|
|Name|`slave_preserve_commit_order`|
|Command line|`--slave-preserve-commit-order[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_preserve_commit_order](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_preserve_commit_order)|

## slave_rows_search_algorithms
|name|value|
|----|-----|
|Name|`slave_rows_search_algorithms`|
|Command line|`--slave-rows-search-algorithms=value`|
|Type of variable|`set`|
|Scope|`global`|
|Default value|`INDEX_SCAN,HASH_SCAN`|
|Dynamic|`true`|
|Valid value(s)|`TABLE_SCAN,INDEX_SCAN`, `INDEX_SCAN,HASH_SCAN`, `TABLE_SCAN,HASH_SCAN`, `TABLE_SCAN,INDEX_SCAN,HASH_SCAN`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_rows_search_algorithms](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_rows_search_algorithms)|

## replica_skip_errors
|name|value|
|----|-----|
|Name|`replica_skip_errors`|
|Command line|`--replica-skip-errors=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|
|Valid value(s)|`OFF`, `[list of error codes]`, `all`, `ddl_exist_errors`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_replica_skip_errors](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_replica_skip_errors)|

## slave_transaction_retries
|name|value|
|----|-----|
|Name|`slave_transaction_retries`|
|Command line|`--slave-transaction-retries=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_transaction_retries](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_transaction_retries)|

## slave_type_conversions
|name|value|
|----|-----|
|Name|`slave_type_conversions`|
|Command line|`--slave-type-conversions=set`|
|Type of variable|`set`|
|Scope|`global`|
|Default value|``|
|Dynamic|`true`|
|Valid value(s)|`ALL_LOSSY`, `ALL_NON_LOSSY`, `ALL_SIGNED`, `ALL_UNSIGNED`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slave_type_conversions](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_slave_type_conversions)|

## sql_replica_skip_counter
|name|value|
|----|-----|
|Name|`sql_replica_skip_counter`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_replica_skip_counter](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_sql_replica_skip_counter)|

## sql_slave_skip_counter
|name|value|
|----|-----|
|Name|`sql_slave_skip_counter`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_slave_skip_counter](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_sql_slave_skip_counter)|

## sync_master_info
|name|value|
|----|-----|
|Name|`sync_master_info`|
|Command line|`--sync-master-info=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sync_master_info](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_sync_master_info)|

## sync_relay_log
|name|value|
|----|-----|
|Name|`sync_relay_log`|
|Command line|`--sync-relay-log=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sync_relay_log](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_sync_relay_log)|

## sync_relay_log_info
|name|value|
|----|-----|
|Name|`sync_relay_log_info`|
|Command line|`--sync-relay-log-info=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sync_relay_log_info](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_sync_relay_log_info)|

## sync_source_info
|name|value|
|----|-----|
|Name|`sync_source_info`|
|Command line|`--sync-source-info=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sync_source_info](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_sync_source_info)|

## terminology_use_previous
|name|value|
|----|-----|
|Name|`terminology_use_previous`|
|Command line|`--terminology-use-previous=#`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`NONE`|
|Dynamic|`true`|
|Valid value(s)|`NONE`, `BEFORE_8_0_26`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_terminology_use_previous](https://dev.mysql.com/doc/refman/8.0/en/replication-options-replica.html#sysvar_terminology_use_previous)|

## show_replica_auth_info
|name|value|
|----|-----|
|Name|`show_replica_auth_info`|
|Command line|`--show-replica-auth-info[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_show-replica-auth-info](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#option_mysqld_show-replica-auth-info)|

## show_slave_auth_info
|name|value|
|----|-----|
|Name|`show_slave_auth_info`|
|Command line|`--show-slave-auth-info[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_show-slave-auth-info](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#option_mysqld_show-slave-auth-info)|

## auto_increment_increment
|name|value|
|----|-----|
|Name|`auto_increment_increment`|
|Command line|`--auto-increment-increment=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_auto_increment_increment](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_auto_increment_increment)|

## auto_increment_offset
|name|value|
|----|-----|
|Name|`auto_increment_offset`|
|Command line|`--auto-increment-offset=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_auto_increment_offset](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_auto_increment_offset)|

## immediate_server_version
|name|value|
|----|-----|
|Name|`immediate_server_version`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`999999`|
|Dynamic|`true`|
|Range|from: `0` to: `999999`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_immediate_server_version](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_immediate_server_version)|

## original_server_version
|name|value|
|----|-----|
|Name|`original_server_version`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_original_server_version](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_original_server_version)|
|dev.mysql.com|[sysvar_original_server_version](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_original_server_version)|

## rpl_semi_sync_master_enabled
|name|value|
|----|-----|
|Name|`rpl_semi_sync_master_enabled`|
|Command line|`--rpl-semi-sync-master-enabled[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_master_enabled](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_master_enabled)|

## rpl_semi_sync_master_timeout
|name|value|
|----|-----|
|Name|`rpl_semi_sync_master_timeout`|
|Command line|`--rpl-semi-sync-master-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_master_timeout](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_master_timeout)|

## rpl_semi_sync_master_trace_level
|name|value|
|----|-----|
|Name|`rpl_semi_sync_master_trace_level`|
|Command line|`--rpl-semi-sync-master-trace-level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_master_trace_level](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_master_trace_level)|

## rpl_semi_sync_master_wait_for_slave_count
|name|value|
|----|-----|
|Name|`rpl_semi_sync_master_wait_for_slave_count`|
|Command line|`--rpl-semi-sync-master-wait-for-slave-count=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_master_wait_for_slave_count](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_master_wait_for_slave_count)|

## rpl_semi_sync_master_wait_no_slave
|name|value|
|----|-----|
|Name|`rpl_semi_sync_master_wait_no_slave`|
|Command line|`--rpl-semi-sync-master-wait-no-slave[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_master_wait_no_slave](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_master_wait_no_slave)|

## rpl_semi_sync_master_wait_point
|name|value|
|----|-----|
|Name|`rpl_semi_sync_master_wait_point`|
|Command line|`--rpl-semi-sync-master-wait-point=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`AFTER_SYNC`|
|Dynamic|`true`|
|Valid value(s)|`AFTER_SYNC`, `AFTER_COMMIT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_master_wait_point](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_master_wait_point)|

## rpl_semi_sync_source_enabled
|name|value|
|----|-----|
|Name|`rpl_semi_sync_source_enabled`|
|Command line|`--rpl-semi-sync-source-enabled[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_source_enabled](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_source_enabled)|

## rpl_semi_sync_source_timeout
|name|value|
|----|-----|
|Name|`rpl_semi_sync_source_timeout`|
|Command line|`--rpl-semi-sync-source-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_source_timeout](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_source_timeout)|

## rpl_semi_sync_source_trace_level
|name|value|
|----|-----|
|Name|`rpl_semi_sync_source_trace_level`|
|Command line|`--rpl-semi-sync-source-trace-level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_source_trace_level](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_source_trace_level)|

## rpl_semi_sync_source_wait_for_replica_count
|name|value|
|----|-----|
|Name|`rpl_semi_sync_source_wait_for_replica_count`|
|Command line|`--rpl-semi-sync-source-wait-for-replica-count=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_source_wait_for_replica_count](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_source_wait_for_replica_count)|

## rpl_semi_sync_source_wait_no_replica
|name|value|
|----|-----|
|Name|`rpl_semi_sync_source_wait_no_replica`|
|Command line|`--rpl-semi-sync-source-wait-no-replica[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_source_wait_no_replica](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_source_wait_no_replica)|

## rpl_semi_sync_source_wait_point
|name|value|
|----|-----|
|Name|`rpl_semi_sync_source_wait_point`|
|Command line|`--rpl-semi-sync-source-wait-point=value`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`AFTER_SYNC`|
|Dynamic|`true`|
|Valid value(s)|`AFTER_SYNC`, `AFTER_COMMIT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rpl_semi_sync_source_wait_point](https://dev.mysql.com/doc/refman/8.0/en/replication-options-source.html#sysvar_rpl_semi_sync_source_wait_point)|

## server_id
|name|value|
|----|-----|
|Name|`server_id`|
|Command line|`--server-id=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_server_id](https://dev.mysql.com/doc/refman/8.0/en/replication-options.html#sysvar_server_id)|

## server_uuid
|name|value|
|----|-----|
|Name|`server_uuid`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_server_uuid](https://dev.mysql.com/doc/refman/8.0/en/replication-options.html#sysvar_server_uuid)|

## help
|name|value|
|----|-----|
|Name|`help`|
|Command line|`--help`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_help](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_help)|

## admin_ssl
|name|value|
|----|-----|
|Name|`admin_ssl`|
|Command line|`--admin-ssl[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_admin-ssl](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_admin-ssl)|

## allow_suspicious_udfs
|name|value|
|----|-----|
|Name|`allow_suspicious_udfs`|
|Command line|`--allow-suspicious-udfs[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_allow-suspicious-udfs](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_allow-suspicious-udfs)|

## ansi
|name|value|
|----|-----|
|Name|`ansi`|
|Command line|`--ansi`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ansi](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_ansi)|

## basedir
|name|value|
|----|-----|
|Name|`basedir`|
|Command line|`--basedir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`c1figurati1-dependent default`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_basedir](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#sysvar_basedir)|
|dev.mysql.com|[sysvar_basedir](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_basedir)|
|dev.mysql.com|[sysvar_basedir](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_basedir)|

## character_set_client_handshake
|name|value|
|----|-----|
|Name|`character_set_client_handshake`|
|Command line|`--character-set-client-handshake[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_character-set-client-handshake](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_character-set-client-handshake)|

## check_table_functions
|name|value|
|----|-----|
|Name|`check_table_functions`|
|Command line|`--check-table-functions=value`|
|Type of variable|`enumeration`|
|Default value|`ABORT`|
|Valid value(s)|`WARN`, `ABORT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_check-table-functions](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_check-table-functions)|

## chroot
|name|value|
|----|-----|
|Name|`chroot`|
|Command line|`--chroot=dir_name`|
|Type of variable|`directory name`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_chroot](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_chroot)|

## console
|name|value|
|----|-----|
|Name|`console`|
|Command line|`--console`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_console](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_console)|

## core_file
|name|value|
|----|-----|
|Name|`core_file`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_core-file](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_core-file)|
|dev.mysql.com|[sysvar_core_file](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_core_file)|
|dev.mysql.com|[sysvar_core_file](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_core_file)|

## daemonize
|name|value|
|----|-----|
|Name|`daemonize`|
|Command line|`--daemonize[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_daemonize](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_daemonize)|

## datadir
|name|value|
|----|-----|
|Name|`datadir`|
|Command line|`--datadir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_datadir](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#sysvar_datadir)|
|dev.mysql.com|[sysvar_datadir](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_datadir)|
|dev.mysql.com|[sysvar_datadir](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_datadir)|

## debug
|name|value|
|----|-----|
|Name|`debug`|
|Command line|`--debug[=debug_options]`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_debug](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_debug)|
|dev.mysql.com|[sysvar_debug](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_debug)|
|dev.mysql.com|[sysvar_debug](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_debug)|

## debug_sync_timeout
|name|value|
|----|-----|
|Name|`debug_sync_timeout`|
|Command line|`--debug-sync-timeout[=#]`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_debug-sync-timeout](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_debug-sync-timeout)|

## default_time_zone
|name|value|
|----|-----|
|Name|`default_time_zone`|
|Command line|`--default-time-zone=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_default-time-zone](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_default-time-zone)|

## early_plugin_load
|name|value|
|----|-----|
|Name|`early_plugin_load`|
|Command line|`--early-plugin-load=plugin_list`|
|Type of variable|`string`|
|Default value|`empty string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_early-plugin-load](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_early-plugin-load)|

## exit_info
|name|value|
|----|-----|
|Name|`exit_info`|
|Command line|`--exit-info[=flags]`|
|Type of variable|`integer`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_exit-info](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_exit-info)|

## external_locking
|name|value|
|----|-----|
|Name|`external_locking`|
|Command line|`--external-locking[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_external-locking](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_external-locking)|

## flush
|name|value|
|----|-----|
|Name|`flush`|
|Command line|`--flush[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_flush](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_flush)|
|dev.mysql.com|[sysvar_flush](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_flush)|
|dev.mysql.com|[sysvar_flush](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_flush)|

## gdb
|name|value|
|----|-----|
|Name|`gdb`|
|Command line|`--gdb[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_gdb](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_gdb)|

## initialize
|name|value|
|----|-----|
|Name|`initialize`|
|Command line|`--initialize[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_initialize](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_initialize)|

## initialize_insecure
|name|value|
|----|-----|
|Name|`initialize_insecure`|
|Command line|`--initialize-insecure[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_initialize-insecure](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_initialize-insecure)|

## install
|name|value|
|----|-----|
|Name|`install`|
|Command line|`--install [service_name]`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_install](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_install)|

## install_manual
|name|value|
|----|-----|
|Name|`install_manual`|
|Command line|`--install-manual [service_name]`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_install-manual](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_install-manual)|

## language
|name|value|
|----|-----|
|Name|`language`|
|Command line|`--language=name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`/usr/local/mysql/share/mysql/english/`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_language](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_language)|

## large_pages
|name|value|
|----|-----|
|Name|`large_pages`|
|Command line|`--large-pages[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_large-pages](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_large-pages)|
|dev.mysql.com|[sysvar_large_pages](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_large_pages)|
|dev.mysql.com|[sysvar_large_pages](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_large_pages)|

## lc_messages
|name|value|
|----|-----|
|Name|`lc_messages`|
|Command line|`--lc-messages=name`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`en_US`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_lc-messages](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_lc-messages)|
|dev.mysql.com|[sysvar_lc_messages](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_lc_messages)|
|dev.mysql.com|[sysvar_lc_messages](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_lc_messages)|

## lc_messages_dir
|name|value|
|----|-----|
|Name|`lc_messages_dir`|
|Command line|`--lc-messages-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_lc-messages-dir](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_lc-messages-dir)|
|dev.mysql.com|[sysvar_lc_messages_dir](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_lc_messages_dir)|
|dev.mysql.com|[sysvar_lc_messages_dir](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_lc_messages_dir)|

## local_service
|name|value|
|----|-----|
|Name|`local_service`|
|Command line|`--local-service`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_local-service](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_local-service)|

## log_error
|name|value|
|----|-----|
|Name|`log_error`|
|Command line|`--log-error[=file_name]`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-error](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_log-error)|
|dev.mysql.com|[sysvar_log_error](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_error)|
|dev.mysql.com|[sysvar_log_error](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_error)|

## log_isam
|name|value|
|----|-----|
|Name|`log_isam`|
|Command line|`--log-isam[=file_name]`|
|Type of variable|`file name`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-isam](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_log-isam)|

## log_raw
|name|value|
|----|-----|
|Name|`log_raw`|
|Command line|`--log-raw[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-raw](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_log-raw)|
|dev.mysql.com|[sysvar_log_raw](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_raw)|

## log_short_format
|name|value|
|----|-----|
|Name|`log_short_format`|
|Command line|`--log-short-format[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-short-format](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_log-short-format)|

## log_tc
|name|value|
|----|-----|
|Name|`log_tc`|
|Command line|`--log-tc=file_name`|
|Type of variable|`file name`|
|Default value|`tc.log`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-tc](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_log-tc)|

## log_tc_size
|name|value|
|----|-----|
|Name|`log_tc_size`|
|Command line|`--log-tc-size=#`|
|Type of variable|`integer`|
|Default value|`6 * page size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_log-tc-size](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_log-tc-size)|

## memlock
|name|value|
|----|-----|
|Name|`memlock`|
|Command line|`--memlock[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_memlock](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_memlock)|

## myisam_block_size
|name|value|
|----|-----|
|Name|`myisam_block_size`|
|Command line|`--myisam-block-size=#`|
|Type of variable|`integer`|
|Default value|`1024`|
|Range|from: `1024` to: `16384`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_myisam-block-size](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_myisam-block-size)|

## no_dd_upgrade
|name|value|
|----|-----|
|Name|`no_dd_upgrade`|
|Command line|`--no-dd-upgrade[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_no-dd-upgrade](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_no-dd-upgrade)|

## no_monitor
|name|value|
|----|-----|
|Name|`no_monitor`|
|Command line|`--no-monitor[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_no-monitor](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_no-monitor)|

## old_style_user_limits
|name|value|
|----|-----|
|Name|`old_style_user_limits`|
|Command line|`--old-style-user-limits[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_old-style-user-limits](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_old-style-user-limits)|

## plugin_load
|name|value|
|----|-----|
|Name|`plugin_load`|
|Command line|`--plugin-load=plugin_list`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_plugin-load](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_plugin-load)|

## plugin_load_add
|name|value|
|----|-----|
|Name|`plugin_load_add`|
|Command line|`--plugin-load-add=plugin_list`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_plugin-load-add](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_plugin-load-add)|

## port
|name|value|
|----|-----|
|Name|`port`|
|Command line|`--port=port_num`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`3306`|
|Dynamic|`false`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_port](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_port)|
|dev.mysql.com|[sysvar_port](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_port)|
|dev.mysql.com|[sysvar_port](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_port)|

## port_open_timeout
|name|value|
|----|-----|
|Name|`port_open_timeout`|
|Command line|`--port-open-timeout=#`|
|Type of variable|`integer`|
|Default value|`0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_port-open-timeout](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_port-open-timeout)|

## remove
|name|value|
|----|-----|
|Name|`remove`|
|Command line|`--remove [service_name]`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_remove](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_remove)|

## safe_user_create
|name|value|
|----|-----|
|Name|`safe_user_create`|
|Command line|`--safe-user-create[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_safe-user-create](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_safe-user-create)|

## skip_grant_tables
|name|value|
|----|-----|
|Name|`skip_grant_tables`|
|Command line|`--skip-grant-tables[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-grant-tables](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_skip-grant-tables)|

## skip_host_cache
|name|value|
|----|-----|
|Name|`skip_host_cache`|
|Command line|`--skip-host-cache`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-host-cache](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_skip-host-cache)|

## skip_new
|name|value|
|----|-----|
|Name|`skip_new`|
|Command line|`--skip-new`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-new](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_skip-new)|

## skip_show_database
|name|value|
|----|-----|
|Name|`skip_show_database`|
|Command line|`--skip-show-database`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-show-database](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_skip-show-database)|
|dev.mysql.com|[sysvar_skip_show_database](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_skip_show_database)|
|dev.mysql.com|[sysvar_skip_show_database](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_skip_show_database)|

## skip_stack_trace
|name|value|
|----|-----|
|Name|`skip_stack_trace`|
|Command line|`--skip-stack-trace`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_skip-stack-trace](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_skip-stack-trace)|

## slow_start_timeout
|name|value|
|----|-----|
|Name|`slow_start_timeout`|
|Command line|`--slow-start-timeout=#`|
|Type of variable|`integer`|
|Default value|`15000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_slow-start-timeout](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_slow-start-timeout)|

## socket
|name|value|
|----|-----|
|Name|`socket`|
|Command line|`--socket={file_name|pipe_name}`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_socket](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_socket)|
|dev.mysql.com|[sysvar_socket](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_socket)|
|dev.mysql.com|[sysvar_socket](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_socket)|

## sql_mode
|name|value|
|----|-----|
|Name|`sql_mode`|
|Command line|`--sql-mode=name`|
|Type of variable|`set`|
|Scope|`global`, `session`|
|Default value|`1LY_FULL_GROUP_BY STRICT_TRANS_TABLES NO_ZERO_IN_DATE NO_ZERO_DATE ERROR_FOR_DIVISI1_BY_ZERO NO_AUTO_CREATE_USER NO_ENGINE_SUBSTITUTI1`|
|Dynamic|`true`|
|Valid value(s)|`ALLOW_INVALID_DATES`, `ANSI_QUOTES`, `ERROR_FOR_DIVISION_BY_ZERO`, `HIGH_NOT_PRECEDENCE`, `IGNORE_SPACE`, `NO_AUTO_VALUE_ON_ZERO`, `NO_BACKSLASH_ESCAPES`, `NO_DIR_IN_CREATE`, `NO_ENGINE_SUBSTITUTION`, `NO_UNSIGNED_SUBTRACTION`, `NO_ZERO_DATE`, `NO_ZERO_IN_DATE`, `ONLY_FULL_GROUP_BY`, `PAD_CHAR_TO_FULL_LENGTH`, `PIPES_AS_CONCAT`, `REAL_AS_FLOAT`, `STRICT_ALL_TABLES`, `STRICT_TRANS_TABLES`, `TIME_TRUNCATE_FRACTIONAL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_sql-mode](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_sql-mode)|
|dev.mysql.com|[sysvar_sql_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_mode)|
|dev.mysql.com|[sysvar_sql_mode](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_mode)|

## ssl
|name|value|
|----|-----|
|Name|`ssl`|
|Command line|`--ssl[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_ssl](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_ssl)|

## standalone
|name|value|
|----|-----|
|Name|`standalone`|
|Command line|`--standalone`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_standalone](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_standalone)|

## super_large_pages
|name|value|
|----|-----|
|Name|`super_large_pages`|
|Command line|`--super-large-pages[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_super-large-pages](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_super-large-pages)|

## symbolic_links
|name|value|
|----|-----|
|Name|`symbolic_links`|
|Command line|`--symbolic-links[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_symbolic-links](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_symbolic-links)|

## sysdate_is_now
|name|value|
|----|-----|
|Name|`sysdate_is_now`|
|Command line|`--sysdate-is-now[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_sysdate-is-now](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_sysdate-is-now)|

## tc_heuristic_recover
|name|value|
|----|-----|
|Name|`tc_heuristic_recover`|
|Command line|`--tc-heuristic-recover=name`|
|Type of variable|`enumeration`|
|Default value|`OFF`|
|Valid value(s)|`OFF`, `COMMIT`, `ROLLBACK`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_tc-heuristic-recover](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_tc-heuristic-recover)|

## transaction_isolation
|name|value|
|----|-----|
|Name|`transaction_isolation`|
|Command line|`--transaction-isolation=name`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`REPEATABLE-READ`|
|Dynamic|`true`|
|Valid value(s)|`READ-UNCOMMITTED`, `READ-COMMITTED`, `REPEATABLE-READ`, `SERIALIZABLE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_transaction-isolation](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_transaction-isolation)|
|dev.mysql.com|[sysvar_transaction_isolation](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_transaction_isolation)|
|dev.mysql.com|[sysvar_transaction_isolation](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_transaction_isolation)|

## transaction_read_only
|name|value|
|----|-----|
|Name|`transaction_read_only`|
|Command line|`--transaction-read-only[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_transaction-read-only](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_transaction-read-only)|
|dev.mysql.com|[sysvar_transaction_read_only](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_transaction_read_only)|
|dev.mysql.com|[sysvar_transaction_read_only](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_transaction_read_only)|

## tmpdir
|name|value|
|----|-----|
|Name|`tmpdir`|
|Command line|`--tmpdir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_tmpdir](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_tmpdir)|
|dev.mysql.com|[sysvar_tmpdir](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_tmpdir)|
|dev.mysql.com|[sysvar_tmpdir](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_tmpdir)|

## upgrade
|name|value|
|----|-----|
|Name|`upgrade`|
|Command line|`--upgrade=value`|
|Type of variable|`enumeration`|
|Default value|`AUTO`|
|Valid value(s)|`AUTO`, `NONE`, `MINIMAL`, `FORCE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_upgrade](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_upgrade)|

## user
|name|value|
|----|-----|
|Name|`user`|
|Command line|`--user=name`|
|Type of variable|`string`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_user](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_user)|

## validate_config
|name|value|
|----|-----|
|Name|`validate_config`|
|Command line|`--validate-config[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_validate-config](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_validate-config)|

## validate_user_plugins
|name|value|
|----|-----|
|Name|`validate_user_plugins`|
|Command line|`--validate-user-plugins[={OFF|ON}]`|
|Type of variable|`boolean`|
|Default value|`ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_validate-user-plugins](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#option_mysqld_validate-user-plugins)|

## init_file
|name|value|
|----|-----|
|Name|`init_file`|
|Command line|`--init-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_init_file](https://dev.mysql.com/doc/refman/8.0/en/server-options.html#sysvar_init_file)|
|dev.mysql.com|[sysvar_init_file](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_init_file)|
|dev.mysql.com|[sysvar_init_file](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_init_file)|

## Aborted_clients
|name|value|
|----|-----|
|Name|`Aborted_clients`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Aborted_clients](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Aborted_clients)|
|dev.mysql.com|[statvar_Aborted_clients](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Aborted_clients)|

## Aborted_connects
|name|value|
|----|-----|
|Name|`Aborted_connects`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Aborted_connects](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Aborted_connects)|
|dev.mysql.com|[statvar_Aborted_connects](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Aborted_connects)|

## Authentication_ldap_sasl_supported_methods
|name|value|
|----|-----|
|Name|`Authentication_ldap_sasl_supported_methods`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Authentication_ldap_sasl_supported_methods](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Authentication_ldap_sasl_supported_methods)|

## Binlog_cache_disk_use
|name|value|
|----|-----|
|Name|`Binlog_cache_disk_use`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Binlog_cache_disk_use](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Binlog_cache_disk_use)|
|dev.mysql.com|[statvar_Binlog_cache_disk_use](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Binlog_cache_disk_use)|

## Acl_cache_items_count
|name|value|
|----|-----|
|Name|`Acl_cache_items_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Acl_cache_items_count](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Acl_cache_items_count)|

## Binlog_cache_use
|name|value|
|----|-----|
|Name|`Binlog_cache_use`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Binlog_cache_use](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Binlog_cache_use)|
|dev.mysql.com|[statvar_Binlog_cache_use](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Binlog_cache_use)|

## Binlog_stmt_cache_disk_use
|name|value|
|----|-----|
|Name|`Binlog_stmt_cache_disk_use`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Binlog_stmt_cache_disk_use](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Binlog_stmt_cache_disk_use)|
|dev.mysql.com|[statvar_Binlog_stmt_cache_disk_use](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Binlog_stmt_cache_disk_use)|

## Binlog_stmt_cache_use
|name|value|
|----|-----|
|Name|`Binlog_stmt_cache_use`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Binlog_stmt_cache_use](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Binlog_stmt_cache_use)|
|dev.mysql.com|[statvar_Binlog_stmt_cache_use](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Binlog_stmt_cache_use)|

## Bytes_received
|name|value|
|----|-----|
|Name|`Bytes_received`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Bytes_received](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Bytes_received)|
|dev.mysql.com|[statvar_Bytes_received](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Bytes_received)|

## Bytes_sent
|name|value|
|----|-----|
|Name|`Bytes_sent`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Bytes_sent](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Bytes_sent)|
|dev.mysql.com|[statvar_Bytes_sent](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Bytes_sent)|

## Caching_sha2_password_rsa_public_key
|name|value|
|----|-----|
|Name|`Caching_sha2_password_rsa_public_key`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Caching_sha2_password_rsa_public_key](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Caching_sha2_password_rsa_public_key)|

## Compression
|name|value|
|----|-----|
|Name|`Compression`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Compression](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Compression)|
|dev.mysql.com|[statvar_Compression](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Compression)|

## Compression_algorithm
|name|value|
|----|-----|
|Name|`Compression_algorithm`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Compression_algorithm](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Compression_algorithm)|

## Compression_level
|name|value|
|----|-----|
|Name|`Compression_level`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Compression_level](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Compression_level)|

## Connection_errors_xxx
|name|value|
|----|-----|
|Name|`Connection_errors_xxx`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connection_errors_xxx](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connection_errors_xxx)|
|dev.mysql.com|[statvar_Connection_errors_xxx](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connection_errors_xxx)|

## Connection_errors_accept
|name|value|
|----|-----|
|Name|`Connection_errors_accept`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connection_errors_accept](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connection_errors_accept)|
|dev.mysql.com|[statvar_Connection_errors_accept](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connection_errors_accept)|

## Connection_errors_internal
|name|value|
|----|-----|
|Name|`Connection_errors_internal`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connection_errors_internal](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connection_errors_internal)|
|dev.mysql.com|[statvar_Connection_errors_internal](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connection_errors_internal)|

## Connection_errors_max_connections
|name|value|
|----|-----|
|Name|`Connection_errors_max_connections`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connection_errors_max_connections](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connection_errors_max_connections)|
|dev.mysql.com|[statvar_Connection_errors_max_connections](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connection_errors_max_connections)|

## Connection_errors_peer_address
|name|value|
|----|-----|
|Name|`Connection_errors_peer_address`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connection_errors_peer_address](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connection_errors_peer_address)|
|dev.mysql.com|[statvar_Connection_errors_peer_address](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connection_errors_peer_address)|

## Connection_errors_select
|name|value|
|----|-----|
|Name|`Connection_errors_select`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connection_errors_select](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connection_errors_select)|
|dev.mysql.com|[statvar_Connection_errors_select](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connection_errors_select)|

## Connection_errors_tcpwrap
|name|value|
|----|-----|
|Name|`Connection_errors_tcpwrap`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connection_errors_tcpwrap](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connection_errors_tcpwrap)|
|dev.mysql.com|[statvar_Connection_errors_tcpwrap](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connection_errors_tcpwrap)|

## Connections
|name|value|
|----|-----|
|Name|`Connections`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Connections](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Connections)|
|dev.mysql.com|[statvar_Connections](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Connections)|

## Created_tmp_disk_tables
|name|value|
|----|-----|
|Name|`Created_tmp_disk_tables`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Created_tmp_disk_tables](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Created_tmp_disk_tables)|
|dev.mysql.com|[statvar_Created_tmp_disk_tables](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Created_tmp_disk_tables)|

## Created_tmp_files
|name|value|
|----|-----|
|Name|`Created_tmp_files`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Created_tmp_files](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Created_tmp_files)|
|dev.mysql.com|[statvar_Created_tmp_files](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Created_tmp_files)|

## Created_tmp_tables
|name|value|
|----|-----|
|Name|`Created_tmp_tables`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Created_tmp_tables](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Created_tmp_tables)|
|dev.mysql.com|[statvar_Created_tmp_tables](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Created_tmp_tables)|

## Current_tls_ca
|name|value|
|----|-----|
|Name|`Current_tls_ca`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_ca](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_ca)|

## Current_tls_capath
|name|value|
|----|-----|
|Name|`Current_tls_capath`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_capath](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_capath)|

## Current_tls_cert
|name|value|
|----|-----|
|Name|`Current_tls_cert`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_cert](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_cert)|

## Current_tls_cipher
|name|value|
|----|-----|
|Name|`Current_tls_cipher`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_cipher](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_cipher)|

## Current_tls_ciphersuites
|name|value|
|----|-----|
|Name|`Current_tls_ciphersuites`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_ciphersuites](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_ciphersuites)|

## Current_tls_crl
|name|value|
|----|-----|
|Name|`Current_tls_crl`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_crl](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_crl)|

## Current_tls_crlpath
|name|value|
|----|-----|
|Name|`Current_tls_crlpath`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_crlpath](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_crlpath)|

## Current_tls_key
|name|value|
|----|-----|
|Name|`Current_tls_key`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_key](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_key)|

## Current_tls_version
|name|value|
|----|-----|
|Name|`Current_tls_version`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Current_tls_version](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Current_tls_version)|

## Delayed_errors
|name|value|
|----|-----|
|Name|`Delayed_errors`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Delayed_errors](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Delayed_errors)|
|dev.mysql.com|[statvar_Delayed_errors](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Delayed_errors)|

## Delayed_insert_threads
|name|value|
|----|-----|
|Name|`Delayed_insert_threads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Delayed_insert_threads](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Delayed_insert_threads)|
|dev.mysql.com|[statvar_Delayed_insert_threads](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Delayed_insert_threads)|

## Delayed_writes
|name|value|
|----|-----|
|Name|`Delayed_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Delayed_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Delayed_writes)|
|dev.mysql.com|[statvar_Delayed_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Delayed_writes)|

## dragnet.Status
|name|value|
|----|-----|
|Name|`dragnet.Status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_dragnet.Status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_dragnet.Status)|

## Error_log_buffered_bytes
|name|value|
|----|-----|
|Name|`Error_log_buffered_bytes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Error_log_buffered_bytes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Error_log_buffered_bytes)|

## Error_log_buffered_events
|name|value|
|----|-----|
|Name|`Error_log_buffered_events`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Error_log_buffered_events](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Error_log_buffered_events)|

## Error_log_expired_events
|name|value|
|----|-----|
|Name|`Error_log_expired_events`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Error_log_expired_events](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Error_log_expired_events)|

## Error_log_latest_write
|name|value|
|----|-----|
|Name|`Error_log_latest_write`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Error_log_latest_write](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Error_log_latest_write)|

## Flush_commands
|name|value|
|----|-----|
|Name|`Flush_commands`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Flush_commands](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Flush_commands)|
|dev.mysql.com|[statvar_Flush_commands](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Flush_commands)|

## Global_connection_memory
|name|value|
|----|-----|
|Name|`Global_connection_memory`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Global_connection_memory](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Global_connection_memory)|

## Handler_commit
|name|value|
|----|-----|
|Name|`Handler_commit`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_commit](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_commit)|
|dev.mysql.com|[statvar_Handler_commit](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_commit)|

## Handler_delete
|name|value|
|----|-----|
|Name|`Handler_delete`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_delete](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_delete)|
|dev.mysql.com|[statvar_Handler_delete](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_delete)|

## Handler_external_lock
|name|value|
|----|-----|
|Name|`Handler_external_lock`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_external_lock](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_external_lock)|
|dev.mysql.com|[statvar_Handler_external_lock](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_external_lock)|

## Handler_mrr_init
|name|value|
|----|-----|
|Name|`Handler_mrr_init`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_mrr_init](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_mrr_init)|
|dev.mysql.com|[statvar_Handler_mrr_init](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_mrr_init)|

## Handler_prepare
|name|value|
|----|-----|
|Name|`Handler_prepare`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_prepare](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_prepare)|
|dev.mysql.com|[statvar_Handler_prepare](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_prepare)|

## Handler_read_first
|name|value|
|----|-----|
|Name|`Handler_read_first`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_read_first](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_read_first)|
|dev.mysql.com|[statvar_Handler_read_first](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_read_first)|

## Handler_read_key
|name|value|
|----|-----|
|Name|`Handler_read_key`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_read_key](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_read_key)|
|dev.mysql.com|[statvar_Handler_read_key](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_read_key)|

## Handler_read_last
|name|value|
|----|-----|
|Name|`Handler_read_last`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_read_last](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_read_last)|
|dev.mysql.com|[statvar_Handler_read_last](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_read_last)|

## Handler_read_next
|name|value|
|----|-----|
|Name|`Handler_read_next`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_read_next](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_read_next)|
|dev.mysql.com|[statvar_Handler_read_next](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_read_next)|

## Handler_read_prev
|name|value|
|----|-----|
|Name|`Handler_read_prev`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_read_prev](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_read_prev)|
|dev.mysql.com|[statvar_Handler_read_prev](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_read_prev)|

## Handler_read_rnd
|name|value|
|----|-----|
|Name|`Handler_read_rnd`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_read_rnd](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_read_rnd)|
|dev.mysql.com|[statvar_Handler_read_rnd](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_read_rnd)|

## Handler_read_rnd_next
|name|value|
|----|-----|
|Name|`Handler_read_rnd_next`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_read_rnd_next](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_read_rnd_next)|
|dev.mysql.com|[statvar_Handler_read_rnd_next](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_read_rnd_next)|

## Handler_rollback
|name|value|
|----|-----|
|Name|`Handler_rollback`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_rollback](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_rollback)|
|dev.mysql.com|[statvar_Handler_rollback](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_rollback)|

## Handler_savepoint
|name|value|
|----|-----|
|Name|`Handler_savepoint`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_savepoint](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_savepoint)|
|dev.mysql.com|[statvar_Handler_savepoint](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_savepoint)|

## Handler_savepoint_rollback
|name|value|
|----|-----|
|Name|`Handler_savepoint_rollback`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_savepoint_rollback](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_savepoint_rollback)|
|dev.mysql.com|[statvar_Handler_savepoint_rollback](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_savepoint_rollback)|

## Handler_update
|name|value|
|----|-----|
|Name|`Handler_update`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_update](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_update)|
|dev.mysql.com|[statvar_Handler_update](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_update)|

## Handler_write
|name|value|
|----|-----|
|Name|`Handler_write`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Handler_write](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Handler_write)|
|dev.mysql.com|[statvar_Handler_write](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Handler_write)|

## Innodb_buffer_pool_dump_status
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_dump_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_dump_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_dump_status)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_dump_status](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_dump_status)|

## Innodb_buffer_pool_load_status
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_load_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_load_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_load_status)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_load_status](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_load_status)|

## Innodb_buffer_pool_bytes_data
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_bytes_data`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_bytes_data](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_bytes_data)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_bytes_data](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_bytes_data)|

## Innodb_buffer_pool_pages_data
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_pages_data`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_data](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_data)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_data](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_data)|

## Innodb_buffer_pool_bytes_dirty
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_bytes_dirty`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_bytes_dirty](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_bytes_dirty)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_bytes_dirty](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_bytes_dirty)|

## Innodb_buffer_pool_pages_dirty
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_pages_dirty`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_dirty](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_dirty)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_dirty](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_dirty)|

## Innodb_buffer_pool_pages_flushed
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_pages_flushed`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_flushed](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_flushed)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_flushed](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_flushed)|

## Innodb_buffer_pool_pages_free
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_pages_free`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_free](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_free)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_free](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_free)|

## Innodb_buffer_pool_pages_latched
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_pages_latched`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_latched](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_latched)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_latched](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_latched)|

## Innodb_buffer_pool_pages_misc
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_pages_misc`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_misc](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_misc)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_misc](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_misc)|

## Innodb_buffer_pool_pages_total
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_pages_total`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_total](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_total)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_pages_total](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_pages_total)|

## Innodb_buffer_pool_read_ahead
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_read_ahead`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_ahead](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_ahead)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_ahead](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_ahead)|

## Innodb_buffer_pool_read_ahead_evicted
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_read_ahead_evicted`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_ahead_evicted](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_ahead_evicted)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_ahead_evicted](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_ahead_evicted)|

## Innodb_buffer_pool_read_ahead_rnd
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_read_ahead_rnd`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_ahead_rnd](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_ahead_rnd)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_ahead_rnd](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_ahead_rnd)|

## Innodb_buffer_pool_read_requests
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_read_requests`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_requests](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_requests)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_read_requests](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_read_requests)|

## Innodb_buffer_pool_reads
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_reads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_reads](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_reads)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_reads](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_reads)|

## Innodb_buffer_pool_resize_status
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_resize_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_resize_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_resize_status)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_resize_status](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_resize_status)|

## Innodb_buffer_pool_resize_status_code
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_resize_status_code`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_resize_status_code](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_resize_status_code)|

## Innodb_buffer_pool_resize_status_progress
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_resize_status_progress`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_resize_status_progress](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_resize_status_progress)|

## Innodb_buffer_pool_wait_free
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_wait_free`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_wait_free](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_wait_free)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_wait_free](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_wait_free)|

## Innodb_buffer_pool_write_requests
|name|value|
|----|-----|
|Name|`Innodb_buffer_pool_write_requests`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_buffer_pool_write_requests](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_buffer_pool_write_requests)|
|dev.mysql.com|[statvar_Innodb_buffer_pool_write_requests](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_buffer_pool_write_requests)|

## Innodb_data_fsyncs
|name|value|
|----|-----|
|Name|`Innodb_data_fsyncs`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_fsyncs](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_fsyncs)|
|dev.mysql.com|[statvar_Innodb_data_fsyncs](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_fsyncs)|

## Innodb_data_pending_fsyncs
|name|value|
|----|-----|
|Name|`Innodb_data_pending_fsyncs`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_pending_fsyncs](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_pending_fsyncs)|
|dev.mysql.com|[statvar_Innodb_data_pending_fsyncs](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_pending_fsyncs)|

## Innodb_data_pending_reads
|name|value|
|----|-----|
|Name|`Innodb_data_pending_reads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_pending_reads](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_pending_reads)|
|dev.mysql.com|[statvar_Innodb_data_pending_reads](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_pending_reads)|

## Innodb_data_pending_writes
|name|value|
|----|-----|
|Name|`Innodb_data_pending_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_pending_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_pending_writes)|
|dev.mysql.com|[statvar_Innodb_data_pending_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_pending_writes)|

## Innodb_data_read
|name|value|
|----|-----|
|Name|`Innodb_data_read`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_read](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_read)|
|dev.mysql.com|[statvar_Innodb_data_read](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_read)|

## Innodb_data_reads
|name|value|
|----|-----|
|Name|`Innodb_data_reads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_reads](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_reads)|
|dev.mysql.com|[statvar_Innodb_data_reads](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_reads)|

## Innodb_data_writes
|name|value|
|----|-----|
|Name|`Innodb_data_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_writes)|
|dev.mysql.com|[statvar_Innodb_data_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_writes)|

## Innodb_data_written
|name|value|
|----|-----|
|Name|`Innodb_data_written`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_data_written](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_data_written)|
|dev.mysql.com|[statvar_Innodb_data_written](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_data_written)|

## Innodb_dblwr_pages_written
|name|value|
|----|-----|
|Name|`Innodb_dblwr_pages_written`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_dblwr_pages_written](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_dblwr_pages_written)|
|dev.mysql.com|[statvar_Innodb_dblwr_pages_written](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_dblwr_pages_written)|

## Innodb_dblwr_writes
|name|value|
|----|-----|
|Name|`Innodb_dblwr_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_dblwr_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_dblwr_writes)|
|dev.mysql.com|[statvar_Innodb_dblwr_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_dblwr_writes)|

## Innodb_have_atomic_builtins
|name|value|
|----|-----|
|Name|`Innodb_have_atomic_builtins`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_have_atomic_builtins](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_have_atomic_builtins)|
|dev.mysql.com|[statvar_Innodb_have_atomic_builtins](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_have_atomic_builtins)|

## Innodb_log_waits
|name|value|
|----|-----|
|Name|`Innodb_log_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_log_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_log_waits)|
|dev.mysql.com|[statvar_Innodb_log_waits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_log_waits)|

## Innodb_log_write_requests
|name|value|
|----|-----|
|Name|`Innodb_log_write_requests`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_log_write_requests](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_log_write_requests)|
|dev.mysql.com|[statvar_Innodb_log_write_requests](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_log_write_requests)|

## Innodb_log_writes
|name|value|
|----|-----|
|Name|`Innodb_log_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_log_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_log_writes)|
|dev.mysql.com|[statvar_Innodb_log_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_log_writes)|

## Innodb_num_open_files
|name|value|
|----|-----|
|Name|`Innodb_num_open_files`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_num_open_files](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_num_open_files)|
|dev.mysql.com|[statvar_Innodb_num_open_files](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_num_open_files)|

## Innodb_os_log_fsyncs
|name|value|
|----|-----|
|Name|`Innodb_os_log_fsyncs`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_os_log_fsyncs](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_os_log_fsyncs)|
|dev.mysql.com|[statvar_Innodb_os_log_fsyncs](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_os_log_fsyncs)|

## Innodb_os_log_pending_fsyncs
|name|value|
|----|-----|
|Name|`Innodb_os_log_pending_fsyncs`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_os_log_pending_fsyncs](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_os_log_pending_fsyncs)|
|dev.mysql.com|[statvar_Innodb_os_log_pending_fsyncs](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_os_log_pending_fsyncs)|

## Innodb_os_log_pending_writes
|name|value|
|----|-----|
|Name|`Innodb_os_log_pending_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_os_log_pending_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_os_log_pending_writes)|
|dev.mysql.com|[statvar_Innodb_os_log_pending_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_os_log_pending_writes)|

## Innodb_os_log_written
|name|value|
|----|-----|
|Name|`Innodb_os_log_written`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_os_log_written](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_os_log_written)|
|dev.mysql.com|[statvar_Innodb_os_log_written](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_os_log_written)|

## Innodb_page_size
|name|value|
|----|-----|
|Name|`Innodb_page_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_page_size](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_page_size)|
|dev.mysql.com|[statvar_Innodb_page_size](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_page_size)|

## Innodb_pages_created
|name|value|
|----|-----|
|Name|`Innodb_pages_created`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_pages_created](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_pages_created)|
|dev.mysql.com|[statvar_Innodb_pages_created](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_pages_created)|

## Innodb_pages_read
|name|value|
|----|-----|
|Name|`Innodb_pages_read`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_pages_read](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_pages_read)|
|dev.mysql.com|[statvar_Innodb_pages_read](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_pages_read)|

## Innodb_pages_written
|name|value|
|----|-----|
|Name|`Innodb_pages_written`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_pages_written](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_pages_written)|
|dev.mysql.com|[statvar_Innodb_pages_written](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_pages_written)|

## Innodb_redo_log_enabled
|name|value|
|----|-----|
|Name|`Innodb_redo_log_enabled`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_enabled](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_enabled)|

## Innodb_redo_log_capacity_resized
|name|value|
|----|-----|
|Name|`Innodb_redo_log_capacity_resized`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_capacity_resized](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_capacity_resized)|

## Innodb_redo_log_checkpoint_lsn
|name|value|
|----|-----|
|Name|`Innodb_redo_log_checkpoint_lsn`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_checkpoint_lsn](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_checkpoint_lsn)|

## Innodb_redo_log_current_lsn
|name|value|
|----|-----|
|Name|`Innodb_redo_log_current_lsn`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_current_lsn](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_current_lsn)|

## Innodb_redo_log_flushed_to_disk_lsn
|name|value|
|----|-----|
|Name|`Innodb_redo_log_flushed_to_disk_lsn`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_flushed_to_disk_lsn](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_flushed_to_disk_lsn)|

## Innodb_redo_log_logical_size
|name|value|
|----|-----|
|Name|`Innodb_redo_log_logical_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_logical_size](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_logical_size)|

## Innodb_redo_log_physical_size
|name|value|
|----|-----|
|Name|`Innodb_redo_log_physical_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_physical_size](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_physical_size)|

## Innodb_redo_log_read_only
|name|value|
|----|-----|
|Name|`Innodb_redo_log_read_only`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_read_only](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_read_only)|

## Innodb_redo_log_resize_status
|name|value|
|----|-----|
|Name|`Innodb_redo_log_resize_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_resize_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_resize_status)|

## Innodb_redo_log_uuid
|name|value|
|----|-----|
|Name|`Innodb_redo_log_uuid`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_redo_log_uuid](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_redo_log_uuid)|

## Innodb_row_lock_current_waits
|name|value|
|----|-----|
|Name|`Innodb_row_lock_current_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_row_lock_current_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_row_lock_current_waits)|
|dev.mysql.com|[statvar_Innodb_row_lock_current_waits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_row_lock_current_waits)|

## Innodb_row_lock_time
|name|value|
|----|-----|
|Name|`Innodb_row_lock_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_row_lock_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_row_lock_time)|
|dev.mysql.com|[statvar_Innodb_row_lock_time](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_row_lock_time)|

## Innodb_row_lock_time_avg
|name|value|
|----|-----|
|Name|`Innodb_row_lock_time_avg`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_row_lock_time_avg](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_row_lock_time_avg)|
|dev.mysql.com|[statvar_Innodb_row_lock_time_avg](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_row_lock_time_avg)|

## Innodb_row_lock_time_max
|name|value|
|----|-----|
|Name|`Innodb_row_lock_time_max`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_row_lock_time_max](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_row_lock_time_max)|
|dev.mysql.com|[statvar_Innodb_row_lock_time_max](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_row_lock_time_max)|

## Innodb_row_lock_waits
|name|value|
|----|-----|
|Name|`Innodb_row_lock_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_row_lock_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_row_lock_waits)|
|dev.mysql.com|[statvar_Innodb_row_lock_waits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_row_lock_waits)|

## Innodb_rows_deleted
|name|value|
|----|-----|
|Name|`Innodb_rows_deleted`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_rows_deleted](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_rows_deleted)|
|dev.mysql.com|[statvar_Innodb_rows_deleted](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_rows_deleted)|

## Innodb_rows_inserted
|name|value|
|----|-----|
|Name|`Innodb_rows_inserted`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_rows_inserted](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_rows_inserted)|
|dev.mysql.com|[statvar_Innodb_rows_inserted](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_rows_inserted)|

## Innodb_rows_read
|name|value|
|----|-----|
|Name|`Innodb_rows_read`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_rows_read](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_rows_read)|
|dev.mysql.com|[statvar_Innodb_rows_read](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_rows_read)|

## Innodb_rows_updated
|name|value|
|----|-----|
|Name|`Innodb_rows_updated`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_rows_updated](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_rows_updated)|
|dev.mysql.com|[statvar_Innodb_rows_updated](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_rows_updated)|

## Innodb_system_rows_deleted
|name|value|
|----|-----|
|Name|`Innodb_system_rows_deleted`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_system_rows_deleted](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_system_rows_deleted)|

## Innodb_system_rows_inserted
|name|value|
|----|-----|
|Name|`Innodb_system_rows_inserted`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_system_rows_inserted](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_system_rows_inserted)|

## Innodb_system_rows_updated
|name|value|
|----|-----|
|Name|`Innodb_system_rows_updated`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_system_rows_updated](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_system_rows_updated)|

## Innodb_system_rows_read
|name|value|
|----|-----|
|Name|`Innodb_system_rows_read`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_system_rows_read](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_system_rows_read)|

## Innodb_truncated_status_writes
|name|value|
|----|-----|
|Name|`Innodb_truncated_status_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_truncated_status_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_truncated_status_writes)|
|dev.mysql.com|[statvar_Innodb_truncated_status_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_truncated_status_writes)|

## Innodb_undo_tablespaces_active
|name|value|
|----|-----|
|Name|`Innodb_undo_tablespaces_active`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_undo_tablespaces_active](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_undo_tablespaces_active)|

## Innodb_undo_tablespaces_explicit
|name|value|
|----|-----|
|Name|`Innodb_undo_tablespaces_explicit`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_undo_tablespaces_explicit](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_undo_tablespaces_explicit)|

## Innodb_undo_tablespaces_implicit
|name|value|
|----|-----|
|Name|`Innodb_undo_tablespaces_implicit`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_undo_tablespaces_implicit](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_undo_tablespaces_implicit)|

## Innodb_undo_tablespaces_total
|name|value|
|----|-----|
|Name|`Innodb_undo_tablespaces_total`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_undo_tablespaces_total](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Innodb_undo_tablespaces_total)|

## Key_blocks_not_flushed
|name|value|
|----|-----|
|Name|`Key_blocks_not_flushed`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Key_blocks_not_flushed](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Key_blocks_not_flushed)|
|dev.mysql.com|[statvar_Key_blocks_not_flushed](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Key_blocks_not_flushed)|

## Key_blocks_unused
|name|value|
|----|-----|
|Name|`Key_blocks_unused`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Key_blocks_unused](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Key_blocks_unused)|
|dev.mysql.com|[statvar_Key_blocks_unused](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Key_blocks_unused)|

## Key_blocks_used
|name|value|
|----|-----|
|Name|`Key_blocks_used`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Key_blocks_used](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Key_blocks_used)|
|dev.mysql.com|[statvar_Key_blocks_used](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Key_blocks_used)|

## Key_read_requests
|name|value|
|----|-----|
|Name|`Key_read_requests`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Key_read_requests](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Key_read_requests)|
|dev.mysql.com|[statvar_Key_read_requests](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Key_read_requests)|

## Key_reads
|name|value|
|----|-----|
|Name|`Key_reads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Key_reads](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Key_reads)|
|dev.mysql.com|[statvar_Key_reads](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Key_reads)|

## Key_write_requests
|name|value|
|----|-----|
|Name|`Key_write_requests`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Key_write_requests](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Key_write_requests)|
|dev.mysql.com|[statvar_Key_write_requests](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Key_write_requests)|

## Key_writes
|name|value|
|----|-----|
|Name|`Key_writes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Key_writes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Key_writes)|
|dev.mysql.com|[statvar_Key_writes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Key_writes)|

## Last_query_cost
|name|value|
|----|-----|
|Name|`Last_query_cost`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Last_query_cost](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Last_query_cost)|
|dev.mysql.com|[statvar_Last_query_cost](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Last_query_cost)|

## Last_query_partial_plans
|name|value|
|----|-----|
|Name|`Last_query_partial_plans`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Last_query_partial_plans](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Last_query_partial_plans)|
|dev.mysql.com|[statvar_Last_query_partial_plans](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Last_query_partial_plans)|

## Locked_connects
|name|value|
|----|-----|
|Name|`Locked_connects`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Locked_connects](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Locked_connects)|
|dev.mysql.com|[statvar_Locked_connects](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Locked_connects)|

## Max_execution_time_exceeded
|name|value|
|----|-----|
|Name|`Max_execution_time_exceeded`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Max_execution_time_exceeded](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Max_execution_time_exceeded)|
|dev.mysql.com|[statvar_Max_execution_time_exceeded](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Max_execution_time_exceeded)|

## Max_execution_time_set
|name|value|
|----|-----|
|Name|`Max_execution_time_set`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Max_execution_time_set](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Max_execution_time_set)|
|dev.mysql.com|[statvar_Max_execution_time_set](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Max_execution_time_set)|

## Max_execution_time_set_failed
|name|value|
|----|-----|
|Name|`Max_execution_time_set_failed`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Max_execution_time_set_failed](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Max_execution_time_set_failed)|
|dev.mysql.com|[statvar_Max_execution_time_set_failed](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Max_execution_time_set_failed)|

## Max_used_connections
|name|value|
|----|-----|
|Name|`Max_used_connections`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Max_used_connections](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Max_used_connections)|
|dev.mysql.com|[statvar_Max_used_connections](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Max_used_connections)|
|dev.mysql.com|[statvar_Max_used_connections](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#statvar_Max_used_connections)|

## Max_used_connections_time
|name|value|
|----|-----|
|Name|`Max_used_connections_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Max_used_connections_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Max_used_connections_time)|
|dev.mysql.com|[statvar_Max_used_connections_time](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Max_used_connections_time)|

## Not_flushed_delayed_rows
|name|value|
|----|-----|
|Name|`Not_flushed_delayed_rows`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Not_flushed_delayed_rows](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Not_flushed_delayed_rows)|
|dev.mysql.com|[statvar_Not_flushed_delayed_rows](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Not_flushed_delayed_rows)|

## mecab_charset
|name|value|
|----|-----|
|Name|`mecab_charset`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_mecab_charset](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_mecab_charset)|
|dev.mysql.com|[statvar_mecab_charset](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_mecab_charset)|

## Ongoing_anonymous_transaction_count
|name|value|
|----|-----|
|Name|`Ongoing_anonymous_transaction_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ongoing_anonymous_transaction_count](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ongoing_anonymous_transaction_count)|
|dev.mysql.com|[statvar_Ongoing_anonymous_transaction_count](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ongoing_anonymous_transaction_count)|

## Ongoing_anonymous_gtid_violating_transaction_count
|name|value|
|----|-----|
|Name|`Ongoing_anonymous_gtid_violating_transaction_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ongoing_anonymous_gtid_violating_transaction_count](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ongoing_anonymous_gtid_violating_transaction_count)|
|dev.mysql.com|[statvar_Ongoing_anonymous_gtid_violating_transaction_count](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ongoing_anonymous_gtid_violating_transaction_count)|

## Ongoing_automatic_gtid_violating_transaction_count
|name|value|
|----|-----|
|Name|`Ongoing_automatic_gtid_violating_transaction_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ongoing_automatic_gtid_violating_transaction_count](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ongoing_automatic_gtid_violating_transaction_count)|
|dev.mysql.com|[statvar_Ongoing_automatic_gtid_violating_transaction_count](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ongoing_automatic_gtid_violating_transaction_count)|

## Open_files
|name|value|
|----|-----|
|Name|`Open_files`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Open_files](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Open_files)|
|dev.mysql.com|[statvar_Open_files](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Open_files)|

## Open_streams
|name|value|
|----|-----|
|Name|`Open_streams`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Open_streams](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Open_streams)|
|dev.mysql.com|[statvar_Open_streams](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Open_streams)|

## Open_table_definitions
|name|value|
|----|-----|
|Name|`Open_table_definitions`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Open_table_definitions](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Open_table_definitions)|
|dev.mysql.com|[statvar_Open_table_definitions](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Open_table_definitions)|

## Open_tables
|name|value|
|----|-----|
|Name|`Open_tables`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Open_tables](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Open_tables)|
|dev.mysql.com|[statvar_Open_tables](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Open_tables)|

## Opened_files
|name|value|
|----|-----|
|Name|`Opened_files`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Opened_files](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Opened_files)|
|dev.mysql.com|[statvar_Opened_files](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Opened_files)|

## Opened_table_definitions
|name|value|
|----|-----|
|Name|`Opened_table_definitions`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Opened_table_definitions](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Opened_table_definitions)|
|dev.mysql.com|[statvar_Opened_table_definitions](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Opened_table_definitions)|

## Opened_tables
|name|value|
|----|-----|
|Name|`Opened_tables`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Opened_tables](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Opened_tables)|
|dev.mysql.com|[statvar_Opened_tables](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Opened_tables)|

## Prepared_stmt_count
|name|value|
|----|-----|
|Name|`Prepared_stmt_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Prepared_stmt_count](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Prepared_stmt_count)|
|dev.mysql.com|[statvar_Prepared_stmt_count](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Prepared_stmt_count)|

## Queries
|name|value|
|----|-----|
|Name|`Queries`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Queries](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Queries)|
|dev.mysql.com|[statvar_Queries](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Queries)|

## Questions
|name|value|
|----|-----|
|Name|`Questions`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Questions](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Questions)|
|dev.mysql.com|[statvar_Questions](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Questions)|

## Replica_open_temp_tables
|name|value|
|----|-----|
|Name|`Replica_open_temp_tables`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Replica_open_temp_tables](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Replica_open_temp_tables)|

## Replica_rows_last_search_algorithm_used
|name|value|
|----|-----|
|Name|`Replica_rows_last_search_algorithm_used`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Replica_rows_last_search_algorithm_used](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Replica_rows_last_search_algorithm_used)|

## Resource_group_supported
|name|value|
|----|-----|
|Name|`Resource_group_supported`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Resource_group_supported](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Resource_group_supported)|

## Rpl_semi_sync_master_clients
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_clients`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_clients](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_clients)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_clients](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_clients)|

## Rpl_semi_sync_master_net_avg_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_net_avg_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_net_avg_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_net_avg_wait_time)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_net_avg_wait_time](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_net_avg_wait_time)|

## Rpl_semi_sync_master_net_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_net_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_net_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_net_wait_time)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_net_wait_time](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_net_wait_time)|

## Rpl_semi_sync_master_net_waits
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_net_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_net_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_net_waits)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_net_waits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_net_waits)|

## Rpl_semi_sync_master_no_times
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_no_times`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_no_times](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_no_times)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_no_times](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_no_times)|

## Rpl_semi_sync_master_no_tx
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_no_tx`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_no_tx](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_no_tx)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_no_tx](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_no_tx)|

## Rpl_semi_sync_master_status
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_status)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_status](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_status)|

## Rpl_semi_sync_master_timefunc_failures
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_timefunc_failures`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_timefunc_failures](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_timefunc_failures)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_timefunc_failures](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_timefunc_failures)|

## Rpl_semi_sync_master_tx_avg_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_tx_avg_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_tx_avg_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_tx_avg_wait_time)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_tx_avg_wait_time](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_tx_avg_wait_time)|

## Rpl_semi_sync_master_tx_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_tx_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_tx_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_tx_wait_time)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_tx_wait_time](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_tx_wait_time)|

## Rpl_semi_sync_master_tx_waits
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_tx_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_tx_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_tx_waits)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_tx_waits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_tx_waits)|

## Rpl_semi_sync_master_wait_pos_backtraverse
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_wait_pos_backtraverse`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_wait_pos_backtraverse](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_wait_pos_backtraverse)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_wait_pos_backtraverse](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_wait_pos_backtraverse)|

## Rpl_semi_sync_master_wait_sessions
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_wait_sessions`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_wait_sessions](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_wait_sessions)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_wait_sessions](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_wait_sessions)|

## Rpl_semi_sync_master_yes_tx
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_master_yes_tx`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_yes_tx](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_master_yes_tx)|
|dev.mysql.com|[statvar_Rpl_semi_sync_master_yes_tx](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_master_yes_tx)|

## Rpl_semi_sync_source_clients
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_clients`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_clients](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_clients)|

## Rpl_semi_sync_source_net_avg_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_net_avg_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_net_avg_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_net_avg_wait_time)|

## Rpl_semi_sync_source_net_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_net_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_net_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_net_wait_time)|

## Rpl_semi_sync_source_net_waits
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_net_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_net_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_net_waits)|

## Rpl_semi_sync_source_no_times
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_no_times`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_no_times](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_no_times)|

## Rpl_semi_sync_source_no_tx
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_no_tx`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_no_tx](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_no_tx)|

## Rpl_semi_sync_source_status
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_status)|

## Rpl_semi_sync_source_timefunc_failures
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_timefunc_failures`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_timefunc_failures](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_timefunc_failures)|

## Rpl_semi_sync_source_tx_avg_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_tx_avg_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_tx_avg_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_tx_avg_wait_time)|

## Rpl_semi_sync_source_tx_wait_time
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_tx_wait_time`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_tx_wait_time](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_tx_wait_time)|

## Rpl_semi_sync_source_tx_waits
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_tx_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_tx_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_tx_waits)|

## Rpl_semi_sync_source_wait_pos_backtraverse
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_wait_pos_backtraverse`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_wait_pos_backtraverse](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_wait_pos_backtraverse)|

## Rpl_semi_sync_source_wait_sessions
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_wait_sessions`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_wait_sessions](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_wait_sessions)|

## Rpl_semi_sync_source_yes_tx
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_source_yes_tx`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_source_yes_tx](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_source_yes_tx)|

## Rpl_semi_sync_replica_status
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_replica_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_replica_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_replica_status)|

## Rpl_semi_sync_slave_status
|name|value|
|----|-----|
|Name|`Rpl_semi_sync_slave_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rpl_semi_sync_slave_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rpl_semi_sync_slave_status)|
|dev.mysql.com|[statvar_Rpl_semi_sync_slave_status](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rpl_semi_sync_slave_status)|

## Rsa_public_key
|name|value|
|----|-----|
|Name|`Rsa_public_key`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Rsa_public_key](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Rsa_public_key)|
|dev.mysql.com|[statvar_Rsa_public_key](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Rsa_public_key)|

## Select_full_join
|name|value|
|----|-----|
|Name|`Select_full_join`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Select_full_join](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Select_full_join)|
|dev.mysql.com|[statvar_Select_full_join](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Select_full_join)|

## Select_full_range_join
|name|value|
|----|-----|
|Name|`Select_full_range_join`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Select_full_range_join](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Select_full_range_join)|
|dev.mysql.com|[statvar_Select_full_range_join](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Select_full_range_join)|

## Select_range
|name|value|
|----|-----|
|Name|`Select_range`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Select_range](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Select_range)|
|dev.mysql.com|[statvar_Select_range](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Select_range)|

## Select_range_check
|name|value|
|----|-----|
|Name|`Select_range_check`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Select_range_check](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Select_range_check)|
|dev.mysql.com|[statvar_Select_range_check](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Select_range_check)|

## Select_scan
|name|value|
|----|-----|
|Name|`Select_scan`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Select_scan](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Select_scan)|
|dev.mysql.com|[statvar_Select_scan](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Select_scan)|

## Slave_open_temp_tables
|name|value|
|----|-----|
|Name|`Slave_open_temp_tables`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slave_open_temp_tables](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Slave_open_temp_tables)|
|dev.mysql.com|[statvar_Slave_open_temp_tables](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slave_open_temp_tables)|

## Slave_rows_last_search_algorithm_used
|name|value|
|----|-----|
|Name|`Slave_rows_last_search_algorithm_used`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slave_rows_last_search_algorithm_used](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Slave_rows_last_search_algorithm_used)|
|dev.mysql.com|[statvar_Slave_rows_last_search_algorithm_used](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slave_rows_last_search_algorithm_used)|

## Slow_launch_threads
|name|value|
|----|-----|
|Name|`Slow_launch_threads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slow_launch_threads](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Slow_launch_threads)|
|dev.mysql.com|[statvar_Slow_launch_threads](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slow_launch_threads)|

## Slow_queries
|name|value|
|----|-----|
|Name|`Slow_queries`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slow_queries](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Slow_queries)|
|dev.mysql.com|[statvar_Slow_queries](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slow_queries)|

## Sort_merge_passes
|name|value|
|----|-----|
|Name|`Sort_merge_passes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Sort_merge_passes](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Sort_merge_passes)|
|dev.mysql.com|[statvar_Sort_merge_passes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Sort_merge_passes)|

## Sort_range
|name|value|
|----|-----|
|Name|`Sort_range`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Sort_range](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Sort_range)|
|dev.mysql.com|[statvar_Sort_range](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Sort_range)|

## Sort_rows
|name|value|
|----|-----|
|Name|`Sort_rows`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Sort_rows](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Sort_rows)|
|dev.mysql.com|[statvar_Sort_rows](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Sort_rows)|

## Sort_scan
|name|value|
|----|-----|
|Name|`Sort_scan`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Sort_scan](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Sort_scan)|
|dev.mysql.com|[statvar_Sort_scan](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Sort_scan)|

## Ssl_accept_renegotiates
|name|value|
|----|-----|
|Name|`Ssl_accept_renegotiates`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_accept_renegotiates](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_accept_renegotiates)|
|dev.mysql.com|[statvar_Ssl_accept_renegotiates](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_accept_renegotiates)|

## Ssl_accepts
|name|value|
|----|-----|
|Name|`Ssl_accepts`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_accepts](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_accepts)|
|dev.mysql.com|[statvar_Ssl_accepts](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_accepts)|

## Ssl_callback_cache_hits
|name|value|
|----|-----|
|Name|`Ssl_callback_cache_hits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_callback_cache_hits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_callback_cache_hits)|
|dev.mysql.com|[statvar_Ssl_callback_cache_hits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_callback_cache_hits)|

## Ssl_cipher
|name|value|
|----|-----|
|Name|`Ssl_cipher`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_cipher](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_cipher)|
|dev.mysql.com|[statvar_Ssl_cipher](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_cipher)|

## Ssl_cipher_list
|name|value|
|----|-----|
|Name|`Ssl_cipher_list`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_cipher_list](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_cipher_list)|
|dev.mysql.com|[statvar_Ssl_cipher_list](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_cipher_list)|

## Ssl_client_connects
|name|value|
|----|-----|
|Name|`Ssl_client_connects`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_client_connects](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_client_connects)|
|dev.mysql.com|[statvar_Ssl_client_connects](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_client_connects)|

## Ssl_connect_renegotiates
|name|value|
|----|-----|
|Name|`Ssl_connect_renegotiates`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_connect_renegotiates](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_connect_renegotiates)|
|dev.mysql.com|[statvar_Ssl_connect_renegotiates](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_connect_renegotiates)|

## Ssl_ctx_verify_depth
|name|value|
|----|-----|
|Name|`Ssl_ctx_verify_depth`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_ctx_verify_depth](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_ctx_verify_depth)|
|dev.mysql.com|[statvar_Ssl_ctx_verify_depth](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_ctx_verify_depth)|

## Ssl_ctx_verify_mode
|name|value|
|----|-----|
|Name|`Ssl_ctx_verify_mode`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_ctx_verify_mode](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_ctx_verify_mode)|
|dev.mysql.com|[statvar_Ssl_ctx_verify_mode](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_ctx_verify_mode)|

## Ssl_default_timeout
|name|value|
|----|-----|
|Name|`Ssl_default_timeout`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_default_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_default_timeout)|
|dev.mysql.com|[statvar_Ssl_default_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_default_timeout)|

## Ssl_finished_accepts
|name|value|
|----|-----|
|Name|`Ssl_finished_accepts`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_finished_accepts](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_finished_accepts)|
|dev.mysql.com|[statvar_Ssl_finished_accepts](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_finished_accepts)|

## Ssl_finished_connects
|name|value|
|----|-----|
|Name|`Ssl_finished_connects`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_finished_connects](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_finished_connects)|
|dev.mysql.com|[statvar_Ssl_finished_connects](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_finished_connects)|

## Ssl_server_not_after
|name|value|
|----|-----|
|Name|`Ssl_server_not_after`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_server_not_after](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_server_not_after)|
|dev.mysql.com|[statvar_Ssl_server_not_after](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_server_not_after)|

## Ssl_server_not_before
|name|value|
|----|-----|
|Name|`Ssl_server_not_before`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_server_not_before](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_server_not_before)|
|dev.mysql.com|[statvar_Ssl_server_not_before](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_server_not_before)|

## Ssl_session_cache_hits
|name|value|
|----|-----|
|Name|`Ssl_session_cache_hits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_session_cache_hits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_session_cache_hits)|
|dev.mysql.com|[statvar_Ssl_session_cache_hits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_session_cache_hits)|

## Ssl_session_cache_misses
|name|value|
|----|-----|
|Name|`Ssl_session_cache_misses`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_session_cache_misses](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_session_cache_misses)|
|dev.mysql.com|[statvar_Ssl_session_cache_misses](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_session_cache_misses)|

## Ssl_session_cache_mode
|name|value|
|----|-----|
|Name|`Ssl_session_cache_mode`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_session_cache_mode](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_session_cache_mode)|
|dev.mysql.com|[statvar_Ssl_session_cache_mode](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_session_cache_mode)|

## Ssl_session_cache_overflows
|name|value|
|----|-----|
|Name|`Ssl_session_cache_overflows`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_session_cache_overflows](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_session_cache_overflows)|
|dev.mysql.com|[statvar_Ssl_session_cache_overflows](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_session_cache_overflows)|

## Ssl_session_cache_size
|name|value|
|----|-----|
|Name|`Ssl_session_cache_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_session_cache_size](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_session_cache_size)|
|dev.mysql.com|[statvar_Ssl_session_cache_size](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_session_cache_size)|

## Ssl_session_cache_timeout
|name|value|
|----|-----|
|Name|`Ssl_session_cache_timeout`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_session_cache_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_session_cache_timeout)|

## Ssl_session_cache_timeouts
|name|value|
|----|-----|
|Name|`Ssl_session_cache_timeouts`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_session_cache_timeouts](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_session_cache_timeouts)|
|dev.mysql.com|[statvar_Ssl_session_cache_timeouts](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_session_cache_timeouts)|

## Ssl_sessions_reused
|name|value|
|----|-----|
|Name|`Ssl_sessions_reused`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_sessions_reused](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_sessions_reused)|
|dev.mysql.com|[statvar_Ssl_sessions_reused](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_sessions_reused)|

## Ssl_used_session_cache_entries
|name|value|
|----|-----|
|Name|`Ssl_used_session_cache_entries`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_used_session_cache_entries](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_used_session_cache_entries)|
|dev.mysql.com|[statvar_Ssl_used_session_cache_entries](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_used_session_cache_entries)|

## Ssl_verify_depth
|name|value|
|----|-----|
|Name|`Ssl_verify_depth`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_verify_depth](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_verify_depth)|
|dev.mysql.com|[statvar_Ssl_verify_depth](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_verify_depth)|

## Ssl_verify_mode
|name|value|
|----|-----|
|Name|`Ssl_verify_mode`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_verify_mode](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_verify_mode)|
|dev.mysql.com|[statvar_Ssl_verify_mode](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_verify_mode)|

## Ssl_version
|name|value|
|----|-----|
|Name|`Ssl_version`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Ssl_version](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Ssl_version)|
|dev.mysql.com|[statvar_Ssl_version](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Ssl_version)|

## Table_locks_immediate
|name|value|
|----|-----|
|Name|`Table_locks_immediate`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Table_locks_immediate](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Table_locks_immediate)|
|dev.mysql.com|[statvar_Table_locks_immediate](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Table_locks_immediate)|

## Table_locks_waited
|name|value|
|----|-----|
|Name|`Table_locks_waited`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Table_locks_waited](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Table_locks_waited)|
|dev.mysql.com|[statvar_Table_locks_waited](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Table_locks_waited)|

## Table_open_cache_hits
|name|value|
|----|-----|
|Name|`Table_open_cache_hits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Table_open_cache_hits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Table_open_cache_hits)|
|dev.mysql.com|[statvar_Table_open_cache_hits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Table_open_cache_hits)|

## Table_open_cache_misses
|name|value|
|----|-----|
|Name|`Table_open_cache_misses`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Table_open_cache_misses](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Table_open_cache_misses)|
|dev.mysql.com|[statvar_Table_open_cache_misses](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Table_open_cache_misses)|

## Table_open_cache_overflows
|name|value|
|----|-----|
|Name|`Table_open_cache_overflows`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Table_open_cache_overflows](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Table_open_cache_overflows)|
|dev.mysql.com|[statvar_Table_open_cache_overflows](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Table_open_cache_overflows)|

## Tc_log_max_pages_used
|name|value|
|----|-----|
|Name|`Tc_log_max_pages_used`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Tc_log_max_pages_used](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Tc_log_max_pages_used)|
|dev.mysql.com|[statvar_Tc_log_max_pages_used](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Tc_log_max_pages_used)|

## Tc_log_page_size
|name|value|
|----|-----|
|Name|`Tc_log_page_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Tc_log_page_size](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Tc_log_page_size)|
|dev.mysql.com|[statvar_Tc_log_page_size](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Tc_log_page_size)|

## Tc_log_page_waits
|name|value|
|----|-----|
|Name|`Tc_log_page_waits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Tc_log_page_waits](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Tc_log_page_waits)|
|dev.mysql.com|[statvar_Tc_log_page_waits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Tc_log_page_waits)|

## Telemetry_traces_supported
|name|value|
|----|-----|
|Name|`Telemetry_traces_supported`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Telemetry_traces_supported](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Telemetry_traces_supported)|

## Threads_cached
|name|value|
|----|-----|
|Name|`Threads_cached`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Threads_cached](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Threads_cached)|
|dev.mysql.com|[statvar_Threads_cached](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Threads_cached)|

## Threads_connected
|name|value|
|----|-----|
|Name|`Threads_connected`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Threads_connected](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Threads_connected)|
|dev.mysql.com|[statvar_Threads_connected](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Threads_connected)|

## Threads_created
|name|value|
|----|-----|
|Name|`Threads_created`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Threads_created](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Threads_created)|
|dev.mysql.com|[statvar_Threads_created](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Threads_created)|

## Threads_running
|name|value|
|----|-----|
|Name|`Threads_running`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Threads_running](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Threads_running)|
|dev.mysql.com|[statvar_Threads_running](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Threads_running)|

## Tls_library_version
|name|value|
|----|-----|
|Name|`Tls_library_version`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Tls_library_version](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Tls_library_version)|

## Uptime
|name|value|
|----|-----|
|Name|`Uptime`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Uptime](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Uptime)|
|dev.mysql.com|[statvar_Uptime](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Uptime)|

## Uptime_since_flush_status
|name|value|
|----|-----|
|Name|`Uptime_since_flush_status`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Uptime_since_flush_status](https://dev.mysql.com/doc/refman/8.0/en/server-status-variables.html#statvar_Uptime_since_flush_status)|
|dev.mysql.com|[statvar_Uptime_since_flush_status](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Uptime_since_flush_status)|

## group_replication_primary_member
|name|value|
|----|-----|
|Name|`group_replication_primary_member`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_group_replication_primary_member](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_group_replication_primary_member)|

## Innodb_available_undo_logs
|name|value|
|----|-----|
|Name|`Innodb_available_undo_logs`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Innodb_available_undo_logs](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Innodb_available_undo_logs)|

## Qcache_free_blocks
|name|value|
|----|-----|
|Name|`Qcache_free_blocks`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_free_blocks](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_free_blocks)|

## Qcache_free_memory
|name|value|
|----|-----|
|Name|`Qcache_free_memory`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_free_memory](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_free_memory)|

## Qcache_hits
|name|value|
|----|-----|
|Name|`Qcache_hits`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_hits](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_hits)|

## Qcache_inserts
|name|value|
|----|-----|
|Name|`Qcache_inserts`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_inserts](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_inserts)|

## Qcache_lowmem_prunes
|name|value|
|----|-----|
|Name|`Qcache_lowmem_prunes`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_lowmem_prunes](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_lowmem_prunes)|

## Qcache_not_cached
|name|value|
|----|-----|
|Name|`Qcache_not_cached`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_not_cached](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_not_cached)|

## Qcache_queries_in_cache
|name|value|
|----|-----|
|Name|`Qcache_queries_in_cache`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_queries_in_cache](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_queries_in_cache)|

## Qcache_total_blocks
|name|value|
|----|-----|
|Name|`Qcache_total_blocks`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Qcache_total_blocks](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Qcache_total_blocks)|

## Slave_heartbeat_period
|name|value|
|----|-----|
|Name|`Slave_heartbeat_period`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slave_heartbeat_period](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slave_heartbeat_period)|

## Slave_last_heartbeat
|name|value|
|----|-----|
|Name|`Slave_last_heartbeat`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slave_last_heartbeat](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slave_last_heartbeat)|

## Slave_received_heartbeats
|name|value|
|----|-----|
|Name|`Slave_received_heartbeats`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slave_received_heartbeats](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slave_received_heartbeats)|

## Slave_retried_transactions
|name|value|
|----|-----|
|Name|`Slave_retried_transactions`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slave_retried_transactions](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slave_retried_transactions)|

## Slave_running
|name|value|
|----|-----|
|Name|`Slave_running`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[statvar_Slave_running](https://dev.mysql.com/doc/refman/5.7/en/server-status-variables.html#statvar_Slave_running)|

## activate_all_roles_on_login
|name|value|
|----|-----|
|Name|`activate_all_roles_on_login`|
|Command line|`--activate-all-roles-on-login[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_activate_all_roles_on_login](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_activate_all_roles_on_login)|

## admin_address
|name|value|
|----|-----|
|Name|`admin_address`|
|Command line|`--admin-address=addr`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_address](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_address)|

## admin_port
|name|value|
|----|-----|
|Name|`admin_port`|
|Command line|`--admin-port=port_num`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`33062`|
|Dynamic|`false`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_port](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_port)|

## admin_ssl_ca
|name|value|
|----|-----|
|Name|`admin_ssl_ca`|
|Command line|`--admin-ssl-ca=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_ssl_ca](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_ssl_ca)|

## admin_ssl_capath
|name|value|
|----|-----|
|Name|`admin_ssl_capath`|
|Command line|`--admin-ssl-capath=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_ssl_capath](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_ssl_capath)|

## admin_ssl_cert
|name|value|
|----|-----|
|Name|`admin_ssl_cert`|
|Command line|`--admin-ssl-cert=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_ssl_cert](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_ssl_cert)|

## admin_ssl_cipher
|name|value|
|----|-----|
|Name|`admin_ssl_cipher`|
|Command line|`--admin-ssl-cipher=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_ssl_cipher](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_ssl_cipher)|

## admin_ssl_crl
|name|value|
|----|-----|
|Name|`admin_ssl_crl`|
|Command line|`--admin-ssl-crl=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_ssl_crl](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_ssl_crl)|

## admin_ssl_crlpath
|name|value|
|----|-----|
|Name|`admin_ssl_crlpath`|
|Command line|`--admin-ssl-crlpath=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_ssl_crlpath](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_ssl_crlpath)|

## admin_ssl_key
|name|value|
|----|-----|
|Name|`admin_ssl_key`|
|Command line|`--admin-ssl-key=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_ssl_key](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_ssl_key)|

## admin_tls_ciphersuites
|name|value|
|----|-----|
|Name|`admin_tls_ciphersuites`|
|Command line|`--admin-tls-ciphersuites=ciphersuite_list`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_tls_ciphersuites](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_tls_ciphersuites)|

## admin_tls_version
|name|value|
|----|-----|
|Name|`admin_tls_version`|
|Command line|`--admin-tls-version=protocol_list`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`TLSv1.2,TLSv1.3`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_admin_tls_version](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_admin_tls_version)|

## authentication_policy
|name|value|
|----|-----|
|Name|`authentication_policy`|
|Command line|`--authentication-policy=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`*,,`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_policy](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_authentication_policy)|

## authentication_windows_log_level
|name|value|
|----|-----|
|Name|`authentication_windows_log_level`|
|Command line|`--authentication-windows-log-level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`false`|
|Range|from: `0` to: `4`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_windows_log_level](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_authentication_windows_log_level)|
|dev.mysql.com|[sysvar_authentication_windows_log_level](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_authentication_windows_log_level)|

## authentication_windows_use_principal_name
|name|value|
|----|-----|
|Name|`authentication_windows_use_principal_name`|
|Command line|`--authentication-windows-use-principal-name[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_authentication_windows_use_principal_name](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_authentication_windows_use_principal_name)|
|dev.mysql.com|[sysvar_authentication_windows_use_principal_name](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_authentication_windows_use_principal_name)|

## autocommit
|name|value|
|----|-----|
|Name|`autocommit`|
|Command line|`--autocommit[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_autocommit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_autocommit)|
|dev.mysql.com|[sysvar_autocommit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_autocommit)|

## automatic_sp_privileges
|name|value|
|----|-----|
|Name|`automatic_sp_privileges`|
|Command line|`--automatic-sp-privileges[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_automatic_sp_privileges](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_automatic_sp_privileges)|
|dev.mysql.com|[sysvar_automatic_sp_privileges](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_automatic_sp_privileges)|

## auto_generate_certs
|name|value|
|----|-----|
|Name|`auto_generate_certs`|
|Command line|`--auto-generate-certs[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_auto_generate_certs](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_auto_generate_certs)|
|dev.mysql.com|[sysvar_auto_generate_certs](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_auto_generate_certs)|

## avoid_temporal_upgrade
|name|value|
|----|-----|
|Name|`avoid_temporal_upgrade`|
|Command line|`--avoid-temporal-upgrade[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_avoid_temporal_upgrade](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_avoid_temporal_upgrade)|
|dev.mysql.com|[sysvar_avoid_temporal_upgrade](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_avoid_temporal_upgrade)|

## back_log
|name|value|
|----|-----|
|Name|`back_log`|
|Command line|`--back-log=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`false`|
|Range|from: `1` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_back_log](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_back_log)|
|dev.mysql.com|[sysvar_back_log](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_back_log)|

## big_tables
|name|value|
|----|-----|
|Name|`big_tables`|
|Command line|`--big-tables[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_big_tables](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_big_tables)|
|dev.mysql.com|[sysvar_big_tables](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_big_tables)|

## bind_address
|name|value|
|----|-----|
|Name|`bind_address`|
|Command line|`--bind-address=addr`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`*`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_bind_address](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_bind_address)|
|dev.mysql.com|[sysvar_bind_address](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_bind_address)|

## block_encryption_mode
|name|value|
|----|-----|
|Name|`block_encryption_mode`|
|Command line|`--block-encryption-mode=#`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`aes-128-ecb`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_block_encryption_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_block_encryption_mode)|
|dev.mysql.com|[sysvar_block_encryption_mode](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_block_encryption_mode)|

## build_id
|name|value|
|----|-----|
|Name|`build_id`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_build_id](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_build_id)|

## bulk_insert_buffer_size
|name|value|
|----|-----|
|Name|`bulk_insert_buffer_size`|
|Command line|`--bulk-insert-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`8388608`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_bulk_insert_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_bulk_insert_buffer_size)|
|dev.mysql.com|[sysvar_bulk_insert_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_bulk_insert_buffer_size)|

## caching_sha2_password_digest_rounds
|name|value|
|----|-----|
|Name|`caching_sha2_password_digest_rounds`|
|Command line|`--caching-sha2-password-digest-rounds=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`5000`|
|Dynamic|`true`|
|Range|from: `5000` to: `4095000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_caching_sha2_password_digest_rounds](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_caching_sha2_password_digest_rounds)|

## caching_sha2_password_auto_generate_rsa_keys
|name|value|
|----|-----|
|Name|`caching_sha2_password_auto_generate_rsa_keys`|
|Command line|`--caching-sha2-password-auto-generate-rsa-keys[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_caching_sha2_password_auto_generate_rsa_keys](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_caching_sha2_password_auto_generate_rsa_keys)|

## caching_sha2_password_private_key_path
|name|value|
|----|-----|
|Name|`caching_sha2_password_private_key_path`|
|Command line|`--caching-sha2-password-private-key-path=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`private_key.pem`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_caching_sha2_password_private_key_path](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_caching_sha2_password_private_key_path)|

## caching_sha2_password_public_key_path
|name|value|
|----|-----|
|Name|`caching_sha2_password_public_key_path`|
|Command line|`--caching-sha2-password-public-key-path=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`public_key.pem`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_caching_sha2_password_public_key_path](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_caching_sha2_password_public_key_path)|

## character_set_client
|name|value|
|----|-----|
|Name|`character_set_client`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`utf8`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_set_client](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_set_client)|
|dev.mysql.com|[sysvar_character_set_client](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_set_client)|

## character_set_connection
|name|value|
|----|-----|
|Name|`character_set_connection`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`utf8`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_set_connection](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_set_connection)|
|dev.mysql.com|[sysvar_character_set_connection](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_set_connection)|

## character_set_database
|name|value|
|----|-----|
|Name|`character_set_database`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`latin1`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_set_database](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_set_database)|
|dev.mysql.com|[sysvar_character_set_database](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_set_database)|

## character_set_filesystem
|name|value|
|----|-----|
|Name|`character_set_filesystem`|
|Command line|`--character-set-filesystem=name`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`binary`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_set_filesystem](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_set_filesystem)|
|dev.mysql.com|[sysvar_character_set_filesystem](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_set_filesystem)|

## character_set_results
|name|value|
|----|-----|
|Name|`character_set_results`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`utf8`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_set_results](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_set_results)|
|dev.mysql.com|[sysvar_character_set_results](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_set_results)|

## character_set_server
|name|value|
|----|-----|
|Name|`character_set_server`|
|Command line|`--character-set-server=name`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`latin1`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_set_server](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_set_server)|
|dev.mysql.com|[sysvar_character_set_server](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_set_server)|

## character_set_system
|name|value|
|----|-----|
|Name|`character_set_system`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`utf8`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_set_system](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_set_system)|
|dev.mysql.com|[sysvar_character_set_system](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_set_system)|

## character_sets_dir
|name|value|
|----|-----|
|Name|`character_sets_dir`|
|Command line|`--character-sets-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_character_sets_dir](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_character_sets_dir)|
|dev.mysql.com|[sysvar_character_sets_dir](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_character_sets_dir)|

## check_proxy_users
|name|value|
|----|-----|
|Name|`check_proxy_users`|
|Command line|`--check-proxy-users[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_check_proxy_users](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_check_proxy_users)|
|dev.mysql.com|[sysvar_check_proxy_users](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_check_proxy_users)|

## collation_connection
|name|value|
|----|-----|
|Name|`collation_connection`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_collation_connection](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_collation_connection)|
|dev.mysql.com|[sysvar_collation_connection](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_collation_connection)|

## collation_database
|name|value|
|----|-----|
|Name|`collation_database`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`latin1_swedish_ci`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_collation_database](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_collation_database)|
|dev.mysql.com|[sysvar_collation_database](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_collation_database)|

## collation_server
|name|value|
|----|-----|
|Name|`collation_server`|
|Command line|`--collation-server=name`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`latin1_swedish_ci`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_collation_server](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_collation_server)|
|dev.mysql.com|[sysvar_collation_server](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_collation_server)|

## completion_type
|name|value|
|----|-----|
|Name|`completion_type`|
|Command line|`--completion-type=#`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`NO_CHAIN`|
|Dynamic|`true`|
|Valid value(s)|`NO_CHAIN`, `CHAIN`, `RELEASE`, `0`, `1`, `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_completion_type](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_completion_type)|
|dev.mysql.com|[sysvar_completion_type](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_completion_type)|

## component_scheduler.enabled
|name|value|
|----|-----|
|Name|`component_scheduler.enabled`|
|Command line|`--component-scheduler.enabled[=value]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_component_scheduler.enabled](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_component_scheduler.enabled)|

## concurrent_insert
|name|value|
|----|-----|
|Name|`concurrent_insert`|
|Command line|`--concurrent-insert[=value]`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`AUTO`|
|Dynamic|`true`|
|Valid value(s)|`NEVER`, `AUTO`, `ALWAYS`, `0`, `1`, `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_concurrent_insert](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_concurrent_insert)|
|dev.mysql.com|[sysvar_concurrent_insert](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_concurrent_insert)|

## connect_timeout
|name|value|
|----|-----|
|Name|`connect_timeout`|
|Command line|`--connect-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `2` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_connect_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_connect_timeout)|
|dev.mysql.com|[sysvar_connect_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_connect_timeout)|

## connection_memory_chunk_size
|name|value|
|----|-----|
|Name|`connection_memory_chunk_size`|
|Command line|`--connection-memory-chunk-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`8192`|
|Dynamic|`true`|
|Range|from: `0` to: `536870912`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_connection_memory_chunk_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_connection_memory_chunk_size)|

## connection_memory_limit
|name|value|
|----|-----|
|Name|`connection_memory_limit`|
|Command line|`--connection-memory-limit=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`18446744073709551615`|
|Dynamic|`true`|
|Range|from: `2097152` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_connection_memory_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_connection_memory_limit)|

## create_admin_listener_thread
|name|value|
|----|-----|
|Name|`create_admin_listener_thread`|
|Command line|`--create-admin-listener-thread[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_create_admin_listener_thread](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_create_admin_listener_thread)|

## cte_max_recursion_depth
|name|value|
|----|-----|
|Name|`cte_max_recursion_depth`|
|Command line|`--cte-max-recursion-depth=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1000`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_cte_max_recursion_depth](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_cte_max_recursion_depth)|

## debug_sync
|name|value|
|----|-----|
|Name|`debug_sync`|
|Type of variable|`string`|
|Scope|`session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_debug_sync](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_debug_sync)|
|dev.mysql.com|[sysvar_debug_sync](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_debug_sync)|

## default_authentication_plugin
|name|value|
|----|-----|
|Name|`default_authentication_plugin`|
|Command line|`--default-authentication-plugin=plugin_name`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`mysql_native_password`|
|Dynamic|`false`|
|Valid value(s)|`mysql_native_password`, `sha256_password`, `caching_sha2_password`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_default_authentication_plugin](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_default_authentication_plugin)|
|dev.mysql.com|[sysvar_default_authentication_plugin](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_default_authentication_plugin)|

## default_collation_for_utf8mb4
|name|value|
|----|-----|
|Name|`default_collation_for_utf8mb4`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`utf8mb4_0900_ai_ci`|
|Dynamic|`true`|
|Valid value(s)|`utf8mb4_0900_ai_ci`, `utf8mb4_general_ci`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_default_collation_for_utf8mb4](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_default_collation_for_utf8mb4)|

## default_password_lifetime
|name|value|
|----|-----|
|Name|`default_password_lifetime`|
|Command line|`--default-password-lifetime=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_default_password_lifetime](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_default_password_lifetime)|
|dev.mysql.com|[sysvar_default_password_lifetime](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_default_password_lifetime)|

## default_storage_engine
|name|value|
|----|-----|
|Name|`default_storage_engine`|
|Command line|`--default-storage-engine=name`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`InnoDB`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_default_storage_engine](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_default_storage_engine)|
|dev.mysql.com|[sysvar_default_storage_engine](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_default_storage_engine)|

## default_table_encryption
|name|value|
|----|-----|
|Name|`default_table_encryption`|
|Command line|`--default-table-encryption[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_default_table_encryption](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_default_table_encryption)|

## default_tmp_storage_engine
|name|value|
|----|-----|
|Name|`default_tmp_storage_engine`|
|Command line|`--default-tmp-storage-engine=name`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`InnoDB`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_default_tmp_storage_engine](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_default_tmp_storage_engine)|
|dev.mysql.com|[sysvar_default_tmp_storage_engine](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_default_tmp_storage_engine)|

## default_week_format
|name|value|
|----|-----|
|Name|`default_week_format`|
|Command line|`--default-week-format=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `7`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_default_week_format](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_default_week_format)|
|dev.mysql.com|[sysvar_default_week_format](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_default_week_format)|

## delay_key_write
|name|value|
|----|-----|
|Name|`delay_key_write`|
|Command line|`--delay-key-write[={OFF|ON|ALL}]`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `ON`, `ALL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_delay_key_write](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_delay_key_write)|
|dev.mysql.com|[sysvar_delay_key_write](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_delay_key_write)|

## delayed_insert_limit
|name|value|
|----|-----|
|Name|`delayed_insert_limit`|
|Command line|`--delayed-insert-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`true`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_delayed_insert_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_delayed_insert_limit)|
|dev.mysql.com|[sysvar_delayed_insert_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_delayed_insert_limit)|

## delayed_insert_timeout
|name|value|
|----|-----|
|Name|`delayed_insert_timeout`|
|Command line|`--delayed-insert-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`300`|
|Dynamic|`true`|
|Range|from: `1` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_delayed_insert_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_delayed_insert_timeout)|
|dev.mysql.com|[sysvar_delayed_insert_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_delayed_insert_timeout)|

## delayed_queue_size
|name|value|
|----|-----|
|Name|`delayed_queue_size`|
|Command line|`--delayed-queue-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1000`|
|Dynamic|`true`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_delayed_queue_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_delayed_queue_size)|
|dev.mysql.com|[sysvar_delayed_queue_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_delayed_queue_size)|

## disabled_storage_engines
|name|value|
|----|-----|
|Name|`disabled_storage_engines`|
|Command line|`--disabled-storage-engines=engine[,engine]...`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_disabled_storage_engines](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_disabled_storage_engines)|
|dev.mysql.com|[sysvar_disabled_storage_engines](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_disabled_storage_engines)|

## disconnect_on_expired_password
|name|value|
|----|-----|
|Name|`disconnect_on_expired_password`|
|Command line|`--disconnect-on-expired-password[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_disconnect_on_expired_password](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_disconnect_on_expired_password)|
|dev.mysql.com|[sysvar_disconnect_on_expired_password](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_disconnect_on_expired_password)|

## div_precision_increment
|name|value|
|----|-----|
|Name|`div_precision_increment`|
|Command line|`--div-precision-increment=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`4`|
|Dynamic|`true`|
|Range|from: `0` to: `30`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_div_precision_increment](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_div_precision_increment)|
|dev.mysql.com|[sysvar_div_precision_increment](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_div_precision_increment)|

## dragnet.log_error_filter_rules
|name|value|
|----|-----|
|Name|`dragnet.log_error_filter_rules`|
|Command line|`--dragnet.log-error-filter-rules=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`IF prio>=INFORMATION THEN drop. IF EXISTS source_line THEN unset source_line.`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_dragnet.log_error_filter_rules](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_dragnet.log_error_filter_rules)|

## enterprise_encryption.maximum_rsa_key_size
|name|value|
|----|-----|
|Name|`enterprise_encryption.maximum_rsa_key_size`|
|Command line|`--enterprise-encryption.maximum-rsa-key-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4096`|
|Dynamic|`true`|
|Range|from: `2048` to: `16384`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_enterprise_encryption.maximum_rsa_key_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_enterprise_encryption.maximum_rsa_key_size)|

## enterprise_encryption.rsa_support_legacy_padding
|name|value|
|----|-----|
|Name|`enterprise_encryption.rsa_support_legacy_padding`|
|Command line|`--enterprise-encryption.rsa_support_legacy_padding[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_enterprise_encryption.rsa_support_legacy_padding](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_enterprise_encryption.rsa_support_legacy_padding)|

## end_markers_in_json
|name|value|
|----|-----|
|Name|`end_markers_in_json`|
|Command line|`--end-markers-in-json[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_end_markers_in_json](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_end_markers_in_json)|
|dev.mysql.com|[sysvar_end_markers_in_json](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_end_markers_in_json)|

## eq_range_index_dive_limit
|name|value|
|----|-----|
|Name|`eq_range_index_dive_limit`|
|Command line|`--eq-range-index-dive-limit=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`200`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_eq_range_index_dive_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_eq_range_index_dive_limit)|
|dev.mysql.com|[sysvar_eq_range_index_dive_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_eq_range_index_dive_limit)|

## event_scheduler
|name|value|
|----|-----|
|Name|`event_scheduler`|
|Command line|`--event-scheduler[=value]`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `ON`, `DISABLED`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_event_scheduler](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_event_scheduler)|
|dev.mysql.com|[sysvar_event_scheduler](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_event_scheduler)|

## explain_format
|name|value|
|----|-----|
|Name|`explain_format`|
|Command line|`--explain-format=format`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`TRADITIONAL`|
|Dynamic|`true`|
|Valid value(s)|`TRADITIONAL`, `JSON`, `TREE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_explain_format](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_explain_format)|

## explicit_defaults_for_timestamp
|name|value|
|----|-----|
|Name|`explicit_defaults_for_timestamp`|
|Command line|`--explicit-defaults-for-timestamp[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_explicit_defaults_for_timestamp](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_explicit_defaults_for_timestamp)|
|dev.mysql.com|[sysvar_explicit_defaults_for_timestamp](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_explicit_defaults_for_timestamp)|

## external_user
|name|value|
|----|-----|
|Name|`external_user`|
|Type of variable|`string`|
|Scope|`session`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_external_user](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_external_user)|
|dev.mysql.com|[sysvar_external_user](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_external_user)|

## flush_time
|name|value|
|----|-----|
|Name|`flush_time`|
|Command line|`--flush-time=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_flush_time](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_flush_time)|
|dev.mysql.com|[sysvar_flush_time](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_flush_time)|

## foreign_key_checks
|name|value|
|----|-----|
|Name|`foreign_key_checks`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_foreign_key_checks](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_foreign_key_checks)|
|dev.mysql.com|[sysvar_foreign_key_checks](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_foreign_key_checks)|

## ft_boolean_syntax
|name|value|
|----|-----|
|Name|`ft_boolean_syntax`|
|Command line|`--ft-boolean-syntax=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`+ -><()~*:""&|`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ft_boolean_syntax](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ft_boolean_syntax)|
|dev.mysql.com|[sysvar_ft_boolean_syntax](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ft_boolean_syntax)|

## ft_max_word_len
|name|value|
|----|-----|
|Name|`ft_max_word_len`|
|Command line|`--ft-max-word-len=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`84`|
|Dynamic|`false`|
|Range|from: `10` to: `84`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ft_max_word_len](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ft_max_word_len)|
|dev.mysql.com|[sysvar_ft_max_word_len](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ft_max_word_len)|

## ft_min_word_len
|name|value|
|----|-----|
|Name|`ft_min_word_len`|
|Command line|`--ft-min-word-len=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4`|
|Dynamic|`false`|
|Range|from: `1` to: `82`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ft_min_word_len](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ft_min_word_len)|
|dev.mysql.com|[sysvar_ft_min_word_len](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ft_min_word_len)|

## ft_query_expansion_limit
|name|value|
|----|-----|
|Name|`ft_query_expansion_limit`|
|Command line|`--ft-query-expansion-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`20`|
|Dynamic|`false`|
|Range|from: `0` to: `1000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ft_query_expansion_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ft_query_expansion_limit)|
|dev.mysql.com|[sysvar_ft_query_expansion_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ft_query_expansion_limit)|

## ft_stopword_file
|name|value|
|----|-----|
|Name|`ft_stopword_file`|
|Command line|`--ft-stopword-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ft_stopword_file](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ft_stopword_file)|
|dev.mysql.com|[sysvar_ft_stopword_file](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ft_stopword_file)|

## general_log
|name|value|
|----|-----|
|Name|`general_log`|
|Command line|`--general-log[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_general_log](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_general_log)|
|dev.mysql.com|[sysvar_general_log](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_general_log)|

## general_log_file
|name|value|
|----|-----|
|Name|`general_log_file`|
|Command line|`--general-log-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`host_name.log`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_general_log_file](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_general_log_file)|
|dev.mysql.com|[sysvar_general_log_file](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_general_log_file)|

## generated_random_password_length
|name|value|
|----|-----|
|Name|`generated_random_password_length`|
|Command line|`--generated-random-password-length=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`20`|
|Dynamic|`true`|
|Range|from: `5` to: `255`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_generated_random_password_length](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_generated_random_password_length)|

## global_connection_memory_limit
|name|value|
|----|-----|
|Name|`global_connection_memory_limit`|
|Command line|`--global-connection-memory-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`18446744073709551615`|
|Dynamic|`true`|
|Range|from: `16777216` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_global_connection_memory_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_global_connection_memory_limit)|

## global_connection_memory_tracking
|name|value|
|----|-----|
|Name|`global_connection_memory_tracking`|
|Command line|`--global-connection-memory-tracking={TRUE|FALSE}`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`FALSE`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_global_connection_memory_tracking](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_global_connection_memory_tracking)|

## group_concat_max_len
|name|value|
|----|-----|
|Name|`group_concat_max_len`|
|Command line|`--group-concat-max-len=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1024`|
|Dynamic|`true`|
|Range|from: `4`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_group_concat_max_len](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_group_concat_max_len)|
|dev.mysql.com|[sysvar_group_concat_max_len](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_group_concat_max_len)|

## have_ssl
|name|value|
|----|-----|
|Name|`have_ssl`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|
|Valid value(s)|`YES`, `DISABLED`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_ssl](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_ssl)|
|dev.mysql.com|[sysvar_have_ssl](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_ssl)|

## have_statement_timeout
|name|value|
|----|-----|
|Name|`have_statement_timeout`|
|Type of variable|`boolean`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_statement_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_statement_timeout)|
|dev.mysql.com|[sysvar_have_statement_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_statement_timeout)|

## histogram_generation_max_mem_size
|name|value|
|----|-----|
|Name|`histogram_generation_max_mem_size`|
|Command line|`--histogram-generation-max-mem-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`20000000`|
|Dynamic|`true`|
|Range|from: `1000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_histogram_generation_max_mem_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_histogram_generation_max_mem_size)|

## host_cache_size
|name|value|
|----|-----|
|Name|`host_cache_size`|
|Command line|`--host-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`true`|
|Range|from: `0` to: `65536`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_host_cache_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_host_cache_size)|
|dev.mysql.com|[sysvar_host_cache_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_host_cache_size)|

## hostname
|name|value|
|----|-----|
|Name|`hostname`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_hostname](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_hostname)|
|dev.mysql.com|[sysvar_hostname](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_hostname)|

## init_connect
|name|value|
|----|-----|
|Name|`init_connect`|
|Command line|`--init-connect=name`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_init_connect](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_init_connect)|
|dev.mysql.com|[sysvar_init_connect](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_init_connect)|

## information_schema_stats_expiry
|name|value|
|----|-----|
|Name|`information_schema_stats_expiry`|
|Command line|`--information-schema-stats-expiry=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`86400`|
|Dynamic|`true`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_information_schema_stats_expiry](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_information_schema_stats_expiry)|

## interactive_timeout
|name|value|
|----|-----|
|Name|`interactive_timeout`|
|Command line|`--interactive-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`28800`|
|Dynamic|`true`|
|Range|from: `1` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_interactive_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_interactive_timeout)|
|dev.mysql.com|[sysvar_interactive_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_interactive_timeout)|

## internal_tmp_mem_storage_engine
|name|value|
|----|-----|
|Name|`internal_tmp_mem_storage_engine`|
|Command line|`--internal-tmp-mem-storage-engine=#`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`TempTable`|
|Dynamic|`true`|
|Valid value(s)|`MEMORY`, `TempTable`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_internal_tmp_mem_storage_engine](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_internal_tmp_mem_storage_engine)|

## join_buffer_size
|name|value|
|----|-----|
|Name|`join_buffer_size`|
|Command line|`--join-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`262144`|
|Dynamic|`true`|
|Range|from: `128`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_join_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_join_buffer_size)|
|dev.mysql.com|[sysvar_join_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_join_buffer_size)|

## keep_files_on_create
|name|value|
|----|-----|
|Name|`keep_files_on_create`|
|Command line|`--keep-files-on-create[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_keep_files_on_create](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_keep_files_on_create)|
|dev.mysql.com|[sysvar_keep_files_on_create](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_keep_files_on_create)|

## key_buffer_size
|name|value|
|----|-----|
|Name|`key_buffer_size`|
|Command line|`--key-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8388608`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_key_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_key_buffer_size)|
|dev.mysql.com|[sysvar_key_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_key_buffer_size)|

## key_cache_age_threshold
|name|value|
|----|-----|
|Name|`key_cache_age_threshold`|
|Command line|`--key-cache-age-threshold=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`300`|
|Dynamic|`true`|
|Range|from: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_key_cache_age_threshold](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_key_cache_age_threshold)|
|dev.mysql.com|[sysvar_key_cache_age_threshold](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_key_cache_age_threshold)|

## key_cache_block_size
|name|value|
|----|-----|
|Name|`key_cache_block_size`|
|Command line|`--key-cache-block-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1024`|
|Dynamic|`true`|
|Range|from: `512` to: `16384`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_key_cache_block_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_key_cache_block_size)|
|dev.mysql.com|[sysvar_key_cache_block_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_key_cache_block_size)|

## key_cache_division_limit
|name|value|
|----|-----|
|Name|`key_cache_division_limit`|
|Command line|`--key-cache-division-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`true`|
|Range|from: `1` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_key_cache_division_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_key_cache_division_limit)|
|dev.mysql.com|[sysvar_key_cache_division_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_key_cache_division_limit)|

## large_files_support
|name|value|
|----|-----|
|Name|`large_files_support`|
|Type of variable|`boolean`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_large_files_support](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_large_files_support)|
|dev.mysql.com|[sysvar_large_files_support](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_large_files_support)|

## large_page_size
|name|value|
|----|-----|
|Name|`large_page_size`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`false`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_large_page_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_large_page_size)|
|dev.mysql.com|[sysvar_large_page_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_large_page_size)|

## lc_time_names
|name|value|
|----|-----|
|Name|`lc_time_names`|
|Command line|`--lc-time-names=value`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_lc_time_names](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_lc_time_names)|
|dev.mysql.com|[sysvar_lc_time_names](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_lc_time_names)|

## license
|name|value|
|----|-----|
|Name|`license`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`GPL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_license](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_license)|
|dev.mysql.com|[sysvar_license](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_license)|

## local_infile
|name|value|
|----|-----|
|Name|`local_infile`|
|Command line|`--local-infile[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_local_infile](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_local_infile)|
|dev.mysql.com|[sysvar_local_infile](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_local_infile)|

## lock_wait_timeout
|name|value|
|----|-----|
|Name|`lock_wait_timeout`|
|Command line|`--lock-wait-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`31536000`|
|Dynamic|`true`|
|Range|from: `1` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_lock_wait_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_lock_wait_timeout)|
|dev.mysql.com|[sysvar_lock_wait_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_lock_wait_timeout)|

## locked_in_memory
|name|value|
|----|-----|
|Name|`locked_in_memory`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_locked_in_memory](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_locked_in_memory)|
|dev.mysql.com|[sysvar_locked_in_memory](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_locked_in_memory)|

## log_error_services
|name|value|
|----|-----|
|Name|`log_error_services`|
|Command line|`--log-error-services=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`log_filter_internal; log_sink_internal`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_error_services](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_error_services)|

## log_error_suppression_list
|name|value|
|----|-----|
|Name|`log_error_suppression_list`|
|Command line|`--log-error-suppression-list=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_error_suppression_list](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_error_suppression_list)|

## log_error_verbosity
|name|value|
|----|-----|
|Name|`log_error_verbosity`|
|Command line|`--log-error-verbosity=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`true`|
|Range|from: `1` to: `3`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_error_verbosity](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_error_verbosity)|
|dev.mysql.com|[sysvar_log_error_verbosity](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_error_verbosity)|

## log_output
|name|value|
|----|-----|
|Name|`log_output`|
|Command line|`--log-output=name`|
|Type of variable|`set`|
|Scope|`global`|
|Default value|`FILE`|
|Dynamic|`true`|
|Valid value(s)|`TABLE`, `FILE`, `NONE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_output](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_output)|
|dev.mysql.com|[sysvar_log_output](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_output)|

## log_queries_not_using_indexes
|name|value|
|----|-----|
|Name|`log_queries_not_using_indexes`|
|Command line|`--log-queries-not-using-indexes[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_queries_not_using_indexes](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_queries_not_using_indexes)|
|dev.mysql.com|[sysvar_log_queries_not_using_indexes](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_queries_not_using_indexes)|

## log_slow_admin_statements
|name|value|
|----|-----|
|Name|`log_slow_admin_statements`|
|Command line|`--log-slow-admin-statements[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_slow_admin_statements](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_slow_admin_statements)|
|dev.mysql.com|[sysvar_log_slow_admin_statements](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_slow_admin_statements)|

## log_slow_extra
|name|value|
|----|-----|
|Name|`log_slow_extra`|
|Command line|`--log-slow-extra[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_slow_extra](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_slow_extra)|

## log_timestamps
|name|value|
|----|-----|
|Name|`log_timestamps`|
|Command line|`--log-timestamps=#`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`UTC`|
|Dynamic|`true`|
|Valid value(s)|`UTC`, `SYSTEM`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_timestamps](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_timestamps)|
|dev.mysql.com|[sysvar_log_timestamps](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_timestamps)|

## log_throttle_queries_not_using_indexes
|name|value|
|----|-----|
|Name|`log_throttle_queries_not_using_indexes`|
|Command line|`--log-throttle-queries-not-using-indexes=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_throttle_queries_not_using_indexes](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_throttle_queries_not_using_indexes)|
|dev.mysql.com|[sysvar_log_throttle_queries_not_using_indexes](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_throttle_queries_not_using_indexes)|

## long_query_time
|name|value|
|----|-----|
|Name|`long_query_time`|
|Command line|`--long-query-time=#`|
|Type of variable|`numeric`|
|Scope|`global`, `session`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_long_query_time](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_long_query_time)|
|dev.mysql.com|[sysvar_long_query_time](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_long_query_time)|

## low_priority_updates
|name|value|
|----|-----|
|Name|`low_priority_updates`|
|Command line|`--low-priority-updates[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_low_priority_updates](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_low_priority_updates)|
|dev.mysql.com|[sysvar_low_priority_updates](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_low_priority_updates)|

## lower_case_file_system
|name|value|
|----|-----|
|Name|`lower_case_file_system`|
|Type of variable|`boolean`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_lower_case_file_system](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_lower_case_file_system)|
|dev.mysql.com|[sysvar_lower_case_file_system](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_lower_case_file_system)|

## lower_case_table_names
|name|value|
|----|-----|
|Name|`lower_case_table_names`|
|Command line|`--lower-case-table-names[=#]`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`false`|
|Range|from: `0` to: `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_lower_case_table_names](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_lower_case_table_names)|
|dev.mysql.com|[sysvar_lower_case_table_names](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_lower_case_table_names)|

## mandatory_roles
|name|value|
|----|-----|
|Name|`mandatory_roles`|
|Command line|`--mandatory-roles=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mandatory_roles](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_mandatory_roles)|

## max_allowed_packet
|name|value|
|----|-----|
|Name|`max_allowed_packet`|
|Command line|`--max-allowed-packet=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Dynamic|`true`|
|Range|from: `1024` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_allowed_packet](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_allowed_packet)|
|dev.mysql.com|[sysvar_max_allowed_packet](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_allowed_packet)|

## max_connect_errors
|name|value|
|----|-----|
|Name|`max_connect_errors`|
|Command line|`--max-connect-errors=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`true`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_connect_errors](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_connect_errors)|
|dev.mysql.com|[sysvar_max_connect_errors](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_connect_errors)|

## max_delayed_threads
|name|value|
|----|-----|
|Name|`max_delayed_threads`|
|Command line|`--max-delayed-threads=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`20`|
|Dynamic|`true`|
|Range|from: `0` to: `16384`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_delayed_threads](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_delayed_threads)|
|dev.mysql.com|[sysvar_max_delayed_threads](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_delayed_threads)|

## max_digest_length
|name|value|
|----|-----|
|Name|`max_digest_length`|
|Command line|`--max-digest-length=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1024`|
|Dynamic|`false`|
|Range|from: `0` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_digest_length](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_digest_length)|
|dev.mysql.com|[sysvar_max_digest_length](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_digest_length)|

## max_error_count
|name|value|
|----|-----|
|Name|`max_error_count`|
|Command line|`--max-error-count=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Dynamic|`true`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_error_count](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_error_count)|
|dev.mysql.com|[sysvar_max_error_count](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_error_count)|

## max_execution_time
|name|value|
|----|-----|
|Name|`max_execution_time`|
|Command line|`--max-execution-time=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_execution_time](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_execution_time)|
|dev.mysql.com|[sysvar_max_execution_time](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_execution_time)|

## max_heap_table_size
|name|value|
|----|-----|
|Name|`max_heap_table_size`|
|Command line|`--max-heap-table-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`16777216`|
|Dynamic|`true`|
|Range|from: `16384`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_heap_table_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_heap_table_size)|
|dev.mysql.com|[sysvar_max_heap_table_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_heap_table_size)|

## max_insert_delayed_threads
|name|value|
|----|-----|
|Name|`max_insert_delayed_threads`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `20` to: `16384`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_insert_delayed_threads](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_insert_delayed_threads)|
|dev.mysql.com|[sysvar_max_insert_delayed_threads](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_insert_delayed_threads)|

## max_join_size
|name|value|
|----|-----|
|Name|`max_join_size`|
|Command line|`--max-join-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`18446744073709551615`|
|Dynamic|`true`|
|Range|from: `1` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_join_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_join_size)|
|dev.mysql.com|[sysvar_max_join_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_join_size)|

## max_length_for_sort_data
|name|value|
|----|-----|
|Name|`max_length_for_sort_data`|
|Command line|`--max-length-for-sort-data=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Dynamic|`true`|
|Range|from: `4` to: `8388608`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_length_for_sort_data](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_length_for_sort_data)|
|dev.mysql.com|[sysvar_max_length_for_sort_data](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_length_for_sort_data)|

## max_points_in_geometry
|name|value|
|----|-----|
|Name|`max_points_in_geometry`|
|Command line|`--max-points-in-geometry=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`65536`|
|Dynamic|`true`|
|Range|from: `3` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_points_in_geometry](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_points_in_geometry)|
|dev.mysql.com|[sysvar_max_points_in_geometry](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_points_in_geometry)|

## max_prepared_stmt_count
|name|value|
|----|-----|
|Name|`max_prepared_stmt_count`|
|Command line|`--max-prepared-stmt-count=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`16382`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_prepared_stmt_count](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_prepared_stmt_count)|
|dev.mysql.com|[sysvar_max_prepared_stmt_count](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_prepared_stmt_count)|

## max_seeks_for_key
|name|value|
|----|-----|
|Name|`max_seeks_for_key`|
|Command line|`--max-seeks-for-key=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Dynamic|`true`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_seeks_for_key](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_seeks_for_key)|
|dev.mysql.com|[sysvar_max_seeks_for_key](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_seeks_for_key)|

## max_sort_length
|name|value|
|----|-----|
|Name|`max_sort_length`|
|Command line|`--max-sort-length=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1024`|
|Dynamic|`true`|
|Range|from: `4` to: `8388608`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_sort_length](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_sort_length)|
|dev.mysql.com|[sysvar_max_sort_length](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_sort_length)|

## max_sp_recursion_depth
|name|value|
|----|-----|
|Name|`max_sp_recursion_depth`|
|Command line|`--max-sp-recursion-depth[=#]`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `255`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_sp_recursion_depth](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_sp_recursion_depth)|
|dev.mysql.com|[sysvar_max_sp_recursion_depth](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_sp_recursion_depth)|

## max_user_connections
|name|value|
|----|-----|
|Name|`max_user_connections`|
|Command line|`--max-user-connections=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_user_connections](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_user_connections)|
|dev.mysql.com|[sysvar_max_user_connections](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_user_connections)|

## max_write_lock_count
|name|value|
|----|-----|
|Name|`max_write_lock_count`|
|Command line|`--max-write-lock-count=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`true`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_write_lock_count](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_max_write_lock_count)|
|dev.mysql.com|[sysvar_max_write_lock_count](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_write_lock_count)|

## mecab_rc_file
|name|value|
|----|-----|
|Name|`mecab_rc_file`|
|Command line|`--mecab-rc-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mecab_rc_file](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_mecab_rc_file)|
|dev.mysql.com|[sysvar_mecab_rc_file](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_mecab_rc_file)|

## min_examined_row_limit
|name|value|
|----|-----|
|Name|`min_examined_row_limit`|
|Command line|`--min-examined-row-limit=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_min_examined_row_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_min_examined_row_limit)|
|dev.mysql.com|[sysvar_min_examined_row_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_min_examined_row_limit)|

## myisam_data_pointer_size
|name|value|
|----|-----|
|Name|`myisam_data_pointer_size`|
|Command line|`--myisam-data-pointer-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`6`|
|Dynamic|`true`|
|Range|from: `2` to: `7`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_data_pointer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_data_pointer_size)|
|dev.mysql.com|[sysvar_myisam_data_pointer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_data_pointer_size)|

## myisam_max_sort_file_size
|name|value|
|----|-----|
|Name|`myisam_max_sort_file_size`|
|Command line|`--myisam-max-sort-file-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_max_sort_file_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_max_sort_file_size)|
|dev.mysql.com|[sysvar_myisam_max_sort_file_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_max_sort_file_size)|

## myisam_mmap_size
|name|value|
|----|-----|
|Name|`myisam_mmap_size`|
|Command line|`--myisam-mmap-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`false`|
|Range|from: `7`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_mmap_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_mmap_size)|
|dev.mysql.com|[sysvar_myisam_mmap_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_mmap_size)|

## myisam_recover_options
|name|value|
|----|-----|
|Name|`myisam_recover_options`|
|Command line|`--myisam-recover-options[=list]`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|
|Valid value(s)|`OFF`, `DEFAULT`, `BACKUP`, `FORCE`, `QUICK`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_recover_options](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_recover_options)|
|dev.mysql.com|[sysvar_myisam_recover_options](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_recover_options)|

## myisam_sort_buffer_size
|name|value|
|----|-----|
|Name|`myisam_sort_buffer_size`|
|Command line|`--myisam-sort-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`8388608`|
|Dynamic|`true`|
|Range|from: `4096`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_sort_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_sort_buffer_size)|
|dev.mysql.com|[sysvar_myisam_sort_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_sort_buffer_size)|

## myisam_stats_method
|name|value|
|----|-----|
|Name|`myisam_stats_method`|
|Command line|`--myisam-stats-method=name`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`nulls_unequal`|
|Dynamic|`true`|
|Valid value(s)|`nulls_unequal`, `nulls_equal`, `nulls_ignored`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_stats_method](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_stats_method)|
|dev.mysql.com|[sysvar_myisam_stats_method](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_stats_method)|

## myisam_use_mmap
|name|value|
|----|-----|
|Name|`myisam_use_mmap`|
|Command line|`--myisam-use-mmap[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_use_mmap](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_use_mmap)|
|dev.mysql.com|[sysvar_myisam_use_mmap](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_use_mmap)|

## mysql_native_password_proxy_users
|name|value|
|----|-----|
|Name|`mysql_native_password_proxy_users`|
|Command line|`--mysql-native-password-proxy-users[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysql_native_password_proxy_users](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_mysql_native_password_proxy_users)|
|dev.mysql.com|[sysvar_mysql_native_password_proxy_users](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_mysql_native_password_proxy_users)|

## named_pipe
|name|value|
|----|-----|
|Name|`named_pipe`|
|Command line|`--named-pipe[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_named_pipe](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_named_pipe)|
|dev.mysql.com|[sysvar_named_pipe](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_named_pipe)|

## named_pipe_full_access_group
|name|value|
|----|-----|
|Name|`named_pipe_full_access_group`|
|Command line|`--named-pipe-full-access-group=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`false`|
|Valid value(s)|`empty string`, `valid Windows local group name`, `*everyone*`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_named_pipe_full_access_group](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_named_pipe_full_access_group)|
|dev.mysql.com|[sysvar_named_pipe_full_access_group](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_named_pipe_full_access_group)|

## net_buffer_length
|name|value|
|----|-----|
|Name|`net_buffer_length`|
|Command line|`--net-buffer-length=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`16384`|
|Dynamic|`true`|
|Range|from: `1024` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_net_buffer_length](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_net_buffer_length)|
|dev.mysql.com|[sysvar_net_buffer_length](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_net_buffer_length)|

## net_read_timeout
|name|value|
|----|-----|
|Name|`net_read_timeout`|
|Command line|`--net-read-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`30`|
|Dynamic|`true`|
|Range|from: `1` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_net_read_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_net_read_timeout)|
|dev.mysql.com|[sysvar_net_read_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_net_read_timeout)|

## net_retry_count
|name|value|
|----|-----|
|Name|`net_retry_count`|
|Command line|`--net-retry-count=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`10`|
|Dynamic|`true`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_net_retry_count](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_net_retry_count)|
|dev.mysql.com|[sysvar_net_retry_count](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_net_retry_count)|

## net_write_timeout
|name|value|
|----|-----|
|Name|`net_write_timeout`|
|Command line|`--net-write-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`60`|
|Dynamic|`true`|
|Range|from: `1` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_net_write_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_net_write_timeout)|
|dev.mysql.com|[sysvar_net_write_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_net_write_timeout)|

## new
|name|value|
|----|-----|
|Name|`new`|
|Command line|`--new[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_new](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_new)|
|dev.mysql.com|[sysvar_new](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_new)|

## ngram_token_size
|name|value|
|----|-----|
|Name|`ngram_token_size`|
|Command line|`--ngram-token-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`false`|
|Range|from: `1` to: `10`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ngram_token_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ngram_token_size)|
|dev.mysql.com|[sysvar_ngram_token_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ngram_token_size)|

## offline_mode
|name|value|
|----|-----|
|Name|`offline_mode`|
|Command line|`--offline-mode[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_offline_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_offline_mode)|
|dev.mysql.com|[sysvar_offline_mode](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_offline_mode)|

## old
|name|value|
|----|-----|
|Name|`old`|
|Command line|`--old[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_old](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_old)|
|dev.mysql.com|[sysvar_old](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_old)|

## old_alter_table
|name|value|
|----|-----|
|Name|`old_alter_table`|
|Command line|`--old-alter-table[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_old_alter_table](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_old_alter_table)|
|dev.mysql.com|[sysvar_old_alter_table](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_old_alter_table)|

## open_files_limit
|name|value|
|----|-----|
|Name|`open_files_limit`|
|Command line|`--open-files-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`5000, with possible adjustment`|
|Dynamic|`false`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_open_files_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_open_files_limit)|
|dev.mysql.com|[sysvar_open_files_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_open_files_limit)|

## optimizer_prune_level
|name|value|
|----|-----|
|Name|`optimizer_prune_level`|
|Command line|`--optimizer-prune-level=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `0` to: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_prune_level](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_prune_level)|
|dev.mysql.com|[sysvar_optimizer_prune_level](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_prune_level)|

## optimizer_search_depth
|name|value|
|----|-----|
|Name|`optimizer_search_depth`|
|Command line|`--optimizer-search-depth=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`62`|
|Dynamic|`true`|
|Range|from: `0` to: `62`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_search_depth](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_search_depth)|
|dev.mysql.com|[sysvar_optimizer_search_depth](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_search_depth)|

## optimizer_switch
|name|value|
|----|-----|
|Name|`optimizer_switch`|
|Command line|`--optimizer-switch=value`|
|Type of variable|`set`|
|Scope|`global`, `session`|
|Dynamic|`true`|
|Valid value(s)|`batched_key_access={on|off}`, `block_nested_loop={on|off}`, `condition_fanout_filter={on|off}`, `derived_condition_pushdown={on|off}`, `derived_merge={on|off}`, `duplicateweedout={on|off}`, `engine_condition_pushdown={on|off}`, `firstmatch={on|off}`, `hash_join={on|off}`, `index_condition_pushdown={on|off}`, `index_merge={on|off}`, `index_merge_intersection={on|off}`, `index_merge_sort_union={on|off}`, `index_merge_union={on|off}`, `loosescan={on|off}`, `materialization={on|off}`, `mrr={on|off}`, `mrr_cost_based={on|off}`, `prefer_ordering_index={on|off}`, `semijoin={on|off}`, `skip_scan={on|off}`, `subquery_materialization_cost_based={on|off}`, `subquery_to_derived={on|off}`, `use_index_extensions={on|off}`, `use_invisible_indexes={on|off}`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_switch](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_switch)|
|dev.mysql.com|[sysvar_optimizer_switch](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_switch)|

## optimizer_trace
|name|value|
|----|-----|
|Name|`optimizer_trace`|
|Command line|`--optimizer-trace=value`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_trace](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_trace)|
|dev.mysql.com|[sysvar_optimizer_trace](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_trace)|

## optimizer_trace_features
|name|value|
|----|-----|
|Name|`optimizer_trace_features`|
|Command line|`--optimizer-trace-features=value`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_trace_features](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_trace_features)|
|dev.mysql.com|[sysvar_optimizer_trace_features](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_trace_features)|

## optimizer_trace_limit
|name|value|
|----|-----|
|Name|`optimizer_trace_limit`|
|Command line|`--optimizer-trace-limit=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `0` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_trace_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_trace_limit)|
|dev.mysql.com|[sysvar_optimizer_trace_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_trace_limit)|

## optimizer_trace_max_mem_size
|name|value|
|----|-----|
|Name|`optimizer_trace_max_mem_size`|
|Command line|`--optimizer-trace-max-mem-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_trace_max_mem_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_trace_max_mem_size)|
|dev.mysql.com|[sysvar_optimizer_trace_max_mem_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_trace_max_mem_size)|

## optimizer_trace_offset
|name|value|
|----|-----|
|Name|`optimizer_trace_offset`|
|Command line|`--optimizer-trace-offset=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`-1`|
|Dynamic|`true`|
|Range|from: `-2147483647` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_optimizer_trace_offset](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_optimizer_trace_offset)|
|dev.mysql.com|[sysvar_optimizer_trace_offset](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_optimizer_trace_offset)|

## parser_max_mem_size
|name|value|
|----|-----|
|Name|`parser_max_mem_size`|
|Command line|`--parser-max-mem-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Dynamic|`true`|
|Range|from: `10000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_parser_max_mem_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_parser_max_mem_size)|
|dev.mysql.com|[sysvar_parser_max_mem_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_parser_max_mem_size)|

## partial_revokes
|name|value|
|----|-----|
|Name|`partial_revokes`|
|Command line|`--partial-revokes[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF (if partial revokes do not exist)ON (if partial revokes exist)`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_partial_revokes](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_partial_revokes)|

## password_history
|name|value|
|----|-----|
|Name|`password_history`|
|Command line|`--password-history=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_password_history](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_password_history)|

## password_require_current
|name|value|
|----|-----|
|Name|`password_require_current`|
|Command line|`--password-require-current[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_password_require_current](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_password_require_current)|

## password_reuse_interval
|name|value|
|----|-----|
|Name|`password_reuse_interval`|
|Command line|`--password-reuse-interval=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_password_reuse_interval](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_password_reuse_interval)|

## persisted_globals_load
|name|value|
|----|-----|
|Name|`persisted_globals_load`|
|Command line|`--persisted-globals-load[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_persisted_globals_load](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_persisted_globals_load)|

## persist_only_admin_x509_subject
|name|value|
|----|-----|
|Name|`persist_only_admin_x509_subject`|
|Command line|`--persist-only-admin-x509-subject=string`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_persist_only_admin_x509_subject](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_persist_only_admin_x509_subject)|

## persist_sensitive_variables_in_plaintext
|name|value|
|----|-----|
|Name|`persist_sensitive_variables_in_plaintext`|
|Command line|`--persist_sensitive_variables_in_plaintext[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_persist_sensitive_variables_in_plaintext](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_persist_sensitive_variables_in_plaintext)|

## pid_file
|name|value|
|----|-----|
|Name|`pid_file`|
|Command line|`--pid-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_pid_file](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_pid_file)|
|dev.mysql.com|[sysvar_pid_file](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_pid_file)|

## plugin_dir
|name|value|
|----|-----|
|Name|`plugin_dir`|
|Command line|`--plugin-dir=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`BASEDIR/lib/plugin`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_plugin_dir](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_plugin_dir)|
|dev.mysql.com|[sysvar_plugin_dir](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_plugin_dir)|

## preload_buffer_size
|name|value|
|----|-----|
|Name|`preload_buffer_size`|
|Command line|`--preload-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`32768`|
|Dynamic|`true`|
|Range|from: `1024` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_preload_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_preload_buffer_size)|
|dev.mysql.com|[sysvar_preload_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_preload_buffer_size)|

## print_identified_with_as_hex
|name|value|
|----|-----|
|Name|`print_identified_with_as_hex`|
|Command line|`--print-identified-with-as-hex[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_print_identified_with_as_hex](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_print_identified_with_as_hex)|

## protocol_compression_algorithms
|name|value|
|----|-----|
|Name|`protocol_compression_algorithms`|
|Command line|`--protocol-compression-algorithms=value`|
|Type of variable|`set`|
|Scope|`global`|
|Default value|`zlib,zstd,uncompressed`|
|Dynamic|`true`|
|Valid value(s)|`zlib`, `zstd`, `uncompressed`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_protocol_compression_algorithms](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_protocol_compression_algorithms)|

## protocol_version
|name|value|
|----|-----|
|Name|`protocol_version`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`10`|
|Dynamic|`false`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_protocol_version](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_protocol_version)|
|dev.mysql.com|[sysvar_protocol_version](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_protocol_version)|

## proxy_user
|name|value|
|----|-----|
|Name|`proxy_user`|
|Type of variable|`string`|
|Scope|`session`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_proxy_user](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_proxy_user)|
|dev.mysql.com|[sysvar_proxy_user](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_proxy_user)|

## pseudo_replica_mode
|name|value|
|----|-----|
|Name|`pseudo_replica_mode`|
|Type of variable|`boolean`|
|Scope|`session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_pseudo_replica_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_pseudo_replica_mode)|

## pseudo_slave_mode
|name|value|
|----|-----|
|Name|`pseudo_slave_mode`|
|Type of variable|`boolean`|
|Scope|`session`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_pseudo_slave_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_pseudo_slave_mode)|
|dev.mysql.com|[sysvar_pseudo_slave_mode](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_pseudo_slave_mode)|

## pseudo_thread_id
|name|value|
|----|-----|
|Name|`pseudo_thread_id`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`2147483647`|
|Dynamic|`true`|
|Range|from: `0` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_pseudo_thread_id](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_pseudo_thread_id)|
|dev.mysql.com|[sysvar_pseudo_thread_id](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_pseudo_thread_id)|

## query_alloc_block_size
|name|value|
|----|-----|
|Name|`query_alloc_block_size`|
|Command line|`--query-alloc-block-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`8192`|
|Dynamic|`true`|
|Range|from: `1024` to: `4294966272`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_query_alloc_block_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_query_alloc_block_size)|
|dev.mysql.com|[sysvar_query_alloc_block_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_query_alloc_block_size)|

## query_prealloc_size
|name|value|
|----|-----|
|Name|`query_prealloc_size`|
|Command line|`--query-prealloc-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`8192`|
|Dynamic|`true`|
|Range|from: `8192`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_query_prealloc_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_query_prealloc_size)|
|dev.mysql.com|[sysvar_query_prealloc_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_query_prealloc_size)|

## rand_seed1
|name|value|
|----|-----|
|Name|`rand_seed1`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`N/A`|
|Dynamic|`true`|
|Range|from: `0` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rand_seed1](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_rand_seed1)|
|dev.mysql.com|[sysvar_rand_seed1](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_rand_seed1)|

## range_alloc_block_size
|name|value|
|----|-----|
|Name|`range_alloc_block_size`|
|Command line|`--range-alloc-block-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`4096`|
|Dynamic|`true`|
|Range|from: `4096` to: `4294966272`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_range_alloc_block_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_range_alloc_block_size)|
|dev.mysql.com|[sysvar_range_alloc_block_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_range_alloc_block_size)|

## range_optimizer_max_mem_size
|name|value|
|----|-----|
|Name|`range_optimizer_max_mem_size`|
|Command line|`--range-optimizer-max-mem-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`8388608`|
|Dynamic|`true`|
|Range|from: `0` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_range_optimizer_max_mem_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_range_optimizer_max_mem_size)|
|dev.mysql.com|[sysvar_range_optimizer_max_mem_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_range_optimizer_max_mem_size)|

## rbr_exec_mode
|name|value|
|----|-----|
|Name|`rbr_exec_mode`|
|Type of variable|`enumeration`|
|Scope|`session`|
|Default value|`STRICT`|
|Dynamic|`true`|
|Valid value(s)|`STRICT`, `IDEMPOTENT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rbr_exec_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_rbr_exec_mode)|
|dev.mysql.com|[sysvar_rbr_exec_mode](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_rbr_exec_mode)|

## read_buffer_size
|name|value|
|----|-----|
|Name|`read_buffer_size`|
|Command line|`--read-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`131072`|
|Dynamic|`true`|
|Range|from: `8192` to: `2147479552`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_read_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_read_buffer_size)|
|dev.mysql.com|[sysvar_read_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_read_buffer_size)|

## read_only
|name|value|
|----|-----|
|Name|`read_only`|
|Command line|`--read-only[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_read_only](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_read_only)|
|dev.mysql.com|[sysvar_read_only](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_read_only)|

## read_rnd_buffer_size
|name|value|
|----|-----|
|Name|`read_rnd_buffer_size`|
|Command line|`--read-rnd-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`262144`|
|Dynamic|`true`|
|Range|from: `1` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_read_rnd_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_read_rnd_buffer_size)|
|dev.mysql.com|[sysvar_read_rnd_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_read_rnd_buffer_size)|

## regexp_stack_limit
|name|value|
|----|-----|
|Name|`regexp_stack_limit`|
|Command line|`--regexp-stack-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8000000`|
|Dynamic|`true`|
|Range|from: `0` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_regexp_stack_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_regexp_stack_limit)|

## regexp_time_limit
|name|value|
|----|-----|
|Name|`regexp_time_limit`|
|Command line|`--regexp-time-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`32`|
|Dynamic|`true`|
|Range|from: `0` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_regexp_time_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_regexp_time_limit)|

## require_row_format
|name|value|
|----|-----|
|Name|`require_row_format`|
|Type of variable|`boolean`|
|Scope|`session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_require_row_format](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_require_row_format)|

## require_secure_transport
|name|value|
|----|-----|
|Name|`require_secure_transport`|
|Command line|`--require-secure-transport[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_require_secure_transport](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_require_secure_transport)|
|dev.mysql.com|[sysvar_require_secure_transport](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_require_secure_transport)|

## resultset_metadata
|name|value|
|----|-----|
|Name|`resultset_metadata`|
|Type of variable|`enumeration`|
|Scope|`session`|
|Default value|`FULL`|
|Dynamic|`true`|
|Valid value(s)|`FULL`, `NONE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_resultset_metadata](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_resultset_metadata)|

## schema_definition_cache
|name|value|
|----|-----|
|Name|`schema_definition_cache`|
|Command line|`--schema-definition-cache=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`256`|
|Dynamic|`true`|
|Range|from: `256` to: `524288`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_schema_definition_cache](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_schema_definition_cache)|

## secure_file_priv
|name|value|
|----|-----|
|Name|`secure_file_priv`|
|Command line|`--secure-file-priv=dir_name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`platform specific`|
|Dynamic|`false`|
|Valid value(s)|`empty string`, `dirname`, `NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_secure_file_priv](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_secure_file_priv)|
|dev.mysql.com|[sysvar_secure_file_priv](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_secure_file_priv)|

## select_into_buffer_size
|name|value|
|----|-----|
|Name|`select_into_buffer_size`|
|Command line|`--select-into-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`131072`|
|Dynamic|`true`|
|Range|from: `8192` to: `2147479552`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_select_into_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_select_into_buffer_size)|

## select_into_disk_sync
|name|value|
|----|-----|
|Name|`select_into_disk_sync`|
|Command line|`--select-into-disk-sync={ON|OFF}`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_select_into_disk_sync](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_select_into_disk_sync)|

## select_into_disk_sync_delay
|name|value|
|----|-----|
|Name|`select_into_disk_sync_delay`|
|Command line|`--select-into-disk-sync-delay=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_select_into_disk_sync_delay](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_select_into_disk_sync_delay)|

## session_track_gtids
|name|value|
|----|-----|
|Name|`session_track_gtids`|
|Command line|`--session-track-gtids=value`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `OWN_GTID`, `ALL_GTIDS`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_session_track_gtids](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_session_track_gtids)|
|dev.mysql.com|[sysvar_session_track_gtids](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_session_track_gtids)|

## session_track_schema
|name|value|
|----|-----|
|Name|`session_track_schema`|
|Command line|`--session-track-schema[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_session_track_schema](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_session_track_schema)|
|dev.mysql.com|[sysvar_session_track_schema](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_session_track_schema)|

## session_track_state_change
|name|value|
|----|-----|
|Name|`session_track_state_change`|
|Command line|`--session-track-state-change[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_session_track_state_change](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_session_track_state_change)|
|dev.mysql.com|[sysvar_session_track_state_change](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_session_track_state_change)|

## session_track_system_variables
|name|value|
|----|-----|
|Name|`session_track_system_variables`|
|Command line|`--session-track-system-variables=#`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`time_zone, autocommit, character_set_client, character_set_results, character_set_connection`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_session_track_system_variables](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_session_track_system_variables)|
|dev.mysql.com|[sysvar_session_track_system_variables](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_session_track_system_variables)|

## session_track_transaction_info
|name|value|
|----|-----|
|Name|`session_track_transaction_info`|
|Command line|`--session-track-transaction-info=value`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `STATE`, `CHARACTERISTICS`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_session_track_transaction_info](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_session_track_transaction_info)|
|dev.mysql.com|[sysvar_session_track_transaction_info](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_session_track_transaction_info)|

## sha256_password_auto_generate_rsa_keys
|name|value|
|----|-----|
|Name|`sha256_password_auto_generate_rsa_keys`|
|Command line|`--sha256-password-auto-generate-rsa-keys[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sha256_password_auto_generate_rsa_keys](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sha256_password_auto_generate_rsa_keys)|
|dev.mysql.com|[sysvar_sha256_password_auto_generate_rsa_keys](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sha256_password_auto_generate_rsa_keys)|

## sha256_password_private_key_path
|name|value|
|----|-----|
|Name|`sha256_password_private_key_path`|
|Command line|`--sha256-password-private-key-path=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`private_key.pem`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sha256_password_private_key_path](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sha256_password_private_key_path)|
|dev.mysql.com|[sysvar_sha256_password_private_key_path](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sha256_password_private_key_path)|

## sha256_password_proxy_users
|name|value|
|----|-----|
|Name|`sha256_password_proxy_users`|
|Command line|`--sha256-password-proxy-users[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sha256_password_proxy_users](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sha256_password_proxy_users)|
|dev.mysql.com|[sysvar_sha256_password_proxy_users](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sha256_password_proxy_users)|

## sha256_password_public_key_path
|name|value|
|----|-----|
|Name|`sha256_password_public_key_path`|
|Command line|`--sha256-password-public-key-path=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`public_key.pem`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sha256_password_public_key_path](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sha256_password_public_key_path)|
|dev.mysql.com|[sysvar_sha256_password_public_key_path](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sha256_password_public_key_path)|

## shared_memory
|name|value|
|----|-----|
|Name|`shared_memory`|
|Command line|`--shared-memory[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_shared_memory](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_shared_memory)|
|dev.mysql.com|[sysvar_shared_memory](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_shared_memory)|

## shared_memory_base_name
|name|value|
|----|-----|
|Name|`shared_memory_base_name`|
|Command line|`--shared-memory-base-name=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`MYSQL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_shared_memory_base_name](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_shared_memory_base_name)|
|dev.mysql.com|[sysvar_shared_memory_base_name](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_shared_memory_base_name)|

## show_create_table_verbosity
|name|value|
|----|-----|
|Name|`show_create_table_verbosity`|
|Command line|`--show-create-table-verbosity[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_show_create_table_verbosity](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_show_create_table_verbosity)|
|dev.mysql.com|[sysvar_show_create_table_verbosity](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_show_create_table_verbosity)|

## show_gipk_in_create_table_and_information_schema
|name|value|
|----|-----|
|Name|`show_gipk_in_create_table_and_information_schema`|
|Command line|`--show-gipk-in-create-table-and-information-schema[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_show_gipk_in_create_table_and_information_schema](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_show_gipk_in_create_table_and_information_schema)|

## show_old_temporals
|name|value|
|----|-----|
|Name|`show_old_temporals`|
|Command line|`--show-old-temporals[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_show_old_temporals](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_show_old_temporals)|
|dev.mysql.com|[sysvar_show_old_temporals](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_show_old_temporals)|

## skip_external_locking
|name|value|
|----|-----|
|Name|`skip_external_locking`|
|Command line|`--skip-external-locking[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_skip_external_locking](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_skip_external_locking)|
|dev.mysql.com|[sysvar_skip_external_locking](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_skip_external_locking)|

## skip_name_resolve
|name|value|
|----|-----|
|Name|`skip_name_resolve`|
|Command line|`--skip-name-resolve[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_skip_name_resolve](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_skip_name_resolve)|
|dev.mysql.com|[sysvar_skip_name_resolve](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_skip_name_resolve)|

## skip_networking
|name|value|
|----|-----|
|Name|`skip_networking`|
|Command line|`--skip-networking[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_skip_networking](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_skip_networking)|
|dev.mysql.com|[sysvar_skip_networking](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_skip_networking)|

## slow_launch_time
|name|value|
|----|-----|
|Name|`slow_launch_time`|
|Command line|`--slow-launch-time=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`true`|
|Range|from: `0` to: `31536000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slow_launch_time](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_slow_launch_time)|
|dev.mysql.com|[sysvar_slow_launch_time](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_slow_launch_time)|

## slow_query_log
|name|value|
|----|-----|
|Name|`slow_query_log`|
|Command line|`--slow-query-log[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slow_query_log](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_slow_query_log)|
|dev.mysql.com|[sysvar_slow_query_log](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_slow_query_log)|

## slow_query_log_file
|name|value|
|----|-----|
|Name|`slow_query_log_file`|
|Command line|`--slow-query-log-file=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`host_name-slow.log`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_slow_query_log_file](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_slow_query_log_file)|
|dev.mysql.com|[sysvar_slow_query_log_file](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_slow_query_log_file)|

## sort_buffer_size
|name|value|
|----|-----|
|Name|`sort_buffer_size`|
|Command line|`--sort-buffer-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`262144`|
|Dynamic|`true`|
|Range|from: `32768`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sort_buffer_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sort_buffer_size)|
|dev.mysql.com|[sysvar_sort_buffer_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sort_buffer_size)|

## sql_auto_is_null
|name|value|
|----|-----|
|Name|`sql_auto_is_null`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_auto_is_null](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_auto_is_null)|
|dev.mysql.com|[sysvar_sql_auto_is_null](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_auto_is_null)|

## sql_big_selects
|name|value|
|----|-----|
|Name|`sql_big_selects`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_big_selects](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_big_selects)|
|dev.mysql.com|[sysvar_sql_big_selects](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_big_selects)|

## sql_buffer_result
|name|value|
|----|-----|
|Name|`sql_buffer_result`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_buffer_result](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_buffer_result)|
|dev.mysql.com|[sysvar_sql_buffer_result](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_buffer_result)|

## sql_generate_invisible_primary_key
|name|value|
|----|-----|
|Name|`sql_generate_invisible_primary_key`|
|Command line|`--sql-generate-invisible-primary-key[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_generate_invisible_primary_key](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_generate_invisible_primary_key)|

## sql_log_off
|name|value|
|----|-----|
|Name|`sql_log_off`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|
|Valid value(s)|`OFF`, `ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_log_off](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_log_off)|
|dev.mysql.com|[sysvar_sql_log_off](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_log_off)|

## sql_notes
|name|value|
|----|-----|
|Name|`sql_notes`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_notes](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_notes)|
|dev.mysql.com|[sysvar_sql_notes](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_notes)|

## sql_quote_show_create
|name|value|
|----|-----|
|Name|`sql_quote_show_create`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_quote_show_create](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_quote_show_create)|
|dev.mysql.com|[sysvar_sql_quote_show_create](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_quote_show_create)|

## sql_require_primary_key
|name|value|
|----|-----|
|Name|`sql_require_primary_key`|
|Command line|`--sql-require-primary-key[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_require_primary_key](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_require_primary_key)|

## sql_safe_updates
|name|value|
|----|-----|
|Name|`sql_safe_updates`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_safe_updates](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_safe_updates)|
|dev.mysql.com|[sysvar_sql_safe_updates](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_safe_updates)|

## sql_select_limit
|name|value|
|----|-----|
|Name|`sql_select_limit`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`18446744073709551615`|
|Dynamic|`true`|
|Range|from: `0` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_select_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_select_limit)|
|dev.mysql.com|[sysvar_sql_select_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_select_limit)|

## sql_warnings
|name|value|
|----|-----|
|Name|`sql_warnings`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sql_warnings](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_sql_warnings)|
|dev.mysql.com|[sysvar_sql_warnings](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sql_warnings)|

## ssl_ca
|name|value|
|----|-----|
|Name|`ssl_ca`|
|Command line|`--ssl-ca=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_ca](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_ca)|
|dev.mysql.com|[sysvar_ssl_ca](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ssl_ca)|

## ssl_capath
|name|value|
|----|-----|
|Name|`ssl_capath`|
|Command line|`--ssl-capath=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_capath](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_capath)|
|dev.mysql.com|[sysvar_ssl_capath](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ssl_capath)|

## ssl_cert
|name|value|
|----|-----|
|Name|`ssl_cert`|
|Command line|`--ssl-cert=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_cert](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_cert)|
|dev.mysql.com|[sysvar_ssl_cert](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ssl_cert)|

## ssl_cipher
|name|value|
|----|-----|
|Name|`ssl_cipher`|
|Command line|`--ssl-cipher=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_cipher](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_cipher)|
|dev.mysql.com|[sysvar_ssl_cipher](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ssl_cipher)|

## ssl_crl
|name|value|
|----|-----|
|Name|`ssl_crl`|
|Command line|`--ssl-crl=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_crl](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_crl)|
|dev.mysql.com|[sysvar_ssl_crl](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ssl_crl)|

## ssl_crlpath
|name|value|
|----|-----|
|Name|`ssl_crlpath`|
|Command line|`--ssl-crlpath=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_crlpath](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_crlpath)|
|dev.mysql.com|[sysvar_ssl_crlpath](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ssl_crlpath)|

## ssl_fips_mode
|name|value|
|----|-----|
|Name|`ssl_fips_mode`|
|Command line|`--ssl-fips-mode={OFF|ON|STRICT}`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|
|Valid value(s)|`OFF`, `ON`, `STRICT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_fips_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_fips_mode)|

## ssl_key
|name|value|
|----|-----|
|Name|`ssl_key`|
|Command line|`--ssl-key=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_key](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_key)|
|dev.mysql.com|[sysvar_ssl_key](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ssl_key)|

## ssl_session_cache_mode
|name|value|
|----|-----|
|Name|`ssl_session_cache_mode`|
|Command line|`--ssl_session_cache_mode={ON|OFF}`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|
|Valid value(s)|`ON`, `OFF`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_session_cache_mode](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_session_cache_mode)|

## ssl_session_cache_timeout
|name|value|
|----|-----|
|Name|`ssl_session_cache_timeout`|
|Command line|`--ssl_session_cache_timeout`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`300`|
|Dynamic|`true`|
|Range|from: `0` to: `84600`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ssl_session_cache_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_ssl_session_cache_timeout)|

## statement_id
|name|value|
|----|-----|
|Name|`statement_id`|
|Type of variable|`integer`|
|Scope|`session`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_statement_id](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_statement_id)|

## stored_program_cache
|name|value|
|----|-----|
|Name|`stored_program_cache`|
|Command line|`--stored-program-cache=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`256`|
|Dynamic|`true`|
|Range|from: `16` to: `524288`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_stored_program_cache](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_stored_program_cache)|
|dev.mysql.com|[sysvar_stored_program_cache](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_stored_program_cache)|

## stored_program_definition_cache
|name|value|
|----|-----|
|Name|`stored_program_definition_cache`|
|Command line|`--stored-program-definition-cache=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`256`|
|Dynamic|`true`|
|Range|from: `256` to: `524288`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_stored_program_definition_cache](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_stored_program_definition_cache)|

## super_read_only
|name|value|
|----|-----|
|Name|`super_read_only`|
|Command line|`--super-read-only[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_super_read_only](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_super_read_only)|
|dev.mysql.com|[sysvar_super_read_only](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_super_read_only)|

## syseventlog.facility
|name|value|
|----|-----|
|Name|`syseventlog.facility`|
|Command line|`--syseventlog.facility=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`daemon`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_syseventlog.facility](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_syseventlog.facility)|

## syseventlog.include_pid
|name|value|
|----|-----|
|Name|`syseventlog.include_pid`|
|Command line|`--syseventlog.include-pid[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_syseventlog.include_pid](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_syseventlog.include_pid)|

## syseventlog.tag
|name|value|
|----|-----|
|Name|`syseventlog.tag`|
|Command line|`--syseventlog.tag=tag`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_syseventlog.tag](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_syseventlog.tag)|

## system_time_zone
|name|value|
|----|-----|
|Name|`system_time_zone`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_system_time_zone](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_system_time_zone)|
|dev.mysql.com|[sysvar_system_time_zone](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_system_time_zone)|

## table_definition_cache
|name|value|
|----|-----|
|Name|`table_definition_cache`|
|Command line|`--table-definition-cache=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`true`|
|Range|from: `400` to: `524288`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_table_definition_cache](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_table_definition_cache)|
|dev.mysql.com|[sysvar_table_definition_cache](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_table_definition_cache)|

## table_encryption_privilege_check
|name|value|
|----|-----|
|Name|`table_encryption_privilege_check`|
|Command line|`--table-encryption-privilege-check[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_table_encryption_privilege_check](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_table_encryption_privilege_check)|

## table_open_cache
|name|value|
|----|-----|
|Name|`table_open_cache`|
|Command line|`--table-open-cache=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`true`|
|Range|from: `1` to: `524288`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_table_open_cache](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_table_open_cache)|
|dev.mysql.com|[sysvar_table_open_cache](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_table_open_cache)|

## table_open_cache_instances
|name|value|
|----|-----|
|Name|`table_open_cache_instances`|
|Command line|`--table-open-cache-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`16`|
|Dynamic|`false`|
|Range|from: `1` to: `64`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_table_open_cache_instances](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_table_open_cache_instances)|
|dev.mysql.com|[sysvar_table_open_cache_instances](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_table_open_cache_instances)|

## tablespace_definition_cache
|name|value|
|----|-----|
|Name|`tablespace_definition_cache`|
|Command line|`--tablespace-definition-cache=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`256`|
|Dynamic|`true`|
|Range|from: `256` to: `524288`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_tablespace_definition_cache](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_tablespace_definition_cache)|

## temptable_max_mmap
|name|value|
|----|-----|
|Name|`temptable_max_mmap`|
|Command line|`--temptable-max-mmap=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1073741824`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_temptable_max_mmap](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_temptable_max_mmap)|

## temptable_max_ram
|name|value|
|----|-----|
|Name|`temptable_max_ram`|
|Command line|`--temptable-max-ram=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1073741824`|
|Dynamic|`true`|
|Range|from: `2097152`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_temptable_max_ram](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_temptable_max_ram)|

## temptable_use_mmap
|name|value|
|----|-----|
|Name|`temptable_use_mmap`|
|Command line|`--temptable-use-mmap[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_temptable_use_mmap](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_temptable_use_mmap)|

## thread_cache_size
|name|value|
|----|-----|
|Name|`thread_cache_size`|
|Command line|`--thread-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`-1 (signifies autosizing; do not assign this literal value)`|
|Dynamic|`true`|
|Range|from: `0` to: `16384`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_cache_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_cache_size)|
|dev.mysql.com|[sysvar_thread_cache_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_cache_size)|

## thread_handling
|name|value|
|----|-----|
|Name|`thread_handling`|
|Command line|`--thread-handling=name`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`one-thread-per-connection`|
|Dynamic|`false`|
|Valid value(s)|`no-threads`, `one-thread-per-connection`, `loaded-dynamically`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_handling](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_handling)|
|dev.mysql.com|[sysvar_thread_handling](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_handling)|

## thread_pool_algorithm
|name|value|
|----|-----|
|Name|`thread_pool_algorithm`|
|Command line|`--thread-pool-algorithm=#`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_algorithm](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_algorithm)|
|dev.mysql.com|[sysvar_thread_pool_algorithm](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_pool_algorithm)|

## thread_pool_dedicated_listeners
|name|value|
|----|-----|
|Name|`thread_pool_dedicated_listeners`|
|Command line|`--thread-pool-dedicated-listeners`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_dedicated_listeners](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_dedicated_listeners)|

## thread_pool_high_priority_connection
|name|value|
|----|-----|
|Name|`thread_pool_high_priority_connection`|
|Command line|`--thread-pool-high-priority-connection=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_high_priority_connection](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_high_priority_connection)|
|dev.mysql.com|[sysvar_thread_pool_high_priority_connection](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_pool_high_priority_connection)|

## thread_pool_max_active_query_threads
|name|value|
|----|-----|
|Name|`thread_pool_max_active_query_threads`|
|Command line|`--thread-pool-max-active-query-threads`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `512`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_max_active_query_threads](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_max_active_query_threads)|

## thread_pool_max_transactions_limit
|name|value|
|----|-----|
|Name|`thread_pool_max_transactions_limit`|
|Command line|`--thread-pool-max-transactions-limit`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `1000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_max_transactions_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_max_transactions_limit)|

## thread_pool_max_unused_threads
|name|value|
|----|-----|
|Name|`thread_pool_max_unused_threads`|
|Command line|`--thread-pool-max-unused-threads=#`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_max_unused_threads](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_max_unused_threads)|
|dev.mysql.com|[sysvar_thread_pool_max_unused_threads](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_pool_max_unused_threads)|

## thread_pool_prio_kickup_timer
|name|value|
|----|-----|
|Name|`thread_pool_prio_kickup_timer`|
|Command line|`--thread-pool-prio-kickup-timer=#`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_prio_kickup_timer](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_prio_kickup_timer)|
|dev.mysql.com|[sysvar_thread_pool_prio_kickup_timer](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_pool_prio_kickup_timer)|

## thread_pool_query_threads_per_group
|name|value|
|----|-----|
|Name|`thread_pool_query_threads_per_group`|
|Command line|`--thread-pool-query-threads-per-group`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1`|
|Dynamic|`true`|
|Range|from: `1` to: `4096`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_query_threads_per_group](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_query_threads_per_group)|

## thread_pool_size
|name|value|
|----|-----|
|Name|`thread_pool_size`|
|Command line|`--thread-pool-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`16`|
|Dynamic|`false`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_size)|
|dev.mysql.com|[sysvar_thread_pool_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_pool_size)|

## thread_pool_stall_limit
|name|value|
|----|-----|
|Name|`thread_pool_stall_limit`|
|Command line|`--thread-pool-stall-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`6`|
|Dynamic|`true`|
|Range|from: `4` to: `600`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_stall_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_stall_limit)|
|dev.mysql.com|[sysvar_thread_pool_stall_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_pool_stall_limit)|

## thread_pool_transaction_delay
|name|value|
|----|-----|
|Name|`thread_pool_transaction_delay`|
|Command line|`--thread-pool-transaction-delay`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `300000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_pool_transaction_delay](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_pool_transaction_delay)|

## thread_stack
|name|value|
|----|-----|
|Name|`thread_stack`|
|Command line|`--thread-stack=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Dynamic|`false`|
|Range|from: `131072`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_thread_stack](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_thread_stack)|
|dev.mysql.com|[sysvar_thread_stack](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_thread_stack)|

## time_zone
|name|value|
|----|-----|
|Name|`time_zone`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`SYSTEM`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_time_zone](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_time_zone)|
|dev.mysql.com|[sysvar_time_zone](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_time_zone)|

## timestamp
|name|value|
|----|-----|
|Name|`timestamp`|
|Type of variable|`numeric`|
|Scope|`session`|
|Default value|`UNIX_TIMESTAMP()`|
|Dynamic|`true`|
|Range|from: `1` to: `2147483647`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_timestamp](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_timestamp)|
|dev.mysql.com|[sysvar_timestamp](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_timestamp)|

## tls_ciphersuites
|name|value|
|----|-----|
|Name|`tls_ciphersuites`|
|Command line|`--tls-ciphersuites=ciphersuite_list`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_tls_ciphersuites](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_tls_ciphersuites)|

## tls_version
|name|value|
|----|-----|
|Name|`tls_version`|
|Command line|`--tls-version=protocol_list`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`TLSv1,TLSv1.1,TLSv1.2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_tls_version](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_tls_version)|
|dev.mysql.com|[sysvar_tls_version](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_tls_version)|

## tmp_table_size
|name|value|
|----|-----|
|Name|`tmp_table_size`|
|Command line|`--tmp-table-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`16777216`|
|Dynamic|`true`|
|Range|from: `1024` to: `1.844674407371E+19`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_tmp_table_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_tmp_table_size)|
|dev.mysql.com|[sysvar_tmp_table_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_tmp_table_size)|

## transaction_alloc_block_size
|name|value|
|----|-----|
|Name|`transaction_alloc_block_size`|
|Command line|`--transaction-alloc-block-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`8192`|
|Dynamic|`true`|
|Range|from: `1024` to: `131072`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_transaction_alloc_block_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_transaction_alloc_block_size)|
|dev.mysql.com|[sysvar_transaction_alloc_block_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_transaction_alloc_block_size)|

## transaction_prealloc_size
|name|value|
|----|-----|
|Name|`transaction_prealloc_size`|
|Command line|`--transaction-prealloc-size=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`4096`|
|Dynamic|`true`|
|Range|from: `1024` to: `131072`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_transaction_prealloc_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_transaction_prealloc_size)|
|dev.mysql.com|[sysvar_transaction_prealloc_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_transaction_prealloc_size)|

## unique_checks
|name|value|
|----|-----|
|Name|`unique_checks`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_unique_checks](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_unique_checks)|
|dev.mysql.com|[sysvar_unique_checks](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_unique_checks)|

## updatable_views_with_limit
|name|value|
|----|-----|
|Name|`updatable_views_with_limit`|
|Command line|`--updatable-views-with-limit[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`1`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_updatable_views_with_limit](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_updatable_views_with_limit)|
|dev.mysql.com|[sysvar_updatable_views_with_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_updatable_views_with_limit)|

## version_comment
|name|value|
|----|-----|
|Name|`version_comment`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_version_comment](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_version_comment)|
|dev.mysql.com|[sysvar_version_comment](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_version_comment)|

## version_compile_machine
|name|value|
|----|-----|
|Name|`version_compile_machine`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_version_compile_machine](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_version_compile_machine)|
|dev.mysql.com|[sysvar_version_compile_machine](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_version_compile_machine)|

## version_compile_os
|name|value|
|----|-----|
|Name|`version_compile_os`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_version_compile_os](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_version_compile_os)|
|dev.mysql.com|[sysvar_version_compile_os](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_version_compile_os)|

## version_compile_zlib
|name|value|
|----|-----|
|Name|`version_compile_zlib`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_version_compile_zlib](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_version_compile_zlib)|

## wait_timeout
|name|value|
|----|-----|
|Name|`wait_timeout`|
|Command line|`--wait-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`28800`|
|Dynamic|`true`|
|Range|from: `1`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_wait_timeout](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_wait_timeout)|
|dev.mysql.com|[sysvar_wait_timeout](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_wait_timeout)|

## windowing_use_high_precision
|name|value|
|----|-----|
|Name|`windowing_use_high_precision`|
|Command line|`--windowing-use-high-precision[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_windowing_use_high_precision](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_windowing_use_high_precision)|

## xa_detach_on_prepare
|name|value|
|----|-----|
|Name|`xa_detach_on_prepare`|
|Command line|`--xa-detach-on-prepare[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_xa_detach_on_prepare](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_xa_detach_on_prepare)|

## error_count
|name|value|
|----|-----|
|Name|`error_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_error_count](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_error_count)|
|dev.mysql.com|[sysvar_error_count](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_error_count)|

## have_compress
|name|value|
|----|-----|
|Name|`have_compress`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_compress](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_compress)|
|dev.mysql.com|[sysvar_have_compress](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_compress)|

## have_dynamic_loading
|name|value|
|----|-----|
|Name|`have_dynamic_loading`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_dynamic_loading](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_dynamic_loading)|
|dev.mysql.com|[sysvar_have_dynamic_loading](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_dynamic_loading)|

## have_geometry
|name|value|
|----|-----|
|Name|`have_geometry`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_geometry](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_geometry)|
|dev.mysql.com|[sysvar_have_geometry](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_geometry)|

## have_openssl
|name|value|
|----|-----|
|Name|`have_openssl`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_openssl](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_openssl)|
|dev.mysql.com|[sysvar_have_openssl](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_openssl)|

## have_profiling
|name|value|
|----|-----|
|Name|`have_profiling`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_profiling](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_profiling)|
|dev.mysql.com|[sysvar_have_profiling](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_profiling)|

## have_query_cache
|name|value|
|----|-----|
|Name|`have_query_cache`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_query_cache](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_query_cache)|
|dev.mysql.com|[sysvar_have_query_cache](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_query_cache)|

## have_rtree_keys
|name|value|
|----|-----|
|Name|`have_rtree_keys`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_rtree_keys](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_rtree_keys)|
|dev.mysql.com|[sysvar_have_rtree_keys](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_rtree_keys)|

## have_symlink
|name|value|
|----|-----|
|Name|`have_symlink`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_symlink](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_have_symlink)|
|dev.mysql.com|[sysvar_have_symlink](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_symlink)|

## identity
|name|value|
|----|-----|
|Name|`identity`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_identity](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_identity)|
|dev.mysql.com|[sysvar_identity](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_identity)|

## insert_id
|name|value|
|----|-----|
|Name|`insert_id`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_insert_id](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_insert_id)|
|dev.mysql.com|[sysvar_insert_id](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_insert_id)|

## internal_tmp_disk_storage_engine
|name|value|
|----|-----|
|Name|`internal_tmp_disk_storage_engine`|
|Command line|`--internal-tmp-disk-storage-engine=#`|
|Type of variable|`enumeration`|
|Scope|`global`|
|Default value|`INNODB`|
|Dynamic|`true`|
|Valid value(s)|`MYISAM`, `INNODB`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_internal_tmp_disk_storage_engine](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_internal_tmp_disk_storage_engine)|
|dev.mysql.com|[sysvar_internal_tmp_disk_storage_engine](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_internal_tmp_disk_storage_engine)|

## last_insert_id
|name|value|
|----|-----|
|Name|`last_insert_id`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_last_insert_id](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_last_insert_id)|
|dev.mysql.com|[sysvar_last_insert_id](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_last_insert_id)|

## log_syslog
|name|value|
|----|-----|
|Name|`log_syslog`|
|Command line|`--log-syslog[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_syslog](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_syslog)|
|dev.mysql.com|[sysvar_log_syslog](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_syslog)|

## log_syslog_facility
|name|value|
|----|-----|
|Name|`log_syslog_facility`|
|Command line|`--log-syslog-facility=value`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`daemon`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_syslog_facility](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_syslog_facility)|
|dev.mysql.com|[sysvar_log_syslog_facility](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_syslog_facility)|

## log_syslog_include_pid
|name|value|
|----|-----|
|Name|`log_syslog_include_pid`|
|Command line|`--log-syslog-include-pid[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_syslog_include_pid](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_syslog_include_pid)|
|dev.mysql.com|[sysvar_log_syslog_include_pid](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_syslog_include_pid)|

## log_syslog_tag
|name|value|
|----|-----|
|Name|`log_syslog_tag`|
|Command line|`--log-syslog-tag=tag`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`empty string`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_syslog_tag](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_log_syslog_tag)|
|dev.mysql.com|[sysvar_log_syslog_tag](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_syslog_tag)|

## metadata_locks_cache_size
|name|value|
|----|-----|
|Name|`metadata_locks_cache_size`|
|Command line|`--metadata-locks-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1024`|
|Dynamic|`false`|
|Range|from: `1` to: `1048576`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_metadata_locks_cache_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_metadata_locks_cache_size)|
|dev.mysql.com|[sysvar_metadata_locks_cache_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_metadata_locks_cache_size)|

## metadata_locks_hash_instances
|name|value|
|----|-----|
|Name|`metadata_locks_hash_instances`|
|Command line|`--metadata-locks-hash-instances=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8`|
|Dynamic|`false`|
|Range|from: `1` to: `1024`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_metadata_locks_hash_instances](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_metadata_locks_hash_instances)|
|dev.mysql.com|[sysvar_metadata_locks_hash_instances](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_metadata_locks_hash_instances)|

## myisam_repair_threads
|name|value|
|----|-----|
|Name|`myisam_repair_threads`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_myisam_repair_threads](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_myisam_repair_threads)|
|dev.mysql.com|[sysvar_myisam_repair_threads](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_myisam_repair_threads)|

## profiling
|name|value|
|----|-----|
|Name|`profiling`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_profiling](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_profiling)|
|dev.mysql.com|[sysvar_profiling](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_profiling)|

## profiling_history_size
|name|value|
|----|-----|
|Name|`profiling_history_size`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_profiling_history_size](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_profiling_history_size)|
|dev.mysql.com|[sysvar_profiling_history_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_profiling_history_size)|

## rand_seed2
|name|value|
|----|-----|
|Name|`rand_seed2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_rand_seed2](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_rand_seed2)|
|dev.mysql.com|[sysvar_rand_seed2](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_rand_seed2)|

## version
|name|value|
|----|-----|
|Name|`version`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_version](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_version)|
|dev.mysql.com|[sysvar_version](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_version)|

## warning_count
|name|value|
|----|-----|
|Name|`warning_count`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_warning_count](https://dev.mysql.com/doc/refman/8.0/en/server-system-variables.html#sysvar_warning_count)|
|dev.mysql.com|[sysvar_warning_count](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_warning_count)|

## ignore_db_dirs
|name|value|
|----|-----|
|Name|`ignore_db_dirs`|
|Type of variable|`string`|
|Scope|`global`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_ignore_db_dirs](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_ignore_db_dirs)|

## log_warnings
|name|value|
|----|-----|
|Name|`log_warnings`|
|Command line|`--log-warnings[=#]`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_log_warnings](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_log_warnings)|

## multi_range_count
|name|value|
|----|-----|
|Name|`multi_range_count`|
|Command line|`--multi-range-count=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`256`|
|Dynamic|`true`|
|Range|from: `1` to: `4294967295`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_multi_range_count](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_multi_range_count)|

## old_passwords
|name|value|
|----|-----|
|Name|`old_passwords`|
|Command line|`--old-passwords=value`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Valid value(s)|`0`, `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_old_passwords](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_old_passwords)|

## query_cache_limit
|name|value|
|----|-----|
|Name|`query_cache_limit`|
|Command line|`--query-cache-limit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1048576`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_query_cache_limit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_query_cache_limit)|

## query_cache_min_res_unit
|name|value|
|----|-----|
|Name|`query_cache_min_res_unit`|
|Command line|`--query-cache-min-res-unit=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`4096`|
|Dynamic|`true`|
|Range|from: `512`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_query_cache_min_res_unit](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_query_cache_min_res_unit)|

## query_cache_size
|name|value|
|----|-----|
|Name|`query_cache_size`|
|Command line|`--query-cache-size=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`1048576`|
|Dynamic|`true`|
|Range|from: `0`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_query_cache_size](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_query_cache_size)|

## query_cache_type
|name|value|
|----|-----|
|Name|`query_cache_type`|
|Command line|`--query-cache-type=#`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`true`|
|Valid value(s)|`0`, `1`, `2`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_query_cache_type](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_query_cache_type)|

## query_cache_wlock_invalidate
|name|value|
|----|-----|
|Name|`query_cache_wlock_invalidate`|
|Command line|`--query-cache-wlock-invalidate[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_query_cache_wlock_invalidate](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_query_cache_wlock_invalidate)|

## secure_auth
|name|value|
|----|-----|
|Name|`secure_auth`|
|Command line|`--secure-auth[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|
|Valid value(s)|`ON`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_secure_auth](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_secure_auth)|

## show_compatibility_56
|name|value|
|----|-----|
|Name|`show_compatibility_56`|
|Command line|`--show-compatibility-56[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_show_compatibility_56](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_show_compatibility_56)|

## sync_frm
|name|value|
|----|-----|
|Name|`sync_frm`|
|Command line|`--sync-frm[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_sync_frm](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_sync_frm)|

## tx_isolation
|name|value|
|----|-----|
|Name|`tx_isolation`|
|Type of variable|`enumeration`|
|Scope|`global`, `session`|
|Default value|`REPEATABLE-READ`|
|Dynamic|`true`|
|Valid value(s)|`READ-UNCOMMITTED`, `READ-COMMITTED`, `REPEATABLE-READ`, `SERIALIZABLE`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_tx_isolation](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_tx_isolation)|

## tx_read_only
|name|value|
|----|-----|
|Name|`tx_read_only`|
|Type of variable|`boolean`|
|Scope|`global`, `session`|
|Default value|`OFF`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_tx_read_only](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_tx_read_only)|

## date_format
|name|value|
|----|-----|
|Name|`date_format`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_date_format](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_date_format)|

## datetime_format
|name|value|
|----|-----|
|Name|`datetime_format`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_datetime_format](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_datetime_format)|

## have_crypt
|name|value|
|----|-----|
|Name|`have_crypt`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_have_crypt](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_have_crypt)|

## max_tmp_tables
|name|value|
|----|-----|
|Name|`max_tmp_tables`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_max_tmp_tables](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_max_tmp_tables)|

## time_format
|name|value|
|----|-----|
|Name|`time_format`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_time_format](https://dev.mysql.com/doc/refman/5.7/en/server-system-variables.html#sysvar_time_format)|

## version_tokens_session
|name|value|
|----|-----|
|Name|`version_tokens_session`|
|Command line|`--version-tokens-session=value`|
|Type of variable|`string`|
|Scope|`global`, `session`|
|Default value|`NULL`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_version_tokens_session](https://dev.mysql.com/doc/refman/8.0/en/version-tokens-reference.html#sysvar_version_tokens_session)|

## version_tokens_session_number
|name|value|
|----|-----|
|Name|`version_tokens_session_number`|
|Command line|`--version-tokens-session-number=#`|
|Type of variable|`integer`|
|Scope|`global`, `session`|
|Default value|`0`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_version_tokens_session_number](https://dev.mysql.com/doc/refman/8.0/en/version-tokens-reference.html#sysvar_version_tokens_session_number)|

## mysqlx
|name|value|
|----|-----|
|Name|`mysqlx`|
|Command line|`--mysqlx[=value]`|
|Type of variable|`enumeration`|
|Default value|`ON`|
|Valid value(s)|`ON`, `OFF`, `FORCE`, `FORCE_PLUS_PERMANENT`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[option_mysqld_mysqlx](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#option_mysqld_mysqlx)|

## mysqlx_bind_address
|name|value|
|----|-----|
|Name|`mysqlx_bind_address`|
|Command line|`--mysqlx-bind-address=addr`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`*`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_bind_address](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_bind_address)|

## mysqlx_compression_algorithms
|name|value|
|----|-----|
|Name|`mysqlx_compression_algorithms`|
|Command line|`--mysqlx-compression-algorithms=value`|
|Type of variable|`set`|
|Scope|`global`|
|Default value|`deflate_stream,lz4_message,zstd_stream`|
|Dynamic|`true`|
|Valid value(s)|`deflate_stream`, `lz4_message`, `zstd_stream`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_compression_algorithms](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_compression_algorithms)|

## mysqlx_connect_timeout
|name|value|
|----|-----|
|Name|`mysqlx_connect_timeout`|
|Command line|`--mysqlx-connect-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`30`|
|Dynamic|`true`|
|Range|from: `1` to: `1000000000`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_connect_timeout](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_connect_timeout)|

## mysqlx_deflate_default_compression_level
|name|value|
|----|-----|
|Name|`mysqlx_deflate_default_compression_level`|
|Command line|`--mysqlx_deflate_default_compression_level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`3`|
|Dynamic|`true`|
|Range|from: `1` to: `9`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_deflate_default_compression_level](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_deflate_default_compression_level)|

## mysqlx_deflate_max_client_compression_level
|name|value|
|----|-----|
|Name|`mysqlx_deflate_max_client_compression_level`|
|Command line|`--mysqlx_deflate_max_client_compression_level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`5`|
|Dynamic|`true`|
|Range|from: `1` to: `9`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_deflate_max_client_compression_level](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_deflate_max_client_compression_level)|

## mysqlx_document_id_unique_prefix
|name|value|
|----|-----|
|Name|`mysqlx_document_id_unique_prefix`|
|Command line|`--mysqlx-document-id-unique-prefix=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`true`|
|Range|from: `0` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_document_id_unique_prefix](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_document_id_unique_prefix)|

## mysqlx_enable_hello_notice
|name|value|
|----|-----|
|Name|`mysqlx_enable_hello_notice`|
|Command line|`--mysqlx-enable-hello-notice[={OFF|ON}]`|
|Type of variable|`boolean`|
|Scope|`global`|
|Default value|`ON`|
|Dynamic|`true`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_enable_hello_notice](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_enable_hello_notice)|

## mysqlx_idle_worker_thread_timeout
|name|value|
|----|-----|
|Name|`mysqlx_idle_worker_thread_timeout`|
|Command line|`--mysqlx-idle-worker-thread-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`60`|
|Dynamic|`true`|
|Range|from: `0` to: `3600`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_idle_worker_thread_timeout](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_idle_worker_thread_timeout)|

## mysqlx_interactive_timeout
|name|value|
|----|-----|
|Name|`mysqlx_interactive_timeout`|
|Command line|`--mysqlx-interactive-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`28800`|
|Dynamic|`true`|
|Range|from: `1` to: `2147483`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_interactive_timeout](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_interactive_timeout)|

## mysqlx_lz4_default_compression_level
|name|value|
|----|-----|
|Name|`mysqlx_lz4_default_compression_level`|
|Command line|`--mysqlx_lz4_default_compression_level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`true`|
|Range|from: `0` to: `16`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_lz4_default_compression_level](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_lz4_default_compression_level)|

## mysqlx_lz4_max_client_compression_level
|name|value|
|----|-----|
|Name|`mysqlx_lz4_max_client_compression_level`|
|Command line|`--mysqlx_lz4_max_client_compression_level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`8`|
|Dynamic|`true`|
|Range|from: `0` to: `16`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_lz4_max_client_compression_level](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_lz4_max_client_compression_level)|

## mysqlx_max_allowed_packet
|name|value|
|----|-----|
|Name|`mysqlx_max_allowed_packet`|
|Command line|`--mysqlx-max-allowed-packet=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`67108864`|
|Dynamic|`true`|
|Range|from: `512` to: `1073741824`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_max_allowed_packet](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_max_allowed_packet)|

## mysqlx_max_connections
|name|value|
|----|-----|
|Name|`mysqlx_max_connections`|
|Command line|`--mysqlx-max-connections=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`100`|
|Dynamic|`true`|
|Range|from: `1` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_max_connections](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_max_connections)|

## mysqlx_min_worker_threads
|name|value|
|----|-----|
|Name|`mysqlx_min_worker_threads`|
|Command line|`--mysqlx-min-worker-threads=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`2`|
|Dynamic|`true`|
|Range|from: `1` to: `100`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_min_worker_threads](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_min_worker_threads)|

## mysqlx_port
|name|value|
|----|-----|
|Name|`mysqlx_port`|
|Command line|`--mysqlx-port=port_num`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`33060`|
|Dynamic|`false`|
|Range|from: `1` to: `65535`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_port](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_port)|

## mysqlx_port_open_timeout
|name|value|
|----|-----|
|Name|`mysqlx_port_open_timeout`|
|Command line|`--mysqlx-port-open-timeout=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`0`|
|Dynamic|`false`|
|Range|from: `0` to: `120`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_port_open_timeout](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_port_open_timeout)|

## mysqlx_read_timeout
|name|value|
|----|-----|
|Name|`mysqlx_read_timeout`|
|Command line|`--mysqlx-read-timeout=#`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`30`|
|Dynamic|`true`|
|Range|from: `1` to: `2147483`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_read_timeout](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_read_timeout)|

## mysqlx_socket
|name|value|
|----|-----|
|Name|`mysqlx_socket`|
|Command line|`--mysqlx-socket=file_name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`/tmp/mysqlx.sock`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_socket](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_socket)|

## mysqlx_ssl_ca
|name|value|
|----|-----|
|Name|`mysqlx_ssl_ca`|
|Command line|`--mysqlx-ssl-ca=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_ssl_ca](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_ssl_ca)|

## mysqlx_ssl_capath
|name|value|
|----|-----|
|Name|`mysqlx_ssl_capath`|
|Command line|`--mysqlx-ssl-capath=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_ssl_capath](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_ssl_capath)|

## mysqlx_ssl_cert
|name|value|
|----|-----|
|Name|`mysqlx_ssl_cert`|
|Command line|`--mysqlx-ssl-cert=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_ssl_cert](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_ssl_cert)|

## mysqlx_ssl_cipher
|name|value|
|----|-----|
|Name|`mysqlx_ssl_cipher`|
|Command line|`--mysqlx-ssl-cipher=name`|
|Type of variable|`string`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_ssl_cipher](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_ssl_cipher)|

## mysqlx_ssl_crl
|name|value|
|----|-----|
|Name|`mysqlx_ssl_crl`|
|Command line|`--mysqlx-ssl-crl=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_ssl_crl](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_ssl_crl)|

## mysqlx_ssl_crlpath
|name|value|
|----|-----|
|Name|`mysqlx_ssl_crlpath`|
|Command line|`--mysqlx-ssl-crlpath=dir_name`|
|Type of variable|`directory name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_ssl_crlpath](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_ssl_crlpath)|

## mysqlx_ssl_key
|name|value|
|----|-----|
|Name|`mysqlx_ssl_key`|
|Command line|`--mysqlx-ssl-key=file_name`|
|Type of variable|`file name`|
|Scope|`global`|
|Default value|`NULL`|
|Dynamic|`false`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_ssl_key](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_ssl_key)|

## mysqlx_wait_timeout
|name|value|
|----|-----|
|Name|`mysqlx_wait_timeout`|
|Command line|`--mysqlx-wait-timeout=#`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`28800`|
|Dynamic|`true`|
|Range|from: `1` to: `2147483`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_wait_timeout](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_wait_timeout)|

## mysqlx_write_timeout
|name|value|
|----|-----|
|Name|`mysqlx_write_timeout`|
|Command line|`--mysqlx-write-timeout=#`|
|Type of variable|`integer`|
|Scope|`session`|
|Default value|`60`|
|Dynamic|`true`|
|Range|from: `1` to: `2147483`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_write_timeout](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_write_timeout)|

## mysqlx_zstd_default_compression_level
|name|value|
|----|-----|
|Name|`mysqlx_zstd_default_compression_level`|
|Command line|`--mysqlx_zstd_default_compression_level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`3`|
|Dynamic|`true`|
|Range|from: `-131072` to: `22`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_zstd_default_compression_level](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_zstd_default_compression_level)|

## mysqlx_zstd_max_client_compression_level
|name|value|
|----|-----|
|Name|`mysqlx_zstd_max_client_compression_level`|
|Command line|`--mysqlx_zstd_max_client_compression_level=#`|
|Type of variable|`integer`|
|Scope|`global`|
|Default value|`11`|
|Dynamic|`true`|
|Range|from: `-131072` to: `22`|

### Documentation(s)
|source|anchor name|
|------|----|
|dev.mysql.com|[sysvar_mysqlx_zstd_max_client_compression_level](https://dev.mysql.com/doc/refman/8.0/en/x-plugin-options-system-variables.html#sysvar_mysqlx_zstd_max_client_compression_level)|


