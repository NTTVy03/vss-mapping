# VSS mapping between 2 versions

Signal is maybe replaced by another signal in a higher VSS version. This program tries to find the top most related signals in other VSS version.

## 1. How to run

### 1.1. Help
* `cargo run -- -h`: introduction and options to run

### 1.2. Default options
* `cargo run`: run app with default options
    * `--vss`: `vss-core/vss_release_4.0.json`
    * `--num`: `5`

### 1.3. Full options
* `cargo run -- --vss <path-2-vss> --num <num-result>`
* Or: `cargo run -- -v <path-2-vss> -n <num-result>`
    > All options are optional

## 2. Interact with CLI
This CLI provides 4 types of command:
* `vss <path-2-VSS-file>`: set vss-file
* `num <usize value>`: set number of results/signal
* `run <path-2-signals-file>`: find top related signals in VSS file of each signal in signals file
* `exit`: quit cli

## 3. Implementation
* Matching algorithm: count number of common characters on the left and right between 2 signals
* Example:
    * `s1 = "Vehicle.Body.Hood.IsOpen"`
    * `s2 = "Vehicle.Body.Trunk.Front.IsOpen"`
    * Common left pattern: `"Vehicle.Body.` --> 13
    * Common right pattern: `".IsOpen"` --> 7
    * Matching score: 13 + 7 = 20