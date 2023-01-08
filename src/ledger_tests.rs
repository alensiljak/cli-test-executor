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

use crate::{TestFile, TestFileSections, TestDef};

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
        //match line
        if line.starts_with(';') {
            // comment, ignore
            continue;
        } else if line.starts_with("test") {
            // parse test line
            // start the new Test section in the output.
            current_test_section += 1;
            result.tests.push(TestDef::default());
            
            // extract the command.
            result.tests[current_test_section].command = extract_command(&line).to_owned();
            
            // The input section is complete, start the output section.
            current_section = TestFileSections::Output;

            continue;
        } else if line.eq("end test") {
            todo!("end of the test section");
            // complete the current section.
        } else {
            // todo!("read into the current section");
            match current_section {
                TestFileSections::Input => {
                    todo!("Add to input section");
                }
                TestFileSections::Output => {
                    todo!("Add to output section")
                }
                _ => {
                    // ignore
                }
            }
        }
    }

    todo!("complete");
}

/// Extract ledger command from the `test` line in the file.
fn extract_command(line: &str) -> &str {
    // get everything after the `test `
    &line[5..].trim()
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use anyhow::Ok;

    use super::{parse_test_file, extract_command};

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

        let actual = parse_test_file(pathbuf)?;

        log::debug!("actual: {:?}", actual);
        
        assert_eq!(1, actual.tests.len());

        Ok(())
    }
}
