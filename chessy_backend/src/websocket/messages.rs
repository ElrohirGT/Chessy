use std::sync::Arc;

use actix::prelude::*;
use chess_engine::{BoardMovement, MovementError};
use serde::Serialize;
use uuid::Uuid;

use crate::{game::Game, player::Player};

type Client = Recipient<GameMessage>;

#[derive(Serialize, Debug)]
pub enum WinReasons {
    Checkmate(Game),
    OpponentSurrenders,
    OpponentLostOnTime,
    OpponentDisconnected,
}

#[derive(Serialize, Debug)]
pub enum LooseReasons {
    Checkmate(Game),
    NoTimeLeft,
    YouSurrendered,
}

#[derive(Serialize, Debug)]
pub enum DrawReasons {
    Stalemate(Game),
    Agreement,
}

#[derive(Serialize, Debug)]
pub enum GameEndedReason {
    YouWin(WinReasons),
    YouLoose(LooseReasons),
    Draw(DrawReasons),
}

/// Server sends this message to sessions
#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub enum GameMessage {
    /// The server responds with this message when the game is created.
    /// The response is the game UUID.
    GameCreated(Uuid),
    /// The server responds with this message when a player joined.
    /// The response is all the necessary data to display a game.
    PlayerJoined(Game),
    ///The server responds with this message when a board movement was played.
    ///The response is a result, that can either be `Ok or `Err`.
    BoardMovement(Result<Game, MovementError>),
    /// The server responds with this message when a game has reached a state where it ends.
    /// The response is the reason it ended.
    GameEnded(GameEndedReason),
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "Uuid")]
pub struct CreateGame {
    pub client_id: Uuid,
    pub name: Arc<str>,
    pub client: Client,
}

#[derive(Debug)]
pub enum JoinedGameResponses {
    /// The client joined a game.
    JoinedGame,
    /// The game the client tried to join was full.
    GameFull,
    /// The game the client tried to join was not found.
    GameNotFound,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "JoinedGameResponses")]
pub struct JoinGame {
    pub game_id: Uuid,
    pub client_id: Uuid,
    pub name: Arc<str>,
    pub client: Client,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveGame {
    pub game_id: Uuid,
    pub client_id: Uuid,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMovement {
    pub game_id: Uuid,
    pub client_id: Uuid,
    pub movement: BoardMovement,
}
