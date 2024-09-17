# VSS mapping between 2 versions

Signal is maybe replaced by another signal in a higher VSS version. This program tries to find the top most related signals in other VSS version.

## How to run
* Modify `src/main.rs`:
    * `signals_v4` (line 98): replace `JSON_V4` by path to VSS file that contains new signals
    * signals_v3 (line 99): contains signals that you have already had
    * `num_result`: number of top signals will be printed
* Run: `cargo run`