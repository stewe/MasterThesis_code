# A sensor dummy: periodically publishes configurable data

## Arguments:
* type=unclutch | invalid-voltage | speed-error | speed-unsafe | clamp15 | sized
* port=PORTNR
* policy=plain | mac | cipher (optional, default: plain)
* format=json | protobuf (optional, default: Protobuf)
* period=MILLISECONDS (optional, default: 100)
* log=y | n (optional, default: n)
* size=SIZE_IN_BYTES (required for sized)
