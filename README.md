# cli-test-executor
Tool for running CLI tests

The purpose of CLI Test Executor (CTE) is to automate testing of CLI tools. It is written in Rust.

CTE is made to support several test dialects, like ctest and shelltestrunner, which are used to test Plain-Text Accounting applications ledger-cli and hledger. The goal is to have a tool in the Rust development toolchain that can read existing tests out-of-the-box.

One of the goals of the project is to support (re-)writing Ledger-cli in Rust by providing a convenient method to run the existing tests with two versions of the application.

That said, it can be used to run any CLI application and to compare the output to the expected text.

This tool is inspired by:

- ctest
- shelltestrunner
- lit

# License

See the [license](LICENSE) file.
