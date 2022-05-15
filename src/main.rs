use std::error::Error;
use std::process::Command;
use mpd_client::{Client, commands, Filter, Subsystem, Tag};
use mpd_client::commands::responses::Status;
use mpd_client::commands::{Find, List, ListAllIn, SeekMode};
use mpd_client::filter::Operator;
use mpd_client::raw::RawCommand;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tracing_subscriber::{FmtSubscriber};
use crate::commands::responses::Song;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>>  {
    let connection = TcpStream::connect("localhost:6600").await?;
    let (client, mut _state_changes) = Client::connect(connection).await?;
    let list  = ListAllIn::root();
    let songs = client.command(list).await?;
    let songs: Vec<&Song> = songs.iter()
        .filter(|song| song.url.contains("Maria"))
        .collect();

    println!("{:#?}", songs);

    Ok(())
}
