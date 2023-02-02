use rocket::fs::{FileServer, relative};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Values(String);

#[get("/index")]
async fn index(queue: &State<Sender<Values>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[derive(Deserialize)]
struct Test {
    stringa: String,
}

#[post("/", format = "application/json", data = "<stringa>")]
fn prova(stringa: Json<Test>, queue: &State<Sender<Values>>) {
    println!("{:?}", stringa.stringa);
    let _res = queue.send(Values(stringa.stringa.to_owned()));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Values>(16536).0)
        .mount("/", routes![index, prova])
        .mount("/", FileServer::from(relative!("frontEnd/build")))
}
