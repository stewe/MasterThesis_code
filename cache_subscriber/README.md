# Helper for the experiments
* generate load with *action=request*
* measure throughput with *action=throughput*
* measure latency with *action=latency*

## Arguments:
* action=latency | throughput | request
* threads=NR_OF_REQUESTER (optional, default: 1)
* period=REQUEST_PERIOD_IN_MS (optional, default: NONE)
* format=json | protobuf (optional, default: Protobuf)
