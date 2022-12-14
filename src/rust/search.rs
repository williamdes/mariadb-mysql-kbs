use std::env;
use std::fs;

use crate::merged_ultraslim::MergedUltraSlim;

pub fn load_data() -> MergedUltraSlim {
    let source_dir = env::current_dir().unwrap();
    const FILE_NAME: &str = "merged-ultraslim.json";
    let file_contents =
        fs::read_to_string(source_dir.to_str().unwrap().to_owned() + "/dist/" + FILE_NAME)
            .expect("Should have been able to read the data file");
    serde_json::from_str(&file_contents).expect("JSON data could not be decoded !")
}

#[cfg(test)]
mod tests {
    use crate::merged_ultraslim::SearchType;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_data() {
        let data = load_data();

        assert_eq!(
            true,
            data.get_by_name("adadadadad", SearchType::MariaDB).is_err()
        );
        assert_eq!(
            true,
            data.get_by_name("adadadadad", SearchType::MySQL).is_err()
        );
        assert_eq!(
            true,
            data.get_by_name("adadadadad", SearchType::Any).is_err()
        );

        assert_eq!(
            "https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-orig",
            data.get_by_name("ndb_log_orig", SearchType::MySQL).unwrap().as_str()
        );
        assert_eq!(
            "https://dev.mysql.com/doc/refman/5.7/en/mysql-cluster-options-variables.html#option_mysqld_ndb-log-orig",
            data.get_by_name("ndb_log_orig", SearchType::Any).unwrap().as_str()
        );
        assert_eq!(
            true,
            data.get_by_name("ndb_log_orig", SearchType::MariaDB)
                .is_err()
        );

        assert_eq!(
            "https://dev.mysql.com/doc/refman/8.0/en/replication-options.html#sysvar_server_id",
            data.get_by_name("server_id", SearchType::MySQL)
                .unwrap()
                .as_str()
        );
        assert_eq!(
            "https://mariadb.com/kb/en/library/documentation/gtid/#server_id",
            data.get_by_name("server_id", SearchType::MariaDB)
                .unwrap()
                .as_str()
        );
        assert_eq!(
            "https://mariadb.com/kb/en/library/documentation/gtid/#server_id",
            data.get_by_name("server_id", SearchType::Any)
                .unwrap()
                .as_str()
        );
    }
}
/*

     * get the type of the variable
     *
     * @param string $name Name of variable
     * @return string
     * @throws KBException

    public static function getVariableType(string $name): string
    {
        self::loadData();
        $kbEntry = self::getVariable($name);
        if (isset($kbEntry->t)) {
            return Search::$data->varTypes->{$kbEntry->t};
        } else {
            throw new KBException($name . ' does have a known type !');
        }
    }


     * Return the list of static variables
     *
     * @return array<int,string>

    public static function getStaticVariables(): array
    {
        return self::getVariablesWithDynamic(false);
    }


     * Return the list of dynamic variables
     *
     * @return array<int,string>

    public static function getDynamicVariables(): array
    {
        return self::getVariablesWithDynamic(true);
    }


     * Return the list of variables having dynamic = $dynamic
     *
     * @param bool $dynamic dynamic=true/dynamic=false
     * @return array<int,string>

    public static function getVariablesWithDynamic(bool $dynamic): array
    {
        self::loadData();
        $staticVars = [];
        foreach (Search::$data->vars as $name => $var) {
            if (isset($var->d)) {
                if ($var->d === $dynamic) {
                    $staticVars[] = $name;
                }
            }
        }
        return $staticVars;
    }

}
*/
