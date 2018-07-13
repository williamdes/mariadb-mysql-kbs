<?php
declare(strict_types = 1);
namespace Williamdes\MariaDBMySQLKBS;

use \Swaggest\JsonSchema\Schema;
use \Exception;

require_once(__DIR__.'/../vendor/autoload.php');

$schemas = [];

foreach (glob(__DIR__."/../schemas/*.json") as $filename) {
    $doc                    = json_decode((string) file_get_contents($filename));
    $schemas[$doc->{'$id'}] = $doc;
}

/**
 * Validate json data
 *
 * @param string $contents The file contents
 * @param string $id       The schema id
 * @example validate($slimData, "urn:williamdes:mariadb-mysql-kbs:slimdata");
 * @return void
 */
function validate(string $contents, string $id)
{
    global $schemas;
    if (trim($contents) === '') {
        throw new Exception("Empty data !");
    }
    if (isset($schemas[$id]) === false) {
        throw new Exception("No schema found !");
    }
    $schema = Schema::import($schemas[$id]);
    $schema->in($contents);
}

$slimData = json_decode((string) file_get_contents(__DIR__."/../dist/merged-ultraslim.json"));
/*
switch (json_last_error()) {
    case JSON_ERROR_NONE:
        echo ' - Aucune erreur';
    break;
    case JSON_ERROR_DEPTH:
        echo ' - Profondeur maximale atteinte';
    break;
    case JSON_ERROR_STATE_MISMATCH:
        echo ' - Inadéquation des modes ou underflow';
    break;
    case JSON_ERROR_CTRL_CHAR:
        echo ' - Erreur lors du contrôle des caractères';
    break;
    case JSON_ERROR_SYNTAX:
        echo ' - Erreur de syntaxe ; JSON malformé';
    break;
    case JSON_ERROR_UTF8:
        echo ' - Caractères UTF-8 malformés, probablement une erreur d\'encodage';
    break;
    default:
        echo ' - Erreur inconnue';
    break;
}

echo PHP_EOL;
*/
validate($slimData, "urn:williamdes:mariadb-mysql-kbs:slimdata");
