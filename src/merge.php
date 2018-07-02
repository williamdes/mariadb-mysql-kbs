<?php
$dataDir = __DIR__."/../data/";
$files = glob("$dataDir*.json");

function JSONcompare($var1, $var2) {
    return json_encode($var1) === json_encode($var2);
}

function fixRange(&$current, &$cache) {
        global $newData, $key, $nbrConflictsSolved;
    $currentHasFrom = isset($current->from);
    $cacheHasFrom = isset($cache->from);
    $currentHasTo = isset($current->to);
    $cacheHasTo = isset($cache->to);
    if($currentHasFrom === false && $cacheHasFrom === true) {
        $current->from = $cache->from;
        if( JSONcompare($cache, $current) ) {
            $newData->$key = $current;
            if(JSONcompare($current, $cache)){
                $nbrConflictsSolved++;
            } else {
                fixRange($current, $cache);
            }
        }
    } else if($currentHasFrom === true && $cacheHasFrom === false) {
        $cache->from = $current->from;
        if( JSONcompare($cache, $current) ) {
            $newData->$key = $cache;
            if(JSONcompare($current, $cache)){
                $nbrConflictsSolved++;
            } else {
                fixRange($current, $cache);
            }
        }
    } else if($currentHasTo === false && $cacheHasTo === true) {
        $current->to = $cache->to;
        if( JSONcompare($cache, $current) ) {
            $newData->$key = $current;
            if(JSONcompare($current, $cache)){
                $nbrConflictsSolved++;
            } else {
                fixRange($current, $cache);
            }
        }
    } else if($currentHasTo === true && $cacheHasTo === false) {
        $cache->to = $current->to;
        if( JSONcompare($cache, $current) ) {
            $newData->$key = $cache;
            if(JSONcompare($current, $cache)){
                $nbrConflictsSolved++;
            } else {
                fixRange($current, $cache);
            }
        }
    } else if(
        ( $currentHasFrom === true && $currentHasFrom === true )
        &&
        ( $cache->from === $current->from )
    ) {
        $onlyFrom = new stdClass();
        $onlyFrom->from = $current->from;
        $newData->$key = $onlyFrom;
        $nbrConflictsSolved++;
        echo '[WARN] conflict range to - '.json_encode($cache).' - '.json_encode($current).PHP_EOL;
    } else {
        echo '[ERROR] conflict range - '.json_encode($cache).' - '.json_encode($current).PHP_EOL;
    }
}


$variables = array();

$nbr = 0;
$nbrConflicts = 0;
$nbrConflictsSolved = 0;
foreach($files as $file) {
    $fileData  = json_decode(file_get_contents($file));
    if(isset($fileData->data) === false)
        continue;
    else
        $data = $fileData->data;

    foreach($data as $doc) {
        if(isset($doc->name)) {
            if(isset($variables[$doc->name]) === false){
                if(isset($doc->ids) === false) {
                    $doc->ids = array();
                }
                $variables[$doc->name] = $doc;
                $kbEntry = new stdClass();
                $kbEntry->anchor = $doc->id;
                $kbEntry->url = $fileData->url;
                $doc->ids[] = $kbEntry;
                unset($doc->id);
            } else {
                if(isset($doc->ids) === false) {
                    $doc->ids = array();
                }
                $kbEntry = new stdClass();
                $kbEntry->anchor = $doc->id;
                $kbEntry->url = $fileData->url;
                $doc->ids[] = $kbEntry;
                unset($doc->id);
                //echo $doc->name." duplicate ! in ".str_replace($dataDir, "", $file).PHP_EOL;
                $newData = new stdClass();
                foreach((array) $doc as $key => $val){
                    if(isset($variables[$doc->name]->$key)) {
                        $cacheValue = $variables[$doc->name]->$key;
                        $docValue = $doc->$key;
                        if(
                            (
                                strtoupper( json_encode($cacheValue))
                                ===
                                strtoupper( json_encode($docValue))
                            )
                            &&
                            (
                                json_encode($cacheValue)
                                !==
                                json_encode($docValue)
                            )
                        ) {
                            $nbrConflicts++;
                            $nbrConflictsSolved++;
                            //echo 'upper conflict '.$key.' - '.json_encode($cacheValue).' - '.json_encode($docValue).PHP_EOL;
                            $docValue = strtoupper( json_encode($docValue));
                        } elseif(json_encode($cacheValue) !== json_encode($docValue)) {
                            $nbrConflicts++;
                            if ($key === "type") {
                                $realTypes = array(
                                    "string",
                                    "boolean",
                                    "integer",
                                    "numeric",
                                    "enumeration",
                                    "set",
                                    "directory name",
                                    "file name"
                                );
                                if(//original type valid
                                    in_array($cacheValue, $realTypes)// original
                                    &&
                                    in_array($docValue, $realTypes) === false// dupe
                                ) {
                                    echo 'original type valid : '.$cacheValue.PHP_EOL;
                                } elseif(// dupe type valid
                                    in_array($cacheValue, $realTypes) === false// original
                                    &&
                                    in_array($docValue, $realTypes)// dupe
                                ) {
                                    $newData->$key = $docValue;
                                    //echo 'dupe type valid : '.$docValue.PHP_EOL;
                                    $nbrConflictsSolved++;
                                } else {
                                    if(// numeric vs integer
                                        (
                                            json_encode($cacheValue) === '"numeric"'
                                            &&
                                            json_encode($docValue) === '"integer"'
                                        )
                                        ||
                                        (
                                            json_encode($cacheValue) === '"integer"'
                                            &&
                                            json_encode($docValue) === '"numeric"'
                                        )
                                    ) {
                                        //echo "integer wins !".PHP_EOL;
                                        $newData->$key = "integer";
                                        $nbrConflictsSolved++;
                                    } else {
                                        echo 'type conflict : '.json_encode($cacheValue).' - '.json_encode($docValue).PHP_EOL;
                                    }
                                }
                            } elseif ($key === "ids") {
                                /*if(isset($newData->ids) === false) {
                                    $newData->ids = array();
                                }*/
                                $newData->ids = array_merge($cacheValue, $docValue);
                                /*$source = array("_", "option-mysqld-", "sysvar-", );
                                $destination = array("-", "","");
                                if(// Replace prefix to see if same id
                                    str_replace($source, $destination, $docValue)
                                    ===
                                    str_replace($source, $destination, $cacheValue)
                                ) {
                                    $newData->$key = str_replace($source, $destination, $docValue);
                                    $nbrConflictsSolved++;// TODO: check if good idea
                                } else {
                                    echo '[ERROR] conflict id : '
                                    .json_encode($cacheValue)
                                    .' - '
                                    .json_encode($docValue)
                                    .' - '
                                    .str_replace($source, $destination, $docValue)
                                    .' - '
                                    .str_replace($source, $destination, $cacheValue)
                                    .PHP_EOL;
                                }*/
                            } elseif ($key === "default") {
                                $originalValues = array("on", "off", "ON", "OFF", "true", "false", "TRUE", "FALSE");
                                $destinationValues = array("1", "0", "1", "0", "1", "0", "1", "0");
                                $docValue = str_replace($originalValues, $destinationValues, $docValue);
                                $cacheValue = str_replace($originalValues, $destinationValues, $cacheValue);
                                if ($docValue === $cacheValue) {
                                    $newData->$key = $docValue;
                                    $nbrConflictsSolved++;
                                } else {
                                    if(
                                        is_array($cacheValue) === false
                                        &&
                                        is_array($docValue) === false
                                    ) {
                                        if(floatval($cacheValue) === floatval($docValue)) {
                                            $newData->$key = $docValue;
                                            $nbrConflictsSolved++;
                                        } else {
                                            echo '[ERROR] conflict default, not array : '.json_encode($cacheValue).' - '.json_encode($docValue).PHP_EOL;
                                        }
                                    } else {
                                        echo '[ERROR] conflict default : '.json_encode($cacheValue).' - '.json_encode($docValue).PHP_EOL;
                                    }
                                }
                            } elseif ($key === "validValues") {
                                $intersecValidValues = array_intersect($docValue, $cacheValue);
                                if (// No variables were lost in process
                                    count($intersecValidValues) === count($docValue)
                                    &&
                                    count($intersecValidValues) === count($cacheValue)
                                ) {
                                    $newData->$key = $intersecValidValues;
                                    $nbrConflictsSolved++;
                                } elseif (// Missing translation (in bytes) for 32k and 64k
                                    array_values(array_diff($docValue, $cacheValue))
                                    ===
                                    array("32768","65536")
                                ) {
                                    $intersecValidValues[] = "32768";
                                    $intersecValidValues[] = "65536";
                                    $newData->$key = $intersecValidValues;
                                    $nbrConflictsSolved++;
                                } elseif (// uppercase / lowercase
                                    strtoupper(json_encode(ksort($docValue)))
                                    ===
                                    strtoupper(json_encode(ksort($cacheValue)))
                                ) {
                                    $newData->$key = json_decode(json_encode(ksort($cacheValue)));
                                    $nbrConflictsSolved++;
                                } else {
                                    echo '[ERROR] conflict validValues : '
                                    .json_encode($cacheValue)
                                    .' - '
                                    .json_encode($docValue)
                                    .' - '
                                    .json_encode($intersecValidValues)
                                    .' - '
                                    .json_encode(array_values(array_diff($docValue, $cacheValue))).PHP_EOL;
                                }
                            } elseif ($key === "cli") {
                                $replaceSource = array("file", "dir_name", "-- ", "_");
                                $replaceDest = array("path", "path", "--", "-");
                                $replacedDocValue = str_replace($replaceSource, $replaceDest, $docValue);
                                $replacedCacheValue = str_replace($replaceSource, $replaceDest, $cacheValue);
                                if(//Try replacements
                                    str_replace($replaceSource, $replaceDest, $docValue)
                                    ===
                                    str_replace($replaceSource, $replaceDest, $cacheValue)
                                ) {
                                    $newData->$key = str_replace($replaceSource, $replaceDest, $docValue);
                                    $nbrConflictsSolved++;
                                } elseif(// Doc not well formated, missing -- before cli command
                                    str_replace("--", "", $docValue)
                                    ===
                                    str_replace("--", "", $cacheValue)
                                ) {
                                    $newData->$key = "--".str_replace("--", "", $docValue);
                                    $nbrConflictsSolved++;
                                } elseif(// More precise doc, value hint, eg: --blablabla={0|1}
                                    strlen(str_replace(str_replace("#", "", $docValue), "", $cacheValue))
                                    !==
                                    strlen($cacheValue)
                                ) {
                                    $newData->$key = $cacheValue;
                                    $nbrConflictsSolved++;
                                } elseif(// More precise doc, value hint, eg: --blablabla={0|1} using replaced values
                                    strlen(str_replace(str_replace(array("#"), array(""), $replacedDocValue), "", $replacedCacheValue))
                                    !==
                                    strlen($replacedCacheValue)
                                ) {
                                    $newData->$key = $replacedCacheValue;
                                    $nbrConflictsSolved++;
                                } elseif(// More precise doc, value hint, eg: --blablabla={0|1} using replaced values, reversed: cache/doc
                                    strlen(str_replace(str_replace(array("#"), array(""), $replacedCacheValue), "", $replacedDocValue))
                                    !==
                                    strlen($replacedDocValue)
                                ) {
                                    $newData->$key = $replacedDocValue;
                                    $nbrConflictsSolved++;
                                } elseif(// More precise doc, value hint, eg: --blablabla={0|1}
                                    strlen(str_replace(str_replace(array("#"), array(""), $docValue), "", $cacheValue))
                                    !==
                                    strlen($cacheValue)
                                ) {
                                    $newData->$key = $cacheValue;
                                    $nbrConflictsSolved++;
                                } elseif(// contained in cache
                                    strlen(str_replace($cacheValue, "", $docValue))
                                    !==
                                    strlen($docValue)
                                ) {
                                    $newData->$key = $docValue;
                                    $nbrConflictsSolved++;
                                } elseif(// contained in conflict
                                    strlen(str_replace($docValue, "", $cacheValue))
                                    !==
                                    strlen($cacheValue)
                                ) {
                                    $newData->$key = $cacheValue;
                                    $nbrConflictsSolved++;
                                } else {
                                    echo '[ERROR] conflict cli : cacheValue: '
                                    .json_encode($cacheValue)
                                    .' - docValue: '
                                    .json_encode($docValue)
                                    .' - docValue: '
                                    .str_replace($replaceSource, $replaceDest, $docValue)
                                    .' - cacheValue: '
                                    .str_replace($replaceSource, $replaceDest, $cacheValue)
                                    .PHP_EOL;
                                }
                            } elseif($key === "range") {
                                $current = $docValue;
                                $cache = $cacheValue;
                                fixRange($current, $cache);
                            } else {
                                echo '[ERROR] conflict '.$key.' + '.$doc->name.' - '.json_encode($cacheValue).' - '.json_encode($docValue).PHP_EOL;
                            }
                        } else {
                            $newData->$key = $val;
                        }
                    } else {
                        $newData->$key = $val;
                    }
                }
                //print_r($newData);
                $variables[$doc->name] = $newData;
            }
        } else {
            //print_r($doc);
        }
    }
    $nbr += count($data);
}
echo "NBR: ".$nbr.PHP_EOL;
echo "NBR_UNIQUE: ".count($variables).PHP_EOL;
echo "NBR_CONFLICTS: ".$nbrConflicts.PHP_EOL;
echo "NBR_CONFLICTS_SOLVED: ".$nbrConflictsSolved.PHP_EOL;
echo "NBR_CONFLICTS_REMAINING: ".($nbrConflicts-$nbrConflictsSolved).PHP_EOL;

$fileOut = new stdClass();
$fileOut->vars = json_decode(json_encode($variables));
file_put_contents(__DIR__."/../dist/merged-raw.json", json_encode($fileOut, JSON_PRETTY_PRINT|JSON_UNESCAPED_UNICODE|JSON_UNESCAPED_SLASHES));

$fileOut->urls = array();

foreach($fileOut->vars as $id => $doc) {
    foreach($doc->ids as &$kbEntry) {
        $urlId = array_search($kbEntry->url, $fileOut->urls, true);
        if($urlId === false){
            $urlId = array_push($fileOut->urls, $kbEntry->url);
        }
        $kbEntry->url = $urlId;
        $kbEntry = "$urlId#$kbEntry->anchor";
    }
}

file_put_contents(__DIR__."/../dist/merged-slim.json", json_encode($fileOut, JSON_PRETTY_PRINT|JSON_UNESCAPED_UNICODE|JSON_UNESCAPED_SLASHES));

$fileOut->vars = $variables;
$fileOut->types = array( "MYSQL" => 1, "MARIADB" => 2 );
foreach($fileOut->vars as $id => &$doc) {
    $links = array();
    foreach($doc->ids as &$kbEntry) {
        $urlId = array_search($kbEntry->url, $fileOut->urls, true);
        if($urlId === false){
            $urlId = array_push($fileOut->urls, $kbEntry->url);
        }
        $kbEntryMin = new stdClass();
        $kbEntryMin->a = $kbEntry->anchor;
        $kbEntryMin->u = $urlId;
        if(preg_match("/mysql\.com/",$kbEntry->url))
            $kbEntryMin->t = $fileOut->types["MYSQL"];
        elseif(preg_match("/mariadb\.com/",$kbEntry->url))
            $kbEntryMin->t = $fileOut->types["MARIADB"];
        $links[] = $kbEntryMin;
    }
    $doc = $links;
}
$fileOut->types = array_flip($fileOut->types);
file_put_contents(__DIR__."/../dist/merged-ultraslim.json", json_encode($fileOut, JSON_UNESCAPED_UNICODE|JSON_UNESCAPED_SLASHES));

echo "Files merged !".PHP_EOL;
