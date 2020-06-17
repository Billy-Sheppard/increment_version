#! /bin/bash

../increment_version -d --no-update -v 0.0.1 > /dev/null
../increment_version -d --no-update -v 0.0.1 -a > /dev/null

../increment_version -d --no-update -p > /dev/null
echo "Patching Cargo.toml"
if ! cat ./Cargo.toml | grep -q '0.0.2'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi
../increment_version -d --no-update -n > /dev/null
echo "Minor Updating Cargo.toml"
if ! cat ./Cargo.toml | grep -q '0.1.2'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi
../increment_version -d --no-update -m > /dev/null
echo "Major Updating Cargo.toml"
if ! cat ./Cargo.toml | grep -q '1.1.2'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi
../increment_version -d --no-update -v 0.0.1 > /dev/null
echo "Setting Cargo.toml Version to 0.0.1"
if ! cat ./Cargo.toml | grep -q '0.0.1'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi

../increment_version -d --no-update -p -a > /dev/null
echo "Patching Version.toml"
if ! cat ./Version.toml | grep -q '0.0.2'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi
../increment_version -d --no-update -n -a > /dev/null
echo "Minor Updating Version.toml"
if ! cat ./Version.toml | grep -q '0.1.2'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi
../increment_version -d --no-update -m -a > /dev/null
echo "Major Updating Version.toml"
if ! cat ./Version.toml | grep -q '1.1.2'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi
../increment_version -d --no-update -v 0.0.1 -a > /dev/null
echo "Setting Version.toml Version to 0.0.1"
if ! cat ./Version.toml | grep -q '0.0.1'; then
    echo "  Failed"
    exit 1
else
    echo "  Success"
fi