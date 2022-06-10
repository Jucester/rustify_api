use crate::{infrastructure::models::song_model::Song, infrastructure::repositories::song_repo::SongRepository};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/songs", data ="<new_song>")]
pub fn create_song(
    db: &State<SongRepository>,
    new_song: Json<Song>
) -> Result<Json<InsertOneResult>, Status> {
    let data = Song {
        id: None,
        name: new_song.name.to_owned(),
        genre: new_song.genre.to_owned(),
        artist: new_song.artist.to_owned(),
    };
    let song_details = db.create(data);
    match song_details {
        Ok(song) => Ok(Json(song)),
        Err(_) => Err(Status::InternalServerError),
    }
}