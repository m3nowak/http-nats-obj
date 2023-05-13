mod upload;
mod cli;
mod server;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = cli::mk_cmd();
    let matches = cmd.get_matches();
    cli::handle(matches).await;
    Ok(())
}
