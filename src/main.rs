#[macro_use]
extern crate lazy_static;

mod style;
pub(crate) mod utils;

use sailfish::TemplateOnce;
use style::{ColorPalette, StyleTemplate};
use utils::{cheap_rng, random_tagline};
use warp::{http::StatusCode, reply, Filter, Rejection, Reply};

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

async fn index_route() -> impl Reply {
    let rng = cheap_rng();
    let ctx = IndexTemplate {
        title: "cum cannon".to_string(),
        tagline: random_tagline(rng).to_string(),
    };

    let body = ctx.render_once().unwrap();

    warp::reply::html(body)
}

async fn style_route() -> impl Reply {
    let rng = cheap_rng();
    let ctx = StyleTemplate {
        palette: ColorPalette::random(rng),
    };

    let body = ctx.render_once().unwrap();

    warp::reply::with_header(body, "Content-Type", "text/css")
}

async fn rejection_handler(reject: Rejection) -> Result<impl Reply, Rejection> {
    if reject.is_not_found() {
        Ok(reply::with_status("Page not found", StatusCode::NOT_FOUND))
    } else {
        Ok(reply::with_status(
            "Internal server error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

#[tokio::main]
async fn main() {
    let index = warp::path::end().then(index_route);
    let style = warp::path("style").then(style_route);
    let static_files = warp::path("res").and(warp::fs::dir(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources"
    )));

    let routes = warp::get()
        .and(index.or(style).or(static_files))
        .recover(rejection_handler);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
