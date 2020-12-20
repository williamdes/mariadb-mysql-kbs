<?php
declare(strict_types = 1);
/**
 * This file has been generated for code-lts/doctum
 */
use Doctum\Doctum;
use Doctum\RemoteRepository\GitHubRemoteRepository;
use Symfony\Component\Finder\Finder;

$srcFiles = realpath(__DIR__ . '/../src');
$rootDir = realpath(__DIR__ . '/../') . DIRECTORY_SEPARATOR;

$iterator = Finder::create()->files()->name('*.php')->in($srcFiles);

$description = json_decode(file_get_contents(__DIR__ . '/../composer.json'))->description;

return new Doctum(
    $iterator, [
        'title'       => $description,
        'build_dir'   => __DIR__ . '/../docs/build',
        'cache_dir'   => __DIR__ . '/../tmp',
        'source_dir'  => $rootDir,
        'remote_repository'    => new GitHubRemoteRepository('williamdes/mariadb-mysql-kbs', $rootDir),
        'footer_link'          => [
            'href' => 'https://github.com/williamdes/mariadb-mysql-kbs#readme',
            'target'      => '_blank',
            'rel'         => 'noopener',
            'before_text' => 'Go back to the',
            'link_text'   => 'MariaDB and MySQL Knowledge bases',
            'after_text'  => 'project',
        ]
    ]
);
