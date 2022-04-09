#!/bin/bash

set -e

cargo build --release
time ./target/release/mandelbrot mandelbrot.png 4000x3000 -1.20,0.35 -1.0,0.20
xdg-open mandelbrot.png
