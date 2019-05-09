<?php
declare(strict_types = 1);
namespace Williamdes\MariaDBMySQLKBS;

use \PHPUnit\Framework\TestCase;
use \Swaggest\JsonSchema\Schema;
use \Exception;
use \stdClass;

class DataTest extends TestCase
{
    private static $schemas = [];

    /**
     * Load all schemas in memory
     *
     * @return void
     */
    public static function setUpBeforeClass(): void
    {
        foreach (glob(__DIR__."/../schemas/*.json") as $filename) {
            $doc                          = json_decode((string) file_get_contents($filename));
            self::$schemas[$doc->{'$id'}] = $doc;
        }
    }

    /**
     * Validate json data
     *
     * @param stdClass $contents The file contents
     * @param string   $id       The schema id
     * @example validate($slimData, "urn:williamdes:mariadb-mysql-kbs:slimdata");
     * @return bool
     */
    public static function validate(stdClass $contents, string $id): bool
    {
        if (isset(self::$schemas[$id]) === false) {
            throw new Exception("No schema found !");
        }
        $schema = Schema::import(self::$schemas[$id]);
        $schema->in($contents);
        return true;// No exception occured
    }

    /**
     * test files
     *
     * @return void
     */
    public function testFileSample(): void
    {
        $slimDataTestData = json_decode((string) file_get_contents(__DIR__."/data/slimDataTestWithVariables.json"));
        $this->assertTrue(self::validate($slimDataTestData, "urn:williamdes:mariadb-mysql-kbs:slimdata"));
    }

    /**
     * test slim data
     *
     * @return void
     */
    public function testFileSlim(): void
    {
        $slimData = json_decode((string) file_get_contents(__DIR__."/../dist/merged-slim.json"));
        $this->assertTrue(self::validate($slimData, "urn:williamdes:mariadb-mysql-kbs:slimdata"));
    }

    /**
     * test ultra slim data
     *
     * @return void
     */
    public function testFileUltraSlim(): void
    {
        $slimData = json_decode((string) file_get_contents(__DIR__."/../dist/merged-ultraslim.json"));
        $this->assertTrue(self::validate($slimData, "urn:williamdes:mariadb-mysql-kbs:ultraslimdata"));
    }

    /**
     * test ultra slim data
     *
     * @return void
     */
    public function testFileRaw(): void
    {
        $slimData = json_decode((string) file_get_contents(__DIR__."/../dist/merged-raw.json"));
        $this->assertTrue(self::validate($slimData, "urn:williamdes:mariadb-mysql-kbs:rawdata"));
    }
}
