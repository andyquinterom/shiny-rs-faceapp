use shiny_rs::session::{ ShinyServer, ShinySession };
use shiny_rs::{ changed, ui };
use actix_web::{ get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder };
use actix_files::NamedFile;
use actix_web_actors::ws;
use std::time::Duration;
use std::fs;
use face::run_model;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}
// These functions take in a ShinyServer and a ShinySession.
// They will run on different moments of our session.
fn initialize(shiny: &mut ShinyServer, session: &mut ShinySession) {
    {
        let mut options: Vec<(String, String)> = vec!();
        for file in fs::read_dir("./static/inputs").unwrap() {
            let name: String = file.unwrap().file_name().to_str().unwrap().to_string();
            if !(name.ends_with(".jpg") | name.ends_with(".png") | name.ends_with(".jpeg")) {
                continue;
            }
            options.push((name.clone(), name));
        }
        ui::update_select_input(
            session,
            "path",
            ui::args!({
                "options": ui::select_options(options)
            })
        )
    }
}

fn update(shiny: &mut ShinyServer, session: &mut ShinySession) {
    if changed!(shiny, ("run_model:shiny.action")) {
        let img = shiny.input.get_string("path").unwrap_or_default();
        ui::render_ui(
            session,
            "result",
            &format!(r#"<img src="/detect_faces/{}" class="loading"/>"#, img)
        );
    }
}
fn tick(shiny: &mut ShinyServer, session: &mut ShinySession) {
}

// These intervals use the `from_secs` function. However,
// you could lower the tick rate or increase it. It's all
// on you!
const HB_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

async fn server(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ShinyServer::new(initialize, update, tick, HB_INTERVAL, CLIENT_TIMEOUT), &req, stream)
}

#[get("/detect_faces/{img}")]
async fn greet(img: web::Path<String>) -> impl Responder {
    let path = format!(
        "static/inputs/{}",
        img
    );
    let result_path = run_model(&path).unwrap();
    NamedFile::open_async(&result_path).await.unwrap()
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(actix_files::Files::new("/css", "./static/css"))
            .service(actix_files::Files::new("/lib", "./static/lib"))
            .service(actix_files::Files::new("/inputs", "./static/inputs"))
            .service(actix_files::Files::new("/img", "./static/img"))
            .service(actix_files::Files::new("/results", "./static/results"))
            .service(greet)
            .service(web::resource("/websocket/").route(web::get().to(server)))
    })
    .workers(4)
    .bind(("0.0.0.0", 8000))? // Change the port and IP accordingly
    .run()
    .await
}
