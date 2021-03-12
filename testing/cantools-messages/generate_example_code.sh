#! /usr/bin/env bash
set -ex

FILE=${1:-example.dbc}

cantools generate_c_source ../dbc-examples/$FILE