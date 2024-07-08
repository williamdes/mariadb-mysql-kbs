#!/bin/bash
# Source: https://github.community/t5/How-to-use-Git-and-GitHub/How-to-create-full-release-from-command-line-not-just-a-tag/m-p/16072/highlight/true#M4943

set -e

version=$1

if [ -z "$version" ]; then
    echo 'Version is empty. Please add a version as first argument';
    exit 1;
fi

if [[ "$version" =~ [a-z]+ ]]; then
    echo 'Version must not contain any letter, "v" prefix is added in the script.';
    exit 1;
fi

if ! command -v jq &> /dev/null
then
    echo "jq could not be found"
    exit
fi

if ! command -v gpg &> /dev/null
then
    echo "gpg could not be found"
    exit
fi

if ! command -v curl &> /dev/null
then
    echo "curl could not be found"
    exit
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

check_token() {
    RELEASES="$(curl -s -H "Authorization: token $token" -H "Accept: application/vnd.github+json" "https://api.github.com/repos/$user/$repo/releases")"

    if [ "$(echo "${RELEASES}" | jq -r 'if type=="object" then has("message") else "false" end')" != "false" ]; then
        echo "Your token is invalid !" > /dev/stderr
        echo "invalid: $RELEASES" > /dev/stderr
        echo 'Please run: git config --add github.token "new_token"';
        exit 1
    fi

    echo 'Token is valid.'
}

function generate_post_data {
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

function uploadArtifact {
    FILE_NAME="$1"
    MIME_TYPE="$2"
    ARTIFACT_OUT=$(curl -# -S -s -L \
        -H "Authorization: token ${token}" \
        -H "Content-Type: ${MIME_TYPE}" \
        --data-binary @${FILE_NAME} \
        "https://uploads.github.com/repos/$user/$repo/releases/$releaseId/assets?name=${FILE_NAME}")
    echo "Uploaded: $(echo $ARTIFACT_OUT | jq -r '.name')."
}

check_token

tmp=$(mktemp)

echo "Update changelog file"
NEXT_VERSION="v$version" npm run changelog-file-next
git diff

read -r -p "Are you sure to commit the diff? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    git add CHANGELOG.md
    git commit --allow-empty --message "update: changelog" --message "[changelog skip]"
fi

echo "Change package versions"

jq --indent 4 --arg a "$version" '.version = $a' package.json > "$tmp" && mv "$tmp" package.json
if [ -f package-lock.json ]; then
    jq --indent 4 --arg a "$version" '.version = $a' package-lock.json > "$tmp" && mv "$tmp" package-lock.json
fi
echo "Here is the diff"
git diff

read -r -p "Are you sure to commit the diff? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    git add package.json
    if [ -f package-lock.json ]; then
        git add package-lock.json
    fi
    git commit --allow-empty --message "update: package version to $version" --message "[changelog skip]"
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

echo "Publish on crates.io (dry-run)"
npm publish --dry-run
read -r -p "Are you sure to publish on crates.io? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    cargo publish
    echo "Published on npm"
fi

echo "Create release $version for repo: $user/$repo branch: $branch"
read -r -p "Are you sure to publish the draft? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    RELEASE_OUT="$(curl -H "Authorization: token $token" -H "Accept: application/vnd.github+json" --data "$(generate_post_data)" "https://api.github.com/repos/$user/$repo/releases")"
fi

GPG_KEY=${GPG_KEY:-C4D91FDFCEF6B4A3C653FD7890A0EF1B8251A889}

echo "Make and publish the signature for this release."
read -r -p "Are you sure to sign and upload to the draft? [Y/n]" response
response=${response,,} # tolower
if [[ $response =~ ^(yes|y| ) ]] || [[ -z $response ]]; then
    curl -L https://github.com/$user/$repo/archive/v$version.tar.gz -O
    gpg --detach-sign --armor --local-user "${GPG_KEY}" v$version.tar.gz
    releaseId=$(echo $RELEASE_OUT | jq -r '.id')
    echo "Created release ($releaseId): $(echo $RELEASE_OUT | jq -r '.html_url')"
    uploadArtifact "v$version.tar.gz.asc" 'application/pgp-signature'
fi

echo "Done."
