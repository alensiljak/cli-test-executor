use std::path::PathBuf;

/**
   The default behaviour is to read the existing directory and all subdirectories for known
   test definitions.

   The test files are text files with .test file extension.


*/
fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::debug!("starting");

    // collect all the test files into a harness
    let paths = glob::glob("*/*.test")?;
    //log::debug!("found {} test files.", &paths.count());
    for entry in paths {
        match entry {
            Ok(path) => run_test(path),
            Err(e) => log::error!("error: {}", e),
        }
    }

    log::debug!("complete");
    Ok(())
}

fn run_test(path: PathBuf) {
    log::debug!("found test: {}", path.as_path().to_str().unwrap());
    log::debug!("running test {}", path.as_os_str().to_str().unwrap());
}
