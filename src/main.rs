use tokio::time::error::Error;

mod core;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    Result::Ok(())
}
