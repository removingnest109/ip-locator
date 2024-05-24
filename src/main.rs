use actix_web::{get, App, web::Data, HttpResponse, HttpServer, Responder};

use tera::{Tera, Context};


#[get("/")]

async fn home(tera: Data<Tera>) -> impl Responder {

    HttpResponse::Ok().body(tera.render("main.html", &Context::new()).unwrap())

}



#[actix::main]

async fn main() -> std::io::Result<()> {

    env_logger::init();

    log::debug!("Starting Server");



    let tera = Data::new(Tera::new("./templates/**/*.html").unwrap());



    HttpServer::new( move || {

        App::new()

            .app_data(tera.clone())

            .service(home)

    })

        .bind(("127.0.0.1", 8000))?

        .run()

        .await

}
