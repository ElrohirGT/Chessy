use std::collections::HashMap;
use std::sync::{atomic::AtomicUsize, Arc};

use actix::{prelude::Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Game {}

#[derive(Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Debug)]
pub struct ChessServer {
    sessions: HashMap<usize, Recipient<Message>>,
    games: HashMap<String, Game>,
    sessions_opened_count: Arc<AtomicUsize>,
}
