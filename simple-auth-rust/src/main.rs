#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;
use std::sync::Mutex;
use rocket::State;
use rocket_contrib::json::Json;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use rocket::http::Status;
use rocket::response::status;


#[derive(Debug, PartialEq, Eq, Hash)]
struct Account {
    username: String,
    password: String,
}

type Accounts = Mutex<HashMap<Account, ()>>;

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    account: String,
    password: String,
}

#[derive(Serialize)]
struct Response {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    message: Option<String>,
}

#[post("/register", format = "json", data = "<register_request>")]
fn register(register_request: Json<RegisterRequest>, accounts: State<'_, Accounts>) -> Json<Response> {
    let account = Account {
        username: register_request.0.account,
        password: register_request.0.password,
    };
    
    let mut accounts = accounts.lock().expect("lock accounts");
    accounts.insert(account, ());
    
    Json(Response {
        status: "success".into(),
        code: None,
        message: None,
    })
}

#[post("/login", format = "json", data = "<login_request>")]
fn login(login_request: Json<RegisterRequest>, accounts: State<'_, Accounts>) -> status::Custom<Json<Response>> {
    let account = Account {
        username: login_request.0.account.clone(),
        password: login_request.0.password.clone(),
    };
    
    let accounts = accounts.lock().expect("lock accounts");
    if accounts.contains_key(&account) {
        status::Custom(Status::Ok, Json(Response {
            status: "success".into(),
            code: None,
            message: None,
        }))
    } else {
        status::Custom(Status::BadRequest, Json(Response {
            status: "error".into(),
            code: Some("2001".into()),
            message: Some("Wrong password or account.".into()),
        }))
    }
}

fn main() {
    rocket::ignite()
        .manage(Mutex::new(HashMap::<Account, ()>::new()))
        .mount("/", routes![register, login])
        .launch();
}
