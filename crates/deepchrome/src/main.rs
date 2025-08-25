use deepchrome::launch_deepseek;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    launch_deepseek()?;
    Ok(())
}
