use crate::{infrastructure::models::song_model::Song, infrastructure::repositories::song_repo::SongRepository};
use mongodb::{results::InsertOneResult, bson::oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};


#[get("/songs")]
pub fn get_all_songs(
    db: &State<SongRepository>,
) -> Result<Json<Vec<Song>>, Status> {
    let details = db.getAllSongs();
    match details {
        Ok(song) => Ok(Json(song)),
        Err(_) => Err(Status::InternalServerError),
    }
}

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

#[get("/songs/<id>")]
pub fn get_song(
    db: &State<SongRepository>,
    id: String
) -> Result<Json<Song>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let details = db.getSong(&id);
    match details {
        Ok(song) => Ok(Json(song)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/songs/<id>", data = "<data>")]
pub fn update_song(
    db: &State<SongRepository>,
    id: String,
    data: Json<Song>,
) -> Result<Json<Song>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let newData = Song {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: data.name.to_owned(),
        genre: data.genre.to_owned(),
        artist: data.artist.to_owned(),
    };

    let updated = db.updateSong(&id, newData);

    match updated {
        Ok(song) => {
            if song.matched_count == 1 {
                let updatedInfo = db.getSong(&id);
                return match updatedInfo {
                    Ok(song) => Ok(Json(song)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/songs/<id>")]
pub fn delete_song(
    db: &State<SongRepository>,
    id: String
) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let details = db.deleteSong(&id);
    match details {
        Ok(song) => {
            if song.deleted_count == 1 {
               return Ok(Json("Song deleted successfully"));
            } else {
                return Err(Status::NotFound);
            }
        }, 
        Err(_) => Err(Status::InternalServerError),
    }
}