use actix_web::{post, get, web, web::Data, HttpResponse, App, Responder};
use tera::{Tera, Context};
use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap;

#[get("/")]
async fn home(tera: Data<Tera>) -> impl Responder {
    let client = Client::new();
    let response = client.get("https://ipinfo.io/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let ip_info: Value = serde_json::from_slice(response.as_bytes()).unwrap();
    
    let mut home_context = Context::new();
    home_context.insert("ip_info", &ip_info);

    HttpResponse::Ok().body(tera.render("main.html", &home_context).unwrap())
}

#[get("/search_ip")]
async fn search_ip(ip: web::Query<HashMap<String, String>>, tera: Data<Tera>) -> impl Responder {
    let ip_address = {
        if let Some(ip) = ip.get("ip") {
            ip.to_string()
        } else {
            String::new()
        }
    };
    
    let client = Client::new();
    let response = client.get(format!("https://ipinfo.io/{}/json", &ip_address))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let ip_info: Value = serde_json::from_slice(response.as_bytes()).unwrap();

    let mut search_ip_context = Context::new();
    search_ip_context.insert("ip_info", &ip_info);

    HttpResponse::Ok().body(tera.render("components/search_results.html", &search_ip_context).unwrap())
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
	    .service(search_ip)
            .service(home)
    })
	.bind(("0.0.0.0", 8000))?
	.run()
	.await
}
