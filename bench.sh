#! /bin/bash

quit="echo :q |"
file="../crablit_old/crablit_before_prs/files/big.txt"
prog="./target/release/crablit"
warm="-w 2"

hyperfine "$quit $prog $file" "$quit $prog -d ';' -m cards $file" $warm
