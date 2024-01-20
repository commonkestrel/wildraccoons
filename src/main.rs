mod refresh;

use tera::Context;
use tera::Tera;
use tide::prelude::*;
use tide::Request;
use tide_tera::prelude::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    tera.render("index.html", &Context::new()).unwrap();

    let mut app = tide::with_state(tera);
    app.with(driftwood::DevLogger);
    app.with(refresh::DebugRefresh);


    app.at("/").get(|req: Request<Tera>| async move {
        let tera = req.state();
        tera.render_response("index.html", &context! {}).map(|mut res| {
            res.set_content_type(tide::http::mime::HTML);
            res
        })
    });

    app.at("/static").serve_dir("static")?;
    app.at("/css").serve_dir("css")?;
    app.at("/js").serve_dir("js")?;
    app.at("/robots.txt").serve_file("static/robots.txt")?;
    app.at("sitemap.txt").serve_file("static/sitemap.txt")?;
    app.at("favicon.ico").serve_file("static/favicon.ico")?;

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
