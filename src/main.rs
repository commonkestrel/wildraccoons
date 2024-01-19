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

    tera.render("index.tera", &Context::new()).unwrap();

    let mut app = tide::with_state(tera);
    app.with(driftwood::DevLogger);
    app.with(refresh::DebugRefresh);


    app.at("/").get(|req: Request<Tera>| async move {
        let tera = req.state();
        tera.render_response("index.tera", &context! {})
    });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
