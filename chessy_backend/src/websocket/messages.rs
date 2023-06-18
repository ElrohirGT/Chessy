use actix::prelude::*;
use chess_engine::BoardMovement;
use uuid::Uuid;

use crate::{game::Game, player::Player};

type Client = Recipient<GameMessage>;

pub enum InvalidMovement {
    GameEnded,
    MovementDoesntRemoveCheck,
    MovementCausesCheck,
    MovementDoesntFollowPiecePattern,
}

pub enum WinReasons {
    Checkmate,
    OpponentSurrenders,
    OpponentLostOnTime,
    OpponentDisconnected,
}

pub enum LooseReasons {
    Checkmate,
    NoTimeLeft,
    YouSurrendered,
}

pub enum DrawReasons {
    Stalemate,
    Agreement,
}

pub enum GameEndedReason {
    YouWin(WinReasons),
    YouLoose(LooseReasons),
    Draw(DrawReasons),
}

/// Server sends this message to sessions
#[derive(Message)]
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
    JoinedGame,
    GameFull,
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
