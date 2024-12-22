use mercury_land::error::ServerError;

fn main() -> Result<(), ServerError> {
    env_logger::init();

    mercury_land::database::init()?;
    mercury_land::webpage::init()?;
    mercury_land::discord::init()?;

    Ok(())
}
