use actix_web::{get, web::Data, HttpResponse, App, Responder};
use tera::{Tera, Context};
use serde_json::Value;
use reqwest::Client;

#[get("/")]
async fn home(tera: Data<Tera>) -> impl Responder {
    let client = Client::new();
    let response = client.get("https://ipinfo.io/json?token=f550d475270a11")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let ip_info: Value = serde_json::from_slice(response.as_bytes()).unwrap();
    
    let mut context = Context::new();
    context.insert("ip_info", &ip_info);

    HttpResponse::Ok().body(tera.render("main.html", &context).unwrap())
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::debug!("Starting Server");

    let tera = Data::new(Tera::new("./templates/**/*.html").unwrap());

    actix_web::HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
	    .service(actix_files::Files::new("/css", "./css").show_files_listing())
            .service(home)
    })
	.bind(("127.0.0.1", 8000))?
	.run()
	.await
}
