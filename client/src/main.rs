use anyhow::Result;

pub mod terminal;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO:
    // 1. Look for the server socket
    //  a. If it doesn't exist, spawn a new server process
    // 2. Connect to the server
    // 3. Start sending and receiving messages
    Ok(())
}
