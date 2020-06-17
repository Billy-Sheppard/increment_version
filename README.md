
# Increment Version
A Rust Utility for updating Cargo.toml or Version.toml files (Linux)

[![Build Status](https://dev.azure.com/billyjsheppard/increment_version/_apis/build/status/Billy-Sheppard.increment_version?branchName=master)](https://dev.azure.com/billyjsheppard/increment_version/_build/latest?definitionId=1&branchName=master)

## Download
Recommend Install Location: '/usr/local/bin/increment_version'

https://github.com/billy-sheppard/increment_version/releases/latest/download/increment_versio

## Usage

    -h: For help

    -m: For major version increase

    -n: For minor version increase

    -p: For patch version increase

    -sf [subfolder]: For .toml in a subfolder

    -v [version]: For specific version increase
            - will only complete if a valid SemVer string (no quotes) is passed

    -a: For using a Version.toml file instead of Cargo.toml
    
    -t: For automatically tagging with the format v{version}, committing, and pushing to git remote

    --no-update: Skips checking for an updated version

    -d: Returns some extra output