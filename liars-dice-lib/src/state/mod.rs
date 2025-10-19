use crate::{CallOutcome, Dice, DiceCall, LiarsDiceGame};
use rand::seq::IndexedRandom;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug)]
pub enum LiarsDiceState<'a> {
    PlayerTurn(PlayerTurnState<'a>),
    GameComplete(GameCompleteState<'a>),
    Start(GameStartState<'a>),
}

#[derive(Debug)]
pub struct PlayerTurnState<'a> {
    pub(crate) game: &'a mut LiarsDiceGame,
    pub player_id: Uuid,
}

impl PlayerTurnState<'_> {
    pub fn propose_choice(&self, call: DiceCall) -> Result<ConfirmChoice, &'static str> {
        // Find the previous call
        let previous_calls = self.game.previous_calls();
        let (_, _, previous) = previous_calls.first().take().unwrap();
        match (previous, call) {
            (None, DiceCall::Increase { count, value }) => Ok(ConfirmChoice(call)),
            (None, _) => Err("First call must be an increase"),
            (
                Some(DiceCall::Increase {
                    count: prev_count,
                    value: prev_val,
                }),
                _,
            ) => match call {
                DiceCall::Increase { count, value } => match value.0 == prev_val.0 {
                    true => match count > *prev_count {
                        true => Ok(ConfirmChoice(call)),
                        false => Err("New count must be higher than previous"),
                    },
                    false => match count > *prev_count {
                        true => Ok(ConfirmChoice(call)),
                        false => Err("New count must be higher than previous"),
                    },
                },
                _ => Ok(ConfirmChoice(call)),
            },
            (Some(_), _) => unreachable!("Previous call should have been final"),
        }
    }
}

pub struct ConfirmChoice(DiceCall);

impl ConfirmChoice {
    pub fn confirm(self, state: PlayerTurnState) -> Option<CallOutcome> {
        match self.0 {
            DiceCall::Increase { count, value } => {
                let player_id = state.game.current_player.unwrap();
                let player = state.game.players.get_mut(&player_id).unwrap();
                player.last_call = Some(DiceCall::Increase { count, value });
                state.game.rotate_player();
                None
            }
            call => {
                let player_dices = state.game.player_dices();
                let tally: BTreeMap<_, _> = player_dices
                    .values()
                    .flat_map(|dice| dice.iter())
                    .fold([0; 6], |mut acc, item| {
                        acc[item.0 as usize - 1] += 1usize;
                        acc
                    })
                    .into_iter()
                    .enumerate()
                    .map(|(index, count)| (Dice((index + 1) as u8), count))
                    .collect();
                let (prev_id, _, previous_call) =
                    state.game.previous_calls().into_iter().next().unwrap();
                let previous_call = previous_call.unwrap();
                let (prev_count, prev_val) = match previous_call {
                    DiceCall::Increase { count, value } => (count, value),
                    _ => unreachable!(),
                };
                let actual_count = *tally.get(&prev_val).unwrap();
                let current_player_id = state.game.current_player.clone().unwrap();
                match call {
                    DiceCall::Bullshit => {
                        let correct_call = actual_count < prev_count;
                        let next_player = state.game.pick_roller_or_next(&current_player_id);
                        match correct_call {
                            true => {
                                // Remove dice from previous
                                state.game.remove_dice_from_player(&prev_id);
                            }
                            false => {
                                // Remove dice from current
                                state.game.remove_dice_from_player(&current_player_id);
                            }
                        };
                        next_player.set_correct_player(state.game);
                        state.game.start_next_round();
                        Some(CallOutcome {
                            player_dices,
                            tally,
                            correct_call,
                        })
                    }
                    DiceCall::SpotOn => {
                        let correct_call = actual_count == prev_count;
                        match correct_call {
                            true => {
                                // Everyone else loses a dice
                                let everyone_else: Vec<_> = state
                                    .game
                                    .current_players
                                    .iter()
                                    .cloned()
                                    .filter(|candidate| candidate != &current_player_id)
                                    .collect();
                                for player_id in everyone_else {
                                    state.game.remove_dice_from_player(&player_id);
                                }
                                state.game.start_next_round();
                                // Caller goes again
                                Some(CallOutcome {
                                    player_dices,
                                    tally,
                                    correct_call,
                                })
                            }
                            false => {
                                // Caller loses a dice
                                let next_player =
                                    state.game.pick_roller_or_next(&current_player_id);
                                state.game.remove_dice_from_player(&state.player_id);
                                next_player.set_correct_player(state.game);
                                Some(CallOutcome {
                                    player_dices,
                                    tally,
                                    correct_call,
                                })
                            }
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct GameCompleteState<'a> {
    pub winner_id: Uuid,
    pub(crate) game: &'a mut LiarsDiceGame,
}

#[derive(Debug)]
pub struct GameStartState<'a> {
    pub(crate) game: &'a mut LiarsDiceGame,
}

impl GameStartState<'_> {
    pub fn initialise_game(self) {
        for (_, player) in self.game.players.iter_mut() {
            player.dice = (0..self.game.starting_dice)
                .map(|_| Dice::roll(&mut self.game.rng))
                .collect();
        }
        self.game.current_players.clear();
        self.game.current_players = self.game.players.keys().cloned().collect();
        let p = self
            .game
            .current_players
            .choose(&mut self.game.rng)
            .cloned()
            .expect("Random choice should have worked");
        self.game.current_player = Some(p);
    }
}
