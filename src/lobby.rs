use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use crate::message::{ClientActorMessage, Connect, WsMessage};

type Socket = Recipient<WsMessage>;

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    waiting: Option<Uuid>,
    opponent: HashMap<Uuid, Uuid>,
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.id, msg.addr);

        if let Some(waiting) = self.waiting {
            self.opponent.insert(waiting, msg.id);
            self.opponent.insert(msg.id, waiting);
            self.send_message("{\"type\": \"start\"}", &waiting);
            self.send_message("{\"type\": \"start\"}", &msg.id);
            self.waiting = None;
        } else {
            self.waiting = Some(msg.id);
        }
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(opponent) = self.opponent.get(&msg.id) {
            self.send_message(&msg.msg, opponent);
        }
    }
}
