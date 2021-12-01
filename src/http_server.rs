use rocket::Rocket;
//use rocket::get;
//use rocket_contrib::json::Json;
//use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};
use rocket_contrib::json::JsonValue;
use rocket_contrib::serve::StaticFiles;
use rocket_okapi::routes_with_openapi;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use std::env;
use std::sync::Mutex;

use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::{self, BufReader};
use std::io::prelude::*;


use crate::cli;
use crate::db::GlobalState;
use crate::foobar;

use std::{thread, time};

// This is using /admin/diskspace route
#[get("/diskspace")]
fn diskspace() -> String {
    let mut file = File::create("/dev/ttyACM0").unwrap();
    file.write_all(b"diskspace\n");
    file.flush();
    let millis = time::Duration::from_millis(200);
    let now = time::Instant::now();
    thread::sleep(millis);
    let mut f = File::open("/dev/ttyACM0").unwrap();
    let mut contents0 = String::new();
    let mut contents1 = String::new();
    let mut contents2 = String::new();
    let mut contents3 = String::new();
    let mut contents4 = String::new();
    let mut contents5 = String::new();
    let mut contents6 = String::new();
    let mut contents7 = String::new();
    let mut contents8 = String::new();
    let mut contents9 = String::new();
    let mut contentsA = String::new();
    let mut contentsB = String::new();
    let mut contentsC = String::new();
    let mut contentsD = String::new();
    let mut contentsE = String::new();
    let mut contentsF = String::new();
    let mut f = BufReader::new(f);
    f.read_line(&mut contents0);
    f.read_line(&mut contents1);
    f.read_line(&mut contents2);
    f.read_line(&mut contents3);
    f.read_line(&mut contents4);
    f.read_line(&mut contents5);
    f.read_line(&mut contents6);
    f.read_line(&mut contents7);
    f.read_line(&mut contents8);
    f.read_line(&mut contents9);
    f.read_line(&mut contentsA);
    f.read_line(&mut contentsB);
    f.read_line(&mut contentsC);
    f.read_line(&mut contentsD);
    f.read_line(&mut contentsE);
    f.read_line(&mut contentsF);
    format!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", contents0, contents1, contents2, contents3, contents4, contents5, contents6, contents7, contents8, contents9, contentsA, contentsB, contentsC, contentsD, contentsE, contentsF)
}

// This is using /admin/cputemp route
#[get("/cputemp")]
fn cputemp() -> String {
    let mut file = File::create("/dev/ttyACM0").unwrap();
    file.write_all(b"cputemp\n");
    file.flush();
    let millis = time::Duration::from_millis(200);
    let now = time::Instant::now();
    thread::sleep(millis);
    let mut f = File::open("/dev/ttyACM0").unwrap();
    let mut contents1 = String::new();
    //let mut contents2 = String::new();
    let mut f = BufReader::new(f);
    f.read_line(&mut contents1);
    f.read_line(&mut contents1);
    //f.read_line(&mut contents2);
    format!("{}", contents1)
    //format!("{}<br />{}", contents1, contents2)
}


// This is using /admin/uptime route
#[get("/uptime")]
fn uptime() -> String {
    let mut file = File::create("/dev/ttyACM0").unwrap();
    file.write_all(b"uptime\n");
    file.flush();
    let millis = time::Duration::from_millis(200);
    let now = time::Instant::now();
    thread::sleep(millis);
    let mut f = File::open("/dev/ttyACM0").unwrap();
    let mut contents1 = String::new();
    let mut f = BufReader::new(f);
    f.read_line(&mut contents1);
    f.read_line(&mut contents1);
    f.read_line(&mut contents1);
    format!("{}", contents1)
}


// This is using /admin/status route
#[get("/status")]
fn status() -> String {
    format!("Rust says your status is super")
}

// This is using /hello route
#[get("/")]
fn hello() -> String {
    println!("Hello, from Rust");
    format!("Hello, from Rust")
}

// This is using /hello/joe route
#[get("/joe")]
fn message() -> JsonValue {
    json!({ "result" : "success",
            "message" : "Hi from Rust!"
    })
}

/**
Each endpoint has an associated function defined in the corresponding module routes.rs file
*/
/// Launch Rocket HTTP Server
pub fn build_app(opt: cli::Opt) -> Rocket {
    env::set_var("ROCKET_PORT", opt.port.to_string());
    let global_state = Mutex::new(GlobalState::new(opt));

    let openapi_routes = routes_with_openapi![
        foobar::foobar,
        foobar::list,
        // foobar::directory,
    ];

    rocket::ignite()
        .manage(global_state)
        .mount("/hello", routes![hello, message])
        .mount("/admin", routes![status, diskspace, uptime, cputemp])
        // routes for which we have the #[openapi] attribute specified
        .mount("/", openapi_routes)
        // http:<hostname>:<port>/api presents a web page
        // with all the openapi endpoints so you can try them out.
        .mount(
            "/api/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        // The svelte endpoint is available at:
        // http://<hostname>:<port>/
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
}
