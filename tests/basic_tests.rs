/*
Runs the CTE on the sample test(s) in the tests directory.
 */

use assert_cmd::{Command, prelude::OutputAssertExt};

/// Runs on the cmd-accounts.test file.
#[test]
fn basic_test() {
    // env!("CARGO_PKG_NAME")
    let output = Command::cargo_bin("cte")
        .unwrap()
        // .arg("tests/cmd-accounts.test")
        .output()
        .unwrap();

    println!("output: {:?}", output);

    let assert = output.assert();
    assert.success();
}
