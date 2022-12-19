#!/usr/bin/env bash

# Enable error options
set -Eeuo pipefail

# Enable debug
#set -x

# Use `cargo asm --lib exper_allocator_api` to see list of functions
gen_lib_asm () {
    cargo asm --rust --lib "exper_allocator_api::$1" > asm/$1.txt
    #cargo asm --lib "exper_allocator_api::$1" > asm/$1.txt
}

# Use `cargo asm --bin exper_allocator_api` to see list of functions
gen_bin_asm () {
    cargo asm --rust --bin exper_allocator_api "exper_allocator_api::$1" > asm/$1.txt
    #cargo asm --bin exper_allocator_api "exper_allocator_api::$1" > asm/$1.txt
}

# Use `cargo asm --bench iai` to see list of functions
gen_iai_asm() {
    cargo asm --rust --bench iai "iai::iai_wrappers::$1" > asm/$1.txt
    #cargo asm --bench iai "iai::iai_wrappers::$1" > asm/$1.txt
}

# Use `cargo asm --bench crit` to see list of functions
gen_crit_asm() {
    cargo asm --rust --bench crit "crit::$1" > asm/$1.txt
    #cargo asm --bench crit "crit::$1" > asm/$1.txt
}

gen_lib_asm "ma_init"
gen_bin_asm "main"

#gen_iai_asm "iai_add"

gen_crit_asm "validate"
gen_crit_asm "ma_test_1"
gen_crit_asm "ga_test_1"
gen_crit_asm "ma_test"
gen_crit_asm "ga_test"
