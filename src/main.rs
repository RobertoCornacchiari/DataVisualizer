use std::sync::RwLock;

use rocket::{serde::{Deserialize, json::Json}, State};

#[macro_use] extern crate rocket;

struct Values(RwLock<String>);

#[get("/")]
fn index(values: &State<Values>) -> String {
    let a = values.0.read().unwrap().to_owned();
    a.to_owned()
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Test {
    stringa: String,
}

#[post("/", format = "application/json", data = "<stringa>")]
fn prova(stringa: Json<Test>,values: &State<Values>) {
    println!("{:?}", stringa.stringa);
    *values.0.write().unwrap() = stringa.stringa.to_owned();
}

#[launch]
fn rocket() -> _ {
    let global_state:Values = Values(RwLock::new("Ciaone".to_string()));
    rocket::build().manage(global_state).mount("/", routes![index, prova])
}