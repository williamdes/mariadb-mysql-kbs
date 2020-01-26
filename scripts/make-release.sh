#!/bin/bash
# Source: https://github.community/t5/How-to-use-Git-and-GitHub/How-to-create-full-release-from-command-line-not-just-a-tag/m-p/16072/highlight/true#M4943

version=$1

if [ -z "$version" ]; then
    echo 'Version is empty. Please add a version as first argument';
    exit 1;
fi

if [[ "$version" =~ [a-z]+ ]]; then
    echo 'Version must not contain any letter, "v" prefix is added in the script.';
    exit 1;
fi

branch=$(git rev-parse --abbrev-ref HEAD)
token=$(git config github.token)

if [ -z "$token" ]; then
    echo 'Token is empty. Please run git config --add github.token "123"';
    exit 1;
fi

repo_full_name=$(git config --get remote.origin.url)
url=$repo_full_name
re="^(https|git)(:\/\/|@)([^\/:]+)[\/:]([^\/:]+)\/(.+).git$"

if [[ $url =~ $re ]]; then
    protocol=${BASH_REMATCH[1]}
    separator=${BASH_REMATCH[2]}
    hostname=${BASH_REMATCH[3]}
    user=${BASH_REMATCH[4]}
    repo=${BASH_REMATCH[5]}
fi


generate_post_data()
{
  cat <<EOF
{
  "tag_name": "v$version",
  "target_commitish": "$branch",
  "name": "v$version",
  "body": "Version $version",
  "draft": true,
  "prerelease": false
}
EOF
}

tmp=$(mktemp)

echo "Update changelog file"
NEXT_VERSION="$version" npm run changelog-file-next
git diff

read -r -p "Are you sure to commit the diff? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    git add CHANGELOG.md
    git commit --message "update: changelog" --message "[changelog skip]"
fi

echo "Change package versions"

jq --indent 4 --arg a "$version" '.version = $a' composer.json > "$tmp" && mv "$tmp" composer.json
jq --indent 4 --arg a "$version" '.version = $a' package.json > "$tmp" && mv "$tmp" package.json
if [ -f package-lock.json ]; then
    jq --indent 4 --arg a "$version" '.version = $a' package-lock.json > "$tmp" && mv "$tmp" package-lock.json
fi
echo "Here is the diff"
git diff

read -r -p "Are you sure to commit the diff? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    git add composer.json package.json
    if [ -f package-lock.json ]; then
        git add package-lock.json
    fi
    git commit --message "update: package version to $version" --message "[changelog skip]"
fi

echo "Make tag"
git tag -f -s -a -m "v$version" "v$version"
git show "v$version"
read -r -p "Are you sure to push the tag? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    git push origin "v$version"
    echo "Tag pushed"
fi

echo "Publish on npm (dry-run)"
npm publish --dry-run
read -r -p "Are you sure to publish on npm? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    npm publish
    echo "Published on npm"
fi

echo "Create release $version for repo: $user/$repo branch: $branch"
read -r -p "Are you sure to publish the draft? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    curl --data "$(generate_post_data)" "https://api.github.com/repos/$user/$repo/releases?access_token=$token"
fi