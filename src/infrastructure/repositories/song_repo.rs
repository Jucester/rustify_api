use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult },
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

      pub fn getAllSongs(&self) -> Result<Vec<Song>, Error> {
        // let objId = ObjectId::parse_str(id).unwrap();
        // let filter = doc!{"_id": objId};
        let cursor = self
            .col
            .find(None, None) 
            .ok()
            .expect("Error retrieving song");
        let songs = cursor.map(|doc| doc.unwrap()).collect();
        return Ok(songs)
    }

    pub fn getSong(&self, id: &String) -> Result<Song, Error> {
        let objId = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": objId};
        let details = self
            .col
            .find_one(filter, None) 
            .ok()
            .expect("Error retrieving song");
        return Ok(details.unwrap())
    }

    pub fn updateSong(&self, id: &String, data: Song) -> Result<UpdateResult, Error> {
        let objId = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": objId};

        let newData = doc!{
            "$set": {
                "name": data.name,
                "genre": data.genre,
                "artist": data.artist,
            }
        };

        
        let details = self
            .col
            .update_one(filter, newData, None) 
            .ok()
            .expect("Error updating song");
        return Ok(details)
    }

    pub fn deleteSong(&self, id: &String) -> Result<DeleteResult, Error> {
        let objId = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": objId};
        let details = self
            .col
            .delete_one(filter, None) 
            .ok()
            .expect("Error deleting song");
        return Ok(details)
    }

}