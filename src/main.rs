use shiny_rs::session::{ ShinyServer, ShinySession };
use shiny_rs::{ changed, ui };
use actix_web::{ get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder };
use actix_files::NamedFile;
use actix_web_actors::ws;
use std::time::Duration;
use face::run_model;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}
// These functions take in a ShinyServer and a ShinySession.
// They will run on different moments of our session.
fn initialize(shiny: &mut ShinyServer, session: &mut ShinySession) {
}

fn model_modules(shiny: &mut ShinyServer, session: &mut ShinySession) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!(
        "static/img/{}",
        shiny.input.get_string("path").unwrap_or_default()
    );
    let result_path_err = run_model(&path);
    let result_path = match result_path_err {
        Ok(_) => result_path_err,
        Err(_) => {
            ui::show_notification(
                session,
                ui::args!({
                    "html": "The image does not exist",
                    "type": "error",
                    "closeButton": false,
                })
            );
            return result_path_err
        }
    };
    result_path

}
fn update(shiny: &mut ShinyServer, session: &mut ShinySession) {
    if changed!(shiny, ("run_model:shiny.action")) {
        let result_path = model_modules(shiny, session).unwrap_or_default();
        ui::render_ui(session, "result", &format!(r#"<img src="{}" />"#, result_path));
    }
}
fn tick(shiny: &mut ShinyServer, session: &mut ShinySession) {
}

// These intervals use the `from_secs` function. However,
// you could lower the tick rate or increase it. It's all
// on you!
const HB_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

async fn server(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ShinyServer::new(initialize, update, tick, HB_INTERVAL, CLIENT_TIMEOUT), &req, stream)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(actix_files::Files::new("/lib", "./static/lib"))
            .service(actix_files::Files::new("/img", "./static/img").show_files_listing())
            .service(actix_files::Files::new("/results", "./static/results"))
            .service(web::resource("/websocket/").route(web::get().to(server)))
    })
    .workers(2)
    .bind(("0.0.0.0", 8000))? // Change the port and IP accordingly
    .run()
    .await
}
