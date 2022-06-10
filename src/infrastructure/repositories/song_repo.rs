use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult },
    sync::{ Client, Collection }
};

use crate::infrastructure::models::song_model::Song;

pub struct SongRepository {
    col: Collection<Song>
}

impl SongRepository {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_DB") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error with env vars"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rusty_api");
        let col: Collection<Song> = db.collection("Song");
        SongRepository { col }
    }

    pub fn create(&self, new_song: Song) -> Result<InsertOneResult, Error> {
        let doc = Song {
            id: None,
            name: new_song.name,
            genre: new_song.genre,
            artist: new_song.artist,
        };

        let song = self
                    .col
                    .insert_one(doc, None)
                    .ok()
                    .expect("Error creating");

        return Ok(song);
    }

}