mod application;
mod domain;
mod infrastructure;

#[macro_use]
extern crate rocket;

use application::controllers::song_controller::create_song;
use infrastructure::repositories::song_repo::SongRepository;

#[launch]
fn rocket() -> _ {
    let db = SongRepository::init();
    rocket::build().manage(db).mount("/", routes![create_song])
}
