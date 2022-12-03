#!/usr/bin/env bash

set -e

rootdir=$(readlink -f "$(dirname $0)")
currdir=$(pwd)

sed_opts=
case "$OSTYPE" in
    darwin*)
        sed_opts="-g"
        ;;
    *)
        sed_opts=
        ;;
esac

function check_dependencies () {
    for bin in protoc protoc-gen-rust protoc-gen-rust-grpc; do
        if [ ! -x "$(which ${bin})" ]; then
            echo "[Error] Please check if you have installed ${bin} in your \$PATH."
            echo "    protoc:                 https://developers.google.com/protocol-buffers/"
            echo "    protoc-gen-rust:        https://crates.io/crates/protobuf"
            exit 1
        fi
    done
}

function remove_all_rs () {
    find ./src/protos -name "*.rs" -exec rm -v {} \;
}

function gen_rs_for_protos () {
    find ./proto -name "*.proto" | while read protofile; do
        protoc ${protofile} --proto_path ./proto --rust_out ./src/protos
    done
}

function add_license () {
    for i in `find ./src/protos -name "*.rs"`
    do
        if grep -q -e "Copyright 2015-20.. Parity Technologies" -e "Copyright 2016-20.. Rivtower Technologies" $i
        then
            echo "Ignoring the " $i
        else
            echo "Starting modify" $i
            (cat ../LICENSE_HEADER | cat - $i > file1) && mv file1 $i
        fi
    done
}

function generate_readme () {
    cat <<EOF
// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

EOF
}

function gen_modrs_for_protos () {
    local modrs="./src/protos/mod.rs"
    generate_readme > "${modrs}"
    find ./proto -maxdepth 1 -name "*.proto" \
            -exec basename {} \; \
            | sort \
            | cut -d"." -f 1 | while read name; do
        echo "pub mod ${name};" >> "${modrs}"
    done
    echo >> "${modrs}"
    find ./proto -maxdepth 1 -name "*.proto" \
            -exec basename {} \; \
            | sort \
            | cut -d"." -f 1 | while read name; do
        items=$(grep "^pub [se].* {$" "./src/protos/${name}.rs" | sort | awk '{ printf $3", " }')
        echo "pub use self::${name}::{${items/%, }};" >> "${modrs}"
    done
}

function camelcase_to_underscore () {
    echo "$1" | sed -e 's/\([[:upper:]]\)/_\L\1/g' -e 's/^_//'
}

function main () {
    cd "${rootdir}"
    check_dependencies
    remove_all_rs
    gen_rs_for_protos
    gen_modrs_for_protos
    cd "${currdir}"
}

main
