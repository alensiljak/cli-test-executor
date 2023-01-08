/*!
Handling the ledger test files.
Format:
```
; comment line
<input>
test <ledger command>
<output>
end test
[
test <cli command>
<output>
]
```

Some rules:
- the ledger command does not include "ledger" (the app name)
 */

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{TestDef, TestFile, TestFileSections};

/// Parses the test text file.
pub(crate) fn parse_test_file(path: PathBuf) -> anyhow::Result<TestFile> {
    let mut result = TestFile::new();
    // read line by line
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut current_section = TestFileSections::Input;
    let mut current_test_section = 0;

    for line_res in reader.lines() {
        // parse
        let line = line_res?;
        log::debug!("line: {:?}", line);

        //match line
        if line.starts_with(';') {
            // comment, ignore
            log::debug!("ignoring");
            continue;
        } else if line.starts_with("test") {
            // test line
            result.tests.push(TestDef::default());

            // extract the command.
            result.tests[current_test_section].command = extract_command(&line).to_owned();

            // The input section is complete, start the output section.
            current_section = TestFileSections::Output;

            log::debug!("got command {:?}", &line);
            continue;
        } else if line.eq("end test") {
            // end of the test section
            // complete the current section.
            // start the new Test section in the output.
            current_test_section += 1;
            current_section = TestFileSections::Unknown;
        } else {
            // read into the current section
            match current_section {
                TestFileSections::Input => {
                    // Add to input section
                    log::debug!("adding to input.");

                    result.input.push(line);
                }
                TestFileSections::Output => {
                    // Add to output section
                    log::debug!("adding to output.");

                    result.tests[current_test_section].ouput.push(line);
                }
                TestFileSections::Unknown => {
                    // Ignore anything until the next test section.
                    log::debug!("ignoring");
                }
                _ => {
                    // ignore
                    log::debug!("skipping");
                }
            }
            continue;
        }
    }

    Ok(result)
}

/// Run the tests in the parsed structure.
pub(crate) fn run_tests(test_file: TestFile) {
    for (index, test) in test_file.tests.iter().enumerate() {
        log::debug!("Test {} results:", index);

        let expected = test
            .ouput
            .iter()
            .map(|line| line.as_str())
            .collect::<Vec<&str>>();

        let output = run_command(&test.command);
        let actual: Vec<&str> = output.iter().map(|line| line.as_str()).collect();

        // let diff = difflib::unified_diff(&expected, &actual, "Expected", "Actual", "", "", 3);
        let diff = difflib::context_diff(&expected, &actual, "Expected", "Actual", "", "", 3);

        if !diff.is_empty() {
            diff.iter().for_each(|line| println!("{}", line));

            log::debug!("actual: {:?}", &actual);
            log::debug!("expected: {:?}", &expected);
        }
    }
}

/// Extract ledger command from the `test` line in the file.
fn extract_command(line: &str) -> &str {
    // get everything after the `test `
    &line[5..].trim()
}

/// Execute the command and get the output.
fn run_command(test_command: &str) -> Vec<String> {
    // todo: read stdin, or
    // read the test file as input.
    
    let mut command = "ledger ".to_string();
    command.push_str(&test_command);

    log::debug!("running {:?}", &command);

    // run the command
    let output = cli_runner::run(&command);

    let error = cli_runner::get_stderr(&output);
    log::debug!("Error: {:?}", error);
    assert!(output.stderr.is_empty());

    // collect the output
    let actual = cli_runner::get_stdout(&output)
        .lines()
        // .collect::<Vec<&str>>();
        .map(|line| line.to_owned())
        .collect();

    actual
}

// Tests

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::TestFile;

    use super::{extract_command, parse_test_file};

    #[test]
    fn test_command_extraction() {
        let line = "test accounts";

        let actual = extract_command(line);

        let expected = "accounts";

        assert_eq!(expected, actual);
    }

    #[test_log::test]
    fn test_parse_file() -> anyhow::Result<()> {
        let path = "tests/cmd-accounts.test";
        let pathbuf = PathBuf::from_str(path)?;

        let actual: TestFile;
        let result = parse_test_file(pathbuf);
        match result {
            Ok(value) => actual = value,
            Err(e) => panic!("Error: {:?}", e),
        }

        log::debug!("actual: {:?}", actual);

        assert_eq!(4, actual.tests.len());
        assert_eq!("accounts b", actual.tests[2].command);
        assert_eq!("Assets:Testing123ÕßDone", actual.tests[3].ouput[0]);

        Ok(())
    }

    #[test_log::test]
    fn test_running_cli() {
        let command = "ledger accounts";

        // run the command
        let output = cli_runner::run(&command);

        let error = cli_runner::get_stderr(&output);
        log::debug!("Error: {:?}", error);
        assert!(output.stderr.is_empty());

        // collect the output
        let actual = cli_runner::get_stdout(&output)
            .lines()
            .collect::<Vec<&str>>();

        assert!(!actual.is_empty());
    }
}
