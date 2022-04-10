#!/bin/bash


set -e

echo find > input.txt

cargo run -- find replace input.txt output.txt

echo testing

test $(cat output.txt) = "replace"

echo ok

