/*!
CLI Test Executor (CTE) is a tool for running the CLI tests.

The default behaviour is to read the existing directory and all subdirectories for known
test definitions.

The test files are text files with .test file extension.
    
 */

mod ledger_tests;
mod shelltestrunner;

use std::path::PathBuf;

/// Main entry point.
pub fn run() -> anyhow::Result<()> {
    // collect all the test files into a harness
    let paths = glob::glob("**/*.test")?;

    // Run each test file.
    for entry in paths {
        match entry {
            Ok(path) => run_test_file(path),
            Err(e) => log::error!("error: {}", e),
        }
    }

    Ok(())
}

/// CTest test file structure:
/// <input>
/// test <command>
/// <output>
/// A test file can have multiple tests defined.
#[derive(Debug, Default)]
struct TestFile {
    pub input: String,
    pub tests: Vec<TestDef>
}

impl TestFile {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Each test consists of a command and an expected output.
#[derive(Debug, Default)]
struct TestDef {
    pub command: String,
    pub ouput: String
}

#[allow(unused)]
enum TestFileSections {
    Unknown,
    Comment,
    Input,
    Command,
    Output
}

/// Runs individual test files
fn run_test_file(path: PathBuf) {
    log::debug!("found test: {}", path.as_path().to_str().unwrap());
    log::debug!("running test {}", path.as_os_str().to_str().unwrap());

    // todo: currently recognising only ledger test files. Add other types.

    // load and parse the test file.
    match ledger_tests::parse_test_file(path) {
        Ok(_) => todo!(),
        Err(e) => log::error!("error parsing file: {:?}", e),
    }
}
