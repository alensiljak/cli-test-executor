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

use crate::{TestFile, TestFileSections};

/// Parses the test text file.
pub(crate) fn parse_test_file(path: PathBuf) -> anyhow::Result<TestFile> {
    // read line by line
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut current_section = TestFileSections::Input;

    for line_res in reader.lines() {
        // parse
        let line = line_res?;
        //match line
        if line.starts_with(';') {
            todo!("comment");
        } else if line.starts_with("test") {
            todo!("parse test");
            // extract the command.
            // start the new Test section in the output.
            // The input section is complete, start the output section.
            current_section = TestFileSections::Output;
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

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use anyhow::Ok;

    use super::parse_test_file;

    #[test]
    fn test_parse_file() -> anyhow::Result<()> {
        let path = "tests/cmd-accounts.test";
        let pathbuf = PathBuf::from_str(path)?;

        let actual = parse_test_file(pathbuf);

        assert_eq!(1, actual.tests.len());

        Ok(())
    }
}
