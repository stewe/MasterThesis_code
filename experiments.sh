#!/bin/bash

path=$(pwd)

# build and start the sensors
format=protobuf
# format=json
policy=mac
logging=yes
# logging=debug

# echo "Starting the sensors..."

mkdir -p logs
# cargo run type=unclutch format=$format policy=mac port=5551 period=20 log=$logging > ../../logs/sensor-unclutch.log &
# cargo run type=invalid-voltage format=$format policy=mac port=5552 period=21 log=$logging > ../../logs/sensor-invalid-voltage.log &
# cargo run type=speed-error format=$format policy=mac port=5553 period=22 log=$logging > ../../logs/sensor-speed-error.log &
# cargo run type=speed-unsafe format=$format policy=mac port=5554 period=23 log=$logging > ../../logs/sensor-speed-unsafe.log &
# cargo run type=clamp15 format=$format policy=mac port=5555 period=24 log=$logging > ../../logs/sensor-clamp15.log &

echo "Starting the cache with format=$format and policy=$policy..."
cd "$path/cache/" || exit
cargo run --quiet --release log=$logging format=$format > ../logs/cache.log &


# param: size of value
latency_over_number_of_values () {
  size=$1
  echo "Starting sensor with value size of $size bytes..."

  cd "$path/sensors/sensor-rust/" || exit
  echo "Sensor with value size of $size bytes" > ../../logs/sensor-sized.log
  cargo run --quiet --release log=$logging type=sized format=$format policy=$policy port=5551 period=20 size=$size >> ../../logs/sensor-sized.log &

  # wait until the cache is filled: 30 sec (1000x 25ms = 25s)
  echo "Waiting for the cache filling its buffers."
  sleep 15  # at the beginning, half filled is sufficient

  cd "$path/cache_subscriber/" || exit
  # cargo build --release
  echo "Starting the latency measurements over number of values for fixed size $size"
  echo "log-timestamp; number of requested values; value size in bytes; seconds; nanoseconds;" > ../logs/latency-$size-bytes-over-valuenumber.csv
  # for i in {1..2}
  for i in {1..50}
    do
      cargo run --quiet --release log=$logging action=latency format=$format valuenr=$i >> ../logs/latency-$size-bytes-over-valuenumber.csv
    done
  for i in {6..50}
    do
      cargo run --quiet --release log=$logging action=latency format=$format valuenr=$i'0' >> ../logs/latency-$size-bytes-over-valuenumber.csv
    done

  kill $! # kills background job cargo
  # pkill cargo run --release type=sized
  pkill sensor_rust # kills sensor_rust, spawned by cargo
  echo "Finished measurements for latency over number of values for fixed size $size."
}

# param: number of values
latency_over_value_size () {
  valuenr=$1
  echo "Starting sensor with value number of $valuenr"

  echo "Starting the latency measurements over size in bytes for fixed value number $valuenr"
  echo "log-timestamp; number of requested values; value size in bytes; seconds; nanoseconds;" > ../logs/latency-$valuenr-values-over-size.csv
  # for i in {1..2}
  for i in {1..40}    # 40 = (4kb)
    do
      size=$i'00'     # * 100 ... -> from 100 to 4000 bytes
      cd "$path/sensors/sensor-rust/" || exit
      echo "Sensor with value size of $size bytes" > ../../logs/sensor-sized.log
      cargo run --quiet --release log=$logging type=sized format=$format policy=$policy port=5551 period=20 size=$size >> ../../logs/sensor-sized.log &
      sleep 5  # wait until the cache is filled with new values
      echo "size: $size"
      cd "$path/cache_subscriber/" || exit
      cargo run --quiet --release log=$logging action=latency format=$format valuenr=$valuenr >> ../logs/latency-$valuenr-values-over-size.csv
      kill $!
      pkill sensor_rust
    done
}

#param: size of value
throughput_over_number_of_values () {
  size=$1
  echo "Starting sensor with value size of $size bytes..."

  cd "$path/sensors/sensor-rust/" || exit
  echo "Sensor with value size of $size bytes" > ../../logs/sensor-sized.log
  cargo run --quiet --release log=$logging type=sized format=$format policy=$policy port=5551 period=20 size=$size >> ../../logs/sensor-sized.log &
  sensor_pid=$!

  # wait until the cache is filled: 30 sec (1000x 25ms = 25s)
  echo "Waiting for the cache filling its buffers..."
  sleep 15  # at the beginning, half filled is sufficient

  cd "$path/cache_subscriber/" || exit
  # cargo build --release

  # TODO find out how much threads are necessary

  threads=20
  echo "Starting $threads requesters for generating cache load."
  # TODO period for requester? period=100
  #  TODO IMPORTANT!!! evaluate!

  echo "Starting the throughput measurements over number of values for fixed size $size"
  echo "log-timestamp; number of requested values; value size in bytes; requests per second" > ../logs/throughput-$size-bytes-over-valuenumber.csv
  echo "Requester started for size $size." > ../logs/requester.log &
  # for i in {1..2}  # TODO evaluate thread number
  for i in {1..50}
    do
      cargo run --quiet --release log=$logging action=request format=$format valuenr=$i threads=$threads >> ../logs/requester.log &
      requester_pid=$!
      cargo run --quiet --release log=$logging action=throughput format=$format valuenr=$i >> ../logs/throughput-$size-bytes-over-valuenumber.csv
      kill $requester_pid # kills background job cargo
      pkill cache_subscriber
    done

  # for i in {49..50} # TODO evaluate thread number
  for i in {6..50}
    do
      cargo run --quiet --release log=$logging action=request format=$format valuenr=$i'0' threads=$threads >> ../logs/requester.log &
      requester_pid=$!
      cargo run --quiet --release log=$logging action=throughput format=$format valuenr=$i'0' >> ../logs/throughput-$size-bytes-over-valuenumber.csv
      kill $requester_pid # kills background job cargo
      pkill cache_subscriber
    done

  kill $sensor_pid # kills background job cargo
  pkill sensor_rust # kills sensor_rust, spawned by cargo
  echo "Finished measurements for latency over number of values for fixed size $size."
}


# # 1st:  latency as a function of the number of values (with value size =150|1000
# #       (facebook memcache mean:135; median:954))
# latency_over_number_of_values 150
# latency_over_number_of_values 1000
#
# # 2nd: latency as a function of the value size (with fixed value number)
# latency_over_value_size 10
# latency_over_value_size 20
# latency_over_value_size 100


#  3rd: throughput as a function of the number of values
throughput_over_number_of_values 150
# throughput_over_number_of_values 1000

# 4th: TODO throughput as a function of the value size (with fixed value number)
# throughput_over_value_size 10
# throughput_over_value_size 20
# throughput_over_value_size 100


# printf "Press Return for terminating the sensors:"
# read

# kill the script and all its (child, grandchild, ...) processes
kill -- -$$

exit 0
