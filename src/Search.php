<?php
declare(strict_types = 1);
namespace Williamdes\MariaDBMySQLKBS;

use \Exception;

class Search
{

    /**
     * Loaded data
     *
     * @var mixed
     */
    public static $data;

    /**
     * Data is loaded
     *
     * @var boolean
     */
    public static $loaded = false;

    public const ANY     = -1;
    public const MYSQL   = 1;
    public const MARIADB = 2;

    /**
     * Load data from disk
     *
     * @return void
     */
    public static function loadData(): void
    {
        $DS = DIRECTORY_SEPARATOR;
        if (Search::$loaded === false) {
            $contents = file_get_contents(
                __DIR__.$DS."..".$DS."dist".$DS."merged-ultraslim.json"
            );
            if ($contents === false) {
                throw new Exception("File not found !");
            }
            Search::$data   = json_decode($contents);
            Search::$loaded = true;
        }
    }

    /**
     * get the first link to doc available
     *
     * @param string $name Name of variable
     * @param int    $type (optional) Type of link Search::MYSQL/Search::MARIADB/Search::ANY
     * @return string
     */
    public static function getByName(string $name, int $type = Search::ANY): string
    {
        self::loadData();
        if (isset(Search::$data->vars->{$name})) {
            $kbEntrys = Search::$data->vars->{$name};
            $kbEntry  = null;
            foreach ($kbEntrys as $kbEntry) {
                if ($type === Search::ANY) {
                    return Search::$data->urls[$kbEntry->u]."#".$kbEntry->a;
                } elseif ($type === Search::MYSQL) {
                    if ($kbEntry->t === Search::MYSQL) {
                        return Search::$data->urls[$kbEntry->u]."#".$kbEntry->a;
                    }
                } elseif ($type === Search::MARIADB) {
                    if ($kbEntry->t === Search::MARIADB) {
                        return Search::$data->urls[$kbEntry->u]."#".$kbEntry->a;
                    }
                }
            }

            throw new Exception("$name does not exist for this type of documentation !");
        } else {
            throw new Exception("$name does not exist !");
        }
    }

}
