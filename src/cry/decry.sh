#!/bin/bash

block_cipher=$1
block_cipher_1_as_iv=$2
key=$3


echo -n "$block_cipher" | xxd -p -r | openssl aes-256-cbc -d -nopad -K "$key" -iv "$block_cipher_1_as_iv" | xxd -c 16
