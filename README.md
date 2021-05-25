# cosmwasm_experiment

## Compiling the contract

The contract has been compiled by running `RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown` from inside the `contract` directory.

```shell
rustc --version                                                                 
rustc 1.52.1 (9bc8c42bb 2021-05-09)
```

## Used validator

The validator used for tests been compiled as per instructions in <https://nymtech.net/docs/run-nym-nodes/validators/>. It was using version `v0.14.1` of wasmd.
The test has been attempted by compiling using both Go 1.15 and 1.16 so both `MADV_FREE` and `MADV_DONTNEED` were checked.

In another attempt [pprof](https://golang.org/pkg/net/http/pprof/) has been put into the binary. It did not detect any memory leaks indicating the problem most likely lies outside the Go's VM.

## Test results

Before starting to query the validator, the `nymd` process was using 95 MB of resident memory.

After running the `querier` (`cargo run --release`) for 5min (5200 loop iterations) the used memory increased to 1300 MB.

After 10 min (10500 loop iterations), it was at 2400 MB

For the next 20min the contract was not queried at all, but the memory was never returned to the OS.