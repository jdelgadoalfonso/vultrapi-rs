#!/bin/bash

echo "Do not forget to run 'cargo clean' before executing this script"

export OPENSSL_INCLUDE_DIR=/usr/include/
export OPENSSL_LIB_DIR=/usr/lib/ 

OPENSSL_STATIC=yes cargo build $1

