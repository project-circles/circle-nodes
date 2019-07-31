//! This crate encloses the top-level circle node application.
#![deny(warnings)]

use failure::Error;
use websocket::ClientBuilder;

// String constant that holds coordinator address URL.
static COORDINATOR_URL: &str = "ws://0.0.0.0:9090";

// Main thread running the circle node.
fn main() -> Result<(), Error> {
    let mut client = ClientBuilder::new(COORDINATOR_URL)?.connect_insecure()?;
    println!("{:?}", client.recv_message()?);
    Ok(())
}
