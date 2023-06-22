use std::{collections::HashMap, sync::Arc, time::Instant};

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use chess_engine::{BoardMovement, PieceColors};
use rand::{rngs::ThreadRng, Rng};
use uuid::Uuid;

use crate::{
    game::{Game, GameConfig, ServerGame},
    player,
    websocket::GameEndedReason,
};

use super::{CreateGame, GameMessage, JoinGame, JoinedGameResponses, LeaveGame, SendMovement};

#[derive(Default)]
pub struct ChessServer {
    games: HashMap<Uuid, ServerGame>,
    rng: ThreadRng,
}

impl ChessServer {
    fn end_game_because_player_leaved(
        &mut self,
        game_id: &Uuid,
        client_id: &Uuid,
        _ctx: &mut Context<ChessServer>,
    ) {
        let mut remove_game = false;

        if let Some(ServerGame { game, sessions, .. }) = self.games.get_mut(game_id) {
            *sessions = sessions
                .drain()
                .filter(|(id, _)| id != client_id)
                .map(|(id, client)| {
                    let res = client.try_send(GameMessage::GameEnded(GameEndedReason::YouWin(
                        super::WinReasons::OpponentDisconnected,
                    )));
                    ((id, client), res)
                })
                .filter(|(_, result)| result.is_ok())
                .map(|(data, _)| data)
                .collect();

            remove_game = sessions.is_empty();
            game.remove_player(client_id);
        }

        if remove_game {
            self.games.remove(game_id);
        }
    }

    fn make_movement(
        &mut self,
        client_id: &Uuid,
        game_id: &Uuid,
        movement: chess_engine::BoardMovement,
        instant: Instant,
        ctx: &mut Context<ChessServer>,
    ) {
        if let Some(ServerGame {
            game,
            sessions,
            last_move,
        }) = self.games.get_mut(game_id)
        {
            let Game { players, board, .. } = game;
            let BoardMovement { piece, .. } = movement.clone();
            let player = match players.get_mut(piece.color()) {
                Some(p) => p,
                None => {
                    self.end_game_because_player_leaved(game_id, client_id, ctx);
                    return;
                }
            };
            let result = chess_engine::move_piece(movement, board);

            match result {
                Ok(success_type) => {
                    let delta_ms = instant.duration_since(*last_move).as_millis();
                    let time_changed = player.reduce_time_by(delta_ms);
                    match time_changed {
                        crate::player::PlayerTimeChanged::InsufficientTime => {
                            sessions
                                .iter_mut()
                                .map(|(id, client)| {
                                    if id == client_id {
                                        client.try_send(GameMessage::GameEnded(
                                            GameEndedReason::YouLoose(
                                                super::LooseReasons::NoTimeLeft,
                                            ),
                                        ))
                                    } else {
                                        client.try_send(GameMessage::GameEnded(
                                            GameEndedReason::YouWin(
                                                super::WinReasons::OpponentLostOnTime,
                                            ),
                                        ))
                                    }
                                })
                                .for_each(|result| {
                                    log::debug!("Message sent with result: {:?}", result)
                                });
                        }
                        crate::player::PlayerTimeChanged::LastMove
                        | crate::player::PlayerTimeChanged::TimeRemaining => match success_type {
                            chess_engine::MovementSuccess::NormalMovement => {
                                sessions
                                    .iter_mut()
                                    .map(move |(_, client)| {
                                        client
                                            .try_send(GameMessage::BoardMovement(Ok(game.clone())))
                                    })
                                    .for_each(|result| {
                                        log::debug!("Message sent with result: {:?}", result)
                                    });
                            }
                            chess_engine::MovementSuccess::CheckmateMovement => {
                                sessions
                                    .iter_mut()
                                    .map(|(id, client)| {
                                        if id == client_id {
                                            client.try_send(GameMessage::GameEnded(
                                                GameEndedReason::YouWin(
                                                    super::WinReasons::Checkmate(game.clone()),
                                                ),
                                            ))
                                        } else {
                                            client.try_send(GameMessage::GameEnded(
                                                GameEndedReason::YouLoose(
                                                    super::LooseReasons::Checkmate(game.clone()),
                                                ),
                                            ))
                                        }
                                    })
                                    .for_each(|result| {
                                        log::debug!("Message sent with result: {:?}", result)
                                    });
                            }
                            chess_engine::MovementSuccess::StalemateMovement => {
                                sessions
                                    .iter_mut()
                                    .map(|(_, client)| {
                                        client.try_send(GameMessage::GameEnded(
                                            GameEndedReason::Draw(super::DrawReasons::Stalemate(
                                                game.clone(),
                                            )),
                                        ))
                                    })
                                    .for_each(|result| {
                                        log::debug!("Message sent with result: {:?}", result)
                                    });
                            }
                        },
                    }
                }
                Err(error_type) => match sessions.iter_mut().find(|(id, _)| id == &client_id) {
                    Some((id, client)) => {
                        log::debug!("Movement error ({}) on client ({})", error_type, id);
                        let _ = client.try_send(GameMessage::BoardMovement(Err(error_type)));
                    }
                    None => log::debug!(
                        "No client with id ({}) to send the movement error found!",
                        client_id
                    ),
                },
            }
        }
    }

    fn join_game(
        &mut self,
        client_id: Uuid,
        name: Arc<str>,
        game_id: Uuid,
        client: Recipient<GameMessage>,
        _ctx: &mut Context<ChessServer>,
    ) -> JoinedGameResponses {
        match self.games.get_mut(&game_id) {
            Some(game) => {
                if game.is_full() {
                    JoinedGameResponses::GameFull
                } else {
                    let player = game
                        .add_opponent(client_id, name.clone(), client.clone())
                        .unwrap_or_else(|_| {
                            let color = get_random_color(&mut self.rng);
                            game.add_player(client_id, name, client, color)
                        });

                    game.update_last_move();

                    game.sessions
                        .iter_mut()
                        .filter(|(id, _)| id != &&client_id)
                        .for_each(|(_, client)| {
                            let _ = client.try_send(GameMessage::PlayerJoined(player.clone()));
                        });

                    JoinedGameResponses::JoinedGame
                }
            }
            None => JoinedGameResponses::GameNotFound,
        }
    }
}

impl Actor for ChessServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<LeaveGame>(ctx);
        self.subscribe_system_async::<SendMovement>(ctx);
    }
}

impl Handler<CreateGame> for ChessServer {
    type Result = MessageResult<CreateGame>;

    fn handle(&mut self, msg: CreateGame, ctx: &mut Self::Context) -> Self::Result {
        let CreateGame {
            client_id,
            client,
            name,
        } = msg;
        let ChessServer { games, rng } = self;

        let game_id = Uuid::new_v4();
        log::debug!(
            "Client ID (`{}`) started game (`{}`) creation...",
            client_id,
            game_id
        );
        let color = get_random_color(rng);
        log::debug!(
            "Client ID (`{}`) identified as `{}` and will play `{}`",
            client_id,
            name,
            color
        );

        let players = HashMap::from([(color, (client_id, name))]);
        let game_config = GameConfig::new(players, 10 * 60 * 1000);

        let sessions = HashMap::from([(client_id, client)]);
        let game = Game::new(game_config);
        log::debug!("Created game:\n{:?}", game);

        games.insert(game_id.clone(), ServerGame::new(game, sessions));

        MessageResult(game_id)
    }
}

fn get_random_color(rng: &mut ThreadRng) -> PieceColors {
    if rng.gen() {
        PieceColors::Black
    } else {
        PieceColors::White
    }
}

impl Handler<JoinGame> for ChessServer {
    type Result = MessageResult<JoinGame>;

    fn handle(&mut self, msg: JoinGame, ctx: &mut Self::Context) -> Self::Result {
        let JoinGame {
            game_id,
            client_id,
            client,
            name,
        } = msg;

        let response = self.join_game(client_id, name, game_id, client, ctx);
        MessageResult(response)
    }
}

impl Handler<LeaveGame> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveGame, ctx: &mut Self::Context) -> Self::Result {
        let LeaveGame { game_id, client_id } = msg;
        match self.games.get_mut(&game_id) {
            Some(ServerGame { game, sessions, .. }) => {
                sessions.remove(&client_id);
                log::debug!("Client `{}` left the game `{}`", client_id, game_id);
                game.remove_player(&client_id);

                self.end_game_because_player_leaved(&game_id, &client_id, ctx);
            }
            None => log::debug!("No game found with id `{}`", game_id),
        }
    }
}

impl Handler<SendMovement> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: SendMovement, ctx: &mut Self::Context) -> Self::Result {
        let instant = Instant::now();
        let SendMovement {
            game_id,
            client_id,
            movement,
        } = msg;

        self.make_movement(&client_id, &game_id, movement, instant, ctx);
    }
}

impl SystemService for ChessServer {}
impl Supervised for ChessServer {}
