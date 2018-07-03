<?php
declare(strict_types = 1);
namespace Williamdes\MariaDBMySQLKBS;

use \PHPUnit\Framework\TestCase;
use \Williamdes\MariaDBMySQLKBS\Search;

class SearchTest extends TestCase
{

    /**
     * test get by name
     *
     * @return void
     */
    public function testGetByName(): void
    {
        $found = Search::getByName("max_relay_log_size");
        $this->assertContains("http", $found);
        $this->assertContains("://", $found);
        $this->assertContains("#", $found);
    }

    /**
     * test get by name for MySQL
     *
     * @return void
     */
    public function testGetByNameMYSQL(): void
    {
        $found = Search::getByName("max_relay_log_size", Search::MYSQL);
        $this->assertContains("http", $found);
        $this->assertContains("://", $found);
        $this->assertContains("mysql.com", $found);
        $this->assertContains("#", $found);

        $found = Search::getByName("innodb_compression_level", Search::MYSQL);
        $this->assertContains("http", $found);
        $this->assertContains("://", $found);
        $this->assertContains("mysql.com", $found);
        $this->assertContains("#", $found);
    }

    /**
     * test get by name for MARIADB
     *
     * @return void
     */
    public function testGetByNameMARIADB(): void
    {
        $found = Search::getByName("use_stat_tables", Search::MARIADB);
        $this->assertContains("http", $found);
        $this->assertContains("://", $found);
        $this->assertContains("mariadb.com", $found);
        $this->assertContains("#", $found);

        $found = Search::getByName("innodb_compression_level", Search::MARIADB);
        $this->assertContains("http", $found);
        $this->assertContains("://", $found);
        $this->assertContains("mariadb.com", $found);
        $this->assertContains("#", $found);
    }

    /**
     * test get by name
     *
     * @expectedException     Exception
     * @expectedExceptionCode 0
     * @expectedExceptionMessageRegExp /(.+) does not exist for this type of documentation !/
     *
     * @return void
     */
    public function testException(): void
    {
        Search::getByName("mysql_native_password_proxy_users", Search::MARIADB);
    }

    /**
     * test get by name not found variable
     *
     * @expectedException     Exception
     * @expectedExceptionCode 0
     * @expectedExceptionMessageRegExp /(.+) does not exist !/
     *
     * @return void
     */
    public function testExceptionNoFoundGetVariableType(): void
    {
        Search::getVariableType("acbdefghi0202");
    }

    /**
     * test get by name not found variable
     *
     * @expectedException     Exception
     * @expectedExceptionCode 0
     * @expectedExceptionMessageRegExp /(.+) does not exist !/
     *
     * @return void
     */
    public function testExceptionNoFound(): void
    {
        Search::getByName("acbdefghi0202", Search::MARIADB);
    }

    /**
     * test load data fail
     *
     * @runInSeparateProcess
     * @expectedException     Exception
     * @expectedExceptionCode 0
     * @expectedExceptionMessageRegExp /(.+) does not exist !/
     *
     * @return void
     */
    public function testExceptionLoadData(): void
    {
        Search::$DATA_DIR = ".";
        Search::$loaded   = false;
        Search::loadData();
    }

    /**
     * test get variables with dynamic status
     *
     * @return void
     */
    public function testGetVariablesWithDynamic(): void
    {
        $dynamic = Search::getVariablesWithDynamic(true);
        $this->assertEquals($dynamic, Search::getDynamicVariables());
        $static = Search::getVariablesWithDynamic(false);
        $this->assertEquals($static, Search::getStaticVariables());
        $this->assertGreaterThan(10, count($dynamic));
        $this->assertGreaterThan(10, count($static));
        $common = \array_intersect($dynamic, $static);
        $this->assertEquals(0, count($common));// Impossible to be dynamic and not
    }

    /**
     * test Exception get variable type has no type
     *
     * @expectedException     Exception
     * @expectedExceptionCode 0
     * @expectedExceptionMessageRegExp /(.+) does have a known type !/
     *
     * @return void
     */
    public function testExceptionGetVariableType(): void
    {
        Search::getVariableType("wsrep_forced_binlog_format");
    }

    /**
     * test get variable type
     *
     * @return void
     */
    public function testGetVariableType(): void
    {
        $type = Search::getVariableType("innodb_stats_persistent");
        $this->assertEquals("boolean", $type);
    }

}
