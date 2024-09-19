# VSS mapping between 2 versions

Signal is maybe replaced by another signal in a higher VSS version. This program tries to find the top most related signals in other VSS version.

## 1. How to run

### 1.1. Help
* `cargo run -- -h`: introduction and options to run

### 1.2. Default options
* `cargo run`: run app with default options
    * `--signal`: `signals/signals_v3.0.json`
    * `--vss`: `vss-core/vss_release_4.0.json`
    * `--num`: `5`

### 1.3. Full options
* `cargo run -- --signal <path-2-signal> --vss <path-2-vss> --num <num-result>`
* Or: `cargo run -- -s <path-2-signal> -v <path-2-vss> -n <num-result>`
    > All options are optional

## 2. Implementation
* Matching algorithm: count number of common characters on the left and right between 2 signals
* Example:
    * `s1 = "Vehicle.Body.Hood.IsOpen"`
    * `s2 = "Vehicle.Body.Trunk.Front.IsOpen"`
    * Common left pattern: `"Vehicle.Body.` --> 13
    * Common right pattern: `".IsOpen"` --> 7
    * Matching score: 13 + 7 = 20