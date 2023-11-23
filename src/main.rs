pub mod lobby;
pub mod message;
pub mod websocket;

use actix::{Actor, Addr};
use actix_web::{
    get,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_actors::ws;

use websocket::MyWebsocket;

use lobby::Lobby;

#[get("/ws")]
pub async fn start_connection(
    req: HttpRequest,
    stream: web::Payload,
    lobby: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebsocket::new(lobby.get_ref().clone()), &req, stream)?;
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let lobby = Lobby::default().start();
    HttpServer::new(move || {
        App::new()
            .service(start_connection)
            .app_data(Data::new(lobby.clone()))
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
