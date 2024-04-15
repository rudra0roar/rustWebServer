use actix_web::{get, web::Path, App, HttpResponse, HttpServer, Responder};


#[get("/home")]
async fn home() -> impl Responder {
    let response = "Welcome to Actix Web Server";
    response
}


#[get("/hello/{firsname}/{lastname}")]
async fn hello_user(params : Path<(String , String)>) -> impl Responder {
    let response : String = format!("Hello {} {}" , params.0 , params.1);
    // println!("{}" ,response);
    // HttpResponse::Ok().body(response) 
    response
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(home).service(hello_user)
    ).bind(("127.0.0.1" , 8000))?.run();
    println!("Server is running at 127.0.0.1:8000");
    server.await
}
