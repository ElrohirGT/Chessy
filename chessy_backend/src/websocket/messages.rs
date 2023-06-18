use actix::prelude::*;
use chess_engine::BoardMovement;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{game::Game, player::Player};

type Client = Recipient<GameMessage>;

#[derive(Serialize, Debug)]
pub enum InvalidMovement {
    GameEnded,
    MovementDoesntRemoveCheck,
    MovementCausesCheck,
    MovementDoesntFollowPiecePattern,
}

#[derive(Serialize, Debug)]
pub enum WinReasons {
    Checkmate,
    OpponentSurrenders,
    OpponentLostOnTime,
    OpponentDisconnected,
}

#[derive(Serialize, Debug)]
pub enum LooseReasons {
    Checkmate,
    NoTimeLeft,
    YouSurrendered,
}

#[derive(Serialize, Debug)]
pub enum DrawReasons {
    Stalemate,
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
    PlayerJoined(Player),
    BoardMovement(Result<Game, InvalidMovement>),
    GameEnded(GameEndedReason),
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "Uuid")]
pub struct CreateGame {
    pub client_id: Uuid,
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
