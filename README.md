# Increment Version
A Rust Utility for updating Cargo.toml or Version.toml files


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