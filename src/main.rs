/**
    Runs the tests.
*/
fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::debug!("starting");

    cli_test_executor::run()?;

    log::debug!("complete");
    Ok(())
}
