<?php
declare(strict_types = 1);
namespace Williamdes\MariaDBMySQLKBS;

use \PHPUnit\Framework\TestCase;
use \Swaggest\JsonSchema\Schema;
use \Swaggest\JsonSchema\Context;
use \Swaggest\JsonSchema\RemoteRefProvider;
use \Exception;
use \stdClass;

class RefProvider implements RemoteRefProvider {

    /**
     * Preloaded urn schemas
     *
     * @var array
     */
    private $urnSchemas = [];

    public function __construct(){
        foreach (glob(__DIR__ . "/../schemas/*.json") as $filename) {
            $schema = json_decode(file_get_contents($filename));
            if(isset($schema) && isset($schema->{'$id'})) {
                $this->urnSchemas[$schema->{'$id'}] = $schema;
            }
        }
    }

    /**
     * @param string $url
     * @return \stdClass|false json_decode of $url resource content
     */
    public function getSchemaData($url)
    {
        if(isset($this->urnSchemas[$url])) {// Handle urn: urls
            return $this->urnSchemas[$url];
        } else if(is_file($url)) {// Handle file
            return json_decode(file_get_contents($url));
        } else {// Handle URL
            return json_decode(file_get_contents(rawurldecode($url)));
        }
    }
};
class DataTest extends TestCase
{

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

        $options = new Context();
        $options->setRemoteRefProvider(new RefProvider());
        $schema = Schema::import($id, $options);
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
        $slimDataTestData = json_decode((string) file_get_contents(__DIR__."/data/ultraSlimDataTestWithVariables.json"));
        $this->assertTrue(self::validate($slimDataTestData, "urn:williamdes:mariadb-mysql-kbs:ultraslimdata"));
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
