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
        $this->assertContains("http",$found);
        $this->assertContains("://",$found);
        $this->assertContains("#",$found);
    }

    /**
     * test get by name for MySQL
     *
     * @return void
     */
    public function testGetByNameMYSQL(): void
    {
        $found = Search::getByName("max_relay_log_size", Search::MYSQL);
        $this->assertContains("http",$found);
        $this->assertContains("://",$found);
        $this->assertContains("mysql.com",$found);
        $this->assertContains("#",$found);

        $found = Search::getByName("innodb_compression_level", Search::MYSQL);
        $this->assertContains("http",$found);
        $this->assertContains("://",$found);
        $this->assertContains("mysql.com",$found);
        $this->assertContains("#",$found);
    }

    /**
     * test get by name for MARIADB
     *
     * @return void
     */
    public function testGetByNameMARIADB(): void
    {
        $found = Search::getByName("use_stat_tables", Search::MARIADB);
        $this->assertContains("http",$found);
        $this->assertContains("://",$found);
        $this->assertContains("mariadb.com",$found);
        $this->assertContains("#",$found);

        $found = Search::getByName("innodb_compression_level", Search::MARIADB);
        $this->assertContains("http",$found);
        $this->assertContains("://",$found);
        $this->assertContains("mariadb.com",$found);
        $this->assertContains("#",$found);
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
}
