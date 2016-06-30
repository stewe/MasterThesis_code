#!/bin/bash

# build and start the sensors
format=protobuf
logging=y

echo "starting the sensors..."

mkdir -p logs
cd sensors/sensor-rust/
cargo build
cargo run type=unclutch format=$format policy=mac port=5551 period=20 log=$logging > ../../logs/sensor-unclutch.log &
cargo run type=voltage format=$format policy=mac port=5552 period=21 log=$logging > ../../logs/sensor-voltage.log &
cargo run type=speed-error format=$format policy=mac port=5553 period=22 log=$logging > ../../logs/sensor-speed-error.log &
cargo run type=speed-unsafe format=$format policy=mac port=5554 period=23 log=$logging > ../../logs/sensor-speed-unsafe.log &
cargo run type=clamp15 format=$format policy=mac port=5555 period=24 log=$logging > ../../logs/sensor-clamp15.log &

printf "Press Return for terminating the sensors:"
read

kill -- -$$ # kills the scrippt and all its (child, grandchild, ...) processes

exit 0
