use mpd_client::commands::responses::{Empty, Song};
use mpd_client::commands::{Add, ListAllIn, Play};
use mpd_client::{Client, CommandError};
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use pop_launcher_toolkit::launcher::{Indice, PluginResponse, PluginSearchResult};
use pop_launcher_toolkit::plugin_trait::{async_trait, PluginExt};

pub struct MpdPlugin {
    client: Arc<Client>,
    db: Arc<Mutex<Vec<Song>>>,
}

impl MpdPlugin {
    async fn filter_song(&self, term: String) -> Vec<(u32, Song)> {
        let songs = self.db.lock().unwrap();

        songs
            .iter()
            .enumerate()
            .filter(move |(_idx, song)| song.url.contains(&term))
            .map(|(idx, song)| (idx as u32, song.clone()))
            .collect()
    }

    async fn play(&self, idx: u32) -> Result<Empty, CommandError> {
        let song_url = {
            let songs = self.db.lock().unwrap();
            let song = songs.get(idx as usize).unwrap();
            song.url.clone()
        };

        let son_id = self.client.command(Add::uri(song_url)).await;

        match son_id {
            Ok(song_id) => self.client.command(Play::song(song_id)).await,
            Err(err) => Err(err),
        }
    }
}

#[async_trait]
impl PluginExt for MpdPlugin {
    async fn activate(&mut self, id: Indice) {
        self.play(id).await.expect("Failed to play song");
        self.respond_with(PluginResponse::Close).await
    }

    async fn activate_context(&mut self, _id: Indice, _context: Indice) {
        // not needed
    }

    async fn complete(&mut self, _id: Indice) {
        // not needed
    }

    async fn context(&mut self, _id: Indice) {
        // not needed
    }

    fn exit(&mut self) {
        // not needed
    }

    async fn interrupt(&mut self) {
        // not needed
    }

    fn name(&self) -> &str {
        "mpd"
    }

    async fn search(&mut self, query: &str) {
        let query = query.strip_prefix("mpd ").unwrap();
        let songs = self.filter_song(query.to_string()).await;
        let songs = songs.iter().map(|(id, song)| (*id, song.url.clone()));

        for (id, song) in songs {
            let response = PluginResponse::Append(PluginSearchResult {
                id,
                name: song,
                description: "".to_string(),
                keywords: None,
                icon: None,
                exec: None,
                window: None,
            });

            self.respond_with(response).await;
        }

        self.respond_with(PluginResponse::Finished).await;
    }

    async fn quit(&mut self, _id: Indice) {
        self.respond_with(PluginResponse::Close).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = TcpStream::connect("localhost:6600").await?;

    let (client, mut _state_changes) = Client::connect(connection)
        .await
        .expect("Failed to connect to MPD server");

    let songs = client
        .command(ListAllIn::root())
        .await
        .expect("Failed to get MPD database");

    let mut plugin = MpdPlugin {
        client: Arc::new(client),
        db: Arc::new(Mutex::new(songs)),
    };

    plugin.run().await;

    Ok(())
}
