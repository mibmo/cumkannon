#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

mod style;
mod utils;

use rocket::http::{ContentType, Status};
use rocket::{Request, Response};
use rocket_contrib::serve::StaticFiles;
use sailfish::{RenderError, TemplateOnce};
use std::io::Cursor;
use utils::random_tagline;

#[derive(TemplateOnce)]
#[template(path = "index.html.stpl")]
struct IndexTemplate {
    title: String,
    tagline: String,
}

#[derive(TemplateOnce)]
#[template(path = "404.html.stpl")]
struct NotFoundTemplate {
    path: String,
}

#[get("/")]
fn index<'a>() -> Result<Response<'a>, RenderError> {
    let ctx = IndexTemplate {
        title: "cum cannon".to_string(),
        tagline: random_tagline().to_string(),
    };

    let body = ctx.render_once()?;

    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::HTML)
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

#[catch(404)]
fn not_found<'a>(req: &Request) -> Result<Response<'a>, RenderError> {
    let ctx = NotFoundTemplate {
        path: req.uri().to_string(),
    };

    let body = ctx.render_once()?;

    let response = Response::build()
        .status(Status::NotFound)
        .header(ContentType::HTML)
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

fn main() {
    let routes = routes![style::route, index];
    let catchers = catchers![not_found];
    let static_files = StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/resources"));

    rocket::ignite()
        .register(catchers)
        .mount("/res", static_files)
        .mount("/", routes)
        .launch();
}
