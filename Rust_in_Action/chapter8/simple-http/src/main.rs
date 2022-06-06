use std::error::Error;

use reqwest;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:8000/index.php?name=Kaines";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    print!("{}", content);

    Ok(())
}