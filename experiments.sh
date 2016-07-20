#!/bin/bash
trap 'kill -- -$$' 2 # SIGINT (ctrl+c)
path=$(pwd)

cache=$path/cache/target/release/enclave_cache_bin
sensor=$path/sensors/sensor-rust/target/release/sensor_rust
subscriber=$path/cache_subscriber/target/release/cache_subscriber

cd "$path/cache" || exit
cargo build --release
cd "$path/cache_subscriber" || exit
cargo build --release
cd "$path/sensors/sensor-rust" || exit
cargo build --release


# build and start the sensors
# format=protobuf
format=json
policy=mac
logging=yes
# logging=debug

mkdir -p logs
# cargo run type=unclutch format=$format policy=mac port=5551 period=20 log=$logging > ../../logs/sensor-unclutch.log &
# cargo run type=invalid-voltage format=$format policy=mac port=5552 period=21 log=$logging > ../../logs/sensor-invalid-voltage.log &
# cargo run type=speed-error format=$format policy=mac port=5553 period=22 log=$logging > ../../logs/sensor-speed-error.log &
# cargo run type=speed-unsafe format=$format policy=mac port=5554 period=23 log=$logging > ../../logs/sensor-speed-unsafe.log &
# cargo run type=clamp15 format=$format policy=mac port=5555 period=24 log=$logging > ../../logs/sensor-clamp15.log &

echo "Starting the cache with format=$format and policy=$policy..."
$cache log=$logging format=$format > $path/logs/cache.log &
cache_pid=$!

# param: size of value
latency_over_number_of_values () {
  size=$1
  mkdir -p $path/logs/lat-over-number
  echo "Starting sensor with value size of $size bytes..."

  echo "Sensor with value size of $size bytes" > $path/logs/lat-over-number/sensor-sized.log
  $sensor log=$logging type=sized format=$format policy=$policy port=5551 period=20 size=$size >> $path/logs/lat-over-number/sensor-sized.log &
  sensor_pid=$!

  # wait until the cache is filled: 30 sec (1000x 25ms = 25s)
  echo "Waiting for the cache filling its buffers."
  sleep 15  # at the beginning, half filled is sufficient

  echo "Starting the latency measurements over number of values for fixed size $size"
  echo "logTimestamp; numberOfRequestedValues; valueSizeInBytes; seconds; nanoseconds;" > $path/logs/lat-over-number/latency-$size-bytes-over-valuenumber.csv
  for i in {1..2} # TODO
  # for i in {1..50}
    do
      $subscriber log=$logging action=latency format=$format valuenr=$i >> $path/logs/lat-over-number/latency-$size-bytes-over-valuenumber.csv
    done
  # for i in {49..50} # TODO
  # for i in {6..50}
    # do
    #   $subscriber log=$logging action=latency format=$format valuenr=$i'0' >> $path/logs/lat-over-number/latency-$size-bytes-over-valuenumber.csv
    # done

  kill $sensor_pid
  wait $sensor_pid 2>/dev/null
  echo "Finished measurements for latency over number of values for fixed size $size."
}

# param: number of values
latency_over_value_size () {
  valuenr=$1
  mkdir -p $path/logs/lat-over-size
  echo "Starting the latency measurements over size in bytes for fixed value number $valuenr."
  echo "logTimestamp; numberOfRequestedValues; valueSizeInBytes; seconds; nanoseconds;" > $path/logs/lat-over-size/latency-$valuenr-values-over-size.csv
  echo "Sensors with increasing value sizes:" >  $path/logs/lat-over-size/sensor-sized.log

  for i in {1..2} # TODO
  # for i in {1..40}    # 40 = (4kb)
    do
      size=$i'00'     # * 100 ... -> from 100 to 4000 bytes
      echo "Starting sensor with value size of $size bytes."
      echo "Sensor with value size of $size bytes" >> $path/logs/lat-over-size/sensor-sized.log
      $sensor log=$logging type=sized format=$format policy=$policy port=5551 period=20 size=$size >> $path/logs/lat-over-size/sensor-sized.log &
      sensor_pid=$!
      sleep 5  # wait until the cache is filled with new values
      echo "size: $size"
      $subscriber log=$logging action=latency format=$format valuenr=$valuenr >> $path/logs/lat-over-size/latency-$valuenr-values-over-size.csv
      kill $sensor_pid
      wait $sensor_pid 2>/dev/null
    done

    echo "Finished the latency measurements over size in bytes for fixed value number $valuenr."
}

#param: size of value
throughput_over_number_of_values () {
  size=$1
  mkdir -p $path/logs/tp-over-number
  echo "Starting the throughput measurements over number of values for fixed size $size"

  echo "Starting sensor with value size of $size bytes..."
  echo "Sensor with value size of $size bytes" > $path/logs/tp-over-number/sensor-sized.log
  $sensor log=$logging type=sized format=$format policy=$policy port=5551 period=20 size=$size >> $path/logs/tp-over-number/sensor-sized.log &
  sensor_pid=$!
  # wait until the cache is filled: 30 sec (1000x 25ms = 25s)
  echo "Waiting for the cache filling its buffers..."
  sleep 15  # at the beginning, half filled is sufficient


  # TODO find out how much threads are necessary
  threads=20
  echo "Starting $threads requesters for generating cache load."
  # TODO period for requester? period=100
  #  TODO IMPORTANT!!! evaluate!

  echo "logTimestamp; numberOfRequestedValues; valueSizeInBytes; requestsPerSecond;" > $path/logs/tp-over-number/throughput-$size-bytes-over-valuenumber.csv
  echo "Requester started for size $size." > $path/logs/tp-over-number/requester.log &
  for i in {1..2}  # TODO evaluate thread number
  # for i in {1..50}
    do
      # terminate the cache in order to ensure it doesn't still respond to old requests
      kill $cache_pid
      wait $cache_pid 2>/dev/null
      $cache log=$logging format=$format >> $path/logs/cache.log &
      cache_pid=$!

      $subscriber log=$logging action=request format=$format valuenr=$i threads=$threads >> $path/logs/tp-over-number/requester.log &
      requester_pid=$!
      $subscriber log=$logging action=throughput format=$format valuenr=$i >> $path/logs/tp-over-number/throughput-$size-bytes-over-valuenumber.csv
      kill $requester_pid
      wait $requester_pid 2>/dev/null

    done

  for i in {49..50} # TODO evaluate thread number
  # for i in {6..50}
    do
      # terminate the cache in order to ensure it doesn't still respond to old requests
      kill $cache_pid
      wait $cache_pid 2>/dev/null
      $cache log=$logging format=$format >> $path/logs/cache.log &
      cache_pid=$!

      $subscriber log=$logging action=request format=$format valuenr=$i'0' threads=$threads >> $path/logs/tp-over-number/requester.log &
      requester_pid=$!
      $subscriber log=$logging action=throughput format=$format valuenr=$i'0' >> $path/logs/tp-over-number/throughput-$size-bytes-over-valuenumber.csv
      kill $requester_pid
      wait $requester_pid 2>/dev/null
    done

    kill $sensor_pid
    wait $sensor_pid 2>/dev/null
  echo "Finished measurements for throughput over number of values for fixed size $size."
}

# param: number of values
throughput_over_value_size () {
  valuenr=$1
  mkdir -p $path/logs/tp-over-size

  echo "Starting the throughput measurements over size in bytes for fixed value number $valuenr"
  echo "logTimestamp; numberOfRequestedValues; valueSizeInBytes; requestsPerSecond;" > $path/logs/tp-over-size/throughput-$valuenr-values-over-size.csv
  echo "Sensors with increasing value sizes:" >  $path/logs/tp-over-size/sensor-sized.log

  # TODO find out how much threads are necessary
  threads=20
  echo "Starting $threads requesters for generating cache load."
  echo "Requester for size $size." > $path/logs/tp-over-size/requester.log &
  # TODO period for requester? period=100
  #  TODO IMPORTANT!!! evaluate!

  for i in {1..2}  # TODO evaluate thread number
  # for i in {1..40}
    do
      size=$i'00'

      # terminate the cache in order to ensure it doesn't still respond to old requests
      kill $cache_pid
      wait $cache_pid 2>/dev/null # suppressing the output of 'kill'
      $cache log=$logging format=$format >> $path/logs/cache.log &
      cache_pid=$!
      echo "Starting sensor with value size of $size bytes..."
      $sensor log=$logging type=sized format=$format policy=$policy port=5551 period=20 size=$size >> $path/logs/tp-over-size/sensor-sized.log &
      sensor_pid=$!
      $subscriber log=$logging action=request format=$format valuenr=$valuenr threads=$threads >> $path/logs/tp-over-size/requester.log &
      subscriber_pid=$!
      # wait until the cache is filled: 30 sec (1000x 25ms = 25s)
      sleep 5  # at the beginning, half filled is sufficient; Cache has an expiration of 4sec
      $subscriber log=$logging action=throughput format=$format valuenr=$valuenr >> $path/logs/tp-over-size/throughput-$valuenr-values-over-size.csv

      kill $sensor_pid
      wait $sensor_pid 2>/dev/null
      kill $subscriber_pid
      wait $subscriber_pid 2>/dev/null
    done

  echo "Finished measurements for throughput over size of values for value number $valuenr."
}


# # 1st:  latency as a function of the number of values (with value size =150|1000
# #       (facebook memcache mean:135; median:954))
latency_over_number_of_values 150
latency_over_number_of_values 1000
#
# # 2nd: latency as a function of the value size (with fixed value number)
latency_over_value_size 10
latency_over_value_size 20
latency_over_value_size 100


#  3rd: throughput as a function of the number of values
throughput_over_number_of_values 150
throughput_over_number_of_values 1000

# 4th: TODO throughput as a function of the value size (with fixed value number)
throughput_over_value_size 10
throughput_over_value_size 20
throughput_over_value_size 100


# printf "Press Return for terminating the sensors:"
# read

# kill the script and all its (child, grandchild, ...) processes
kill -- -$$

exit 0
