#!/bin/bash

path=$(pwd)

# build and start the sensors
format=protobuf
# format=json
policy=mac
logging=yes

# echo "Starting the sensors..."

mkdir -p logs
# cargo run type=unclutch format=$format policy=mac port=5551 period=20 log=$logging > ../../logs/sensor-unclutch.log &
# cargo run type=invalid-voltage format=$format policy=mac port=5552 period=21 log=$logging > ../../logs/sensor-invalid-voltage.log &
# cargo run type=speed-error format=$format policy=mac port=5553 period=22 log=$logging > ../../logs/sensor-speed-error.log &
# cargo run type=speed-unsafe format=$format policy=mac port=5554 period=23 log=$logging > ../../logs/sensor-speed-unsafe.log &
# cargo run type=clamp15 format=$format policy=mac port=5555 period=24 log=$logging > ../../logs/sensor-clamp15.log &

echo "Starting the cache with format=$format and policy=$policy..."
cd "$path/cache/" || exit
cargo run --quiet --release log=yes format=$format > ../logs/cache.log &


# param: size of value
latency_over_number_of_values () {
  size=$1
  echo "Starting sensor with value size of $size bytes..."

  cd "$path/sensors/sensor-rust/" || exit
  echo "Sensor with value size of $size bytes" > ../../logs/sensor-sized.log
  cargo run --quiet --release type=sized format=$format policy=$policy port=5551 period=20 size=$size log=$logging >> ../../logs/sensor-sized.log &

  # wait until the cache is filled: 30 sec (1000x 25ms = 25s)
  echo "Waiting for the cache filling its buffers."
  sleep 15  # at the beginning, half filled is sufficient

  cd "$path/cache_subscriber/" || exit
  # # cargo build --release
  echo "Starting the latency measurements over number of values for fixed size $size"
  echo "log-timestamp; number of requested values; value size in bytes; seconds; nanoseconds;" > ../logs/latency-$size-bytes-over-valuenumber.csv
  for i in {1..5}   # TODO!!! 500
    do
      cargo run --quiet --release action=latency format=$format valuenr=$i >> ../logs/latency-$size-bytes-over-valuenumber.csv
    done

  kill $! # kills background job cargo
  # pkill cargo run --release type=sized
  pkill sensor_rust # kills sensor_rust, spawned by cargo
  echo "Finished measurements."
}

latency_over_value_size () {
  valuenr=$1
  echo "Starting sensor with value number of $valuenr"

  echo "Starting the latency measurements over size in bytes for fixed value number $valuenr"
  echo "log-timestamp; number of requested values; value size in bytes; seconds; nanoseconds;" > ../logs/latency-$valuenr-values-over-size.csv
  for i in {1..5}    # TODO go up to 40 (4kb)
  do
    size=$i'00'     # * 100 ... -> from 100 to 4000 bytes
    cd "$path/sensors/sensor-rust/" || exit
    echo "Sensor with value size of $size bytes" > ../../logs/sensor-sized.log
    cargo run --quiet --release type=sized format=$format policy=$policy port=5551 period=20 size=$size log=$logging >> ../../logs/sensor-sized.log &
    sleep 5  # wait until the cache is filled with new values
    echo "size: $size"
    cd "$path/cache_subscriber/" || exit
    cargo run --quiet --release action=latency format=$format valuenr=$valuenr >> ../logs/latency-$valuenr-values-over-size.csv
    kill $!
    pkill sensor_rust
  done
}

# 1st: latency as a function of the number of values (with value size =1000 (facebook memcache median:954))
latency_over_number_of_values 1000

# 2nd: latency as a function of the number of values (with value size =150 (facebook memcache mean:135))
latency_over_number_of_values 150

# 3rd: latency as a function of the value size (with fixed value number)
latency_over_value_size 10
latency_over_value_size 20
latency_over_value_size 100


# printf "Press Return for terminating the sensors:"
# read

# kill the script and all its (child, grandchild, ...) processes
kill -- -$$

exit 0
