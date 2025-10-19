mod state;
#[cfg(test)]
mod test;

use crate::state::{GameCompleteState, GameStartState, LiarsDiceState, PlayerTurnState};
use rand::seq::IndexedRandom;
use rand::Rng;
use rand_chacha::ChaCha12Rng;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct LiarsDiceGame {
    rng: ChaCha12Rng,
    starting_dice: u8,
    original_player_order: Vec<Uuid>,
    current_players: Vec<Uuid>,
    current_player: Option<Uuid>,
    players: BTreeMap<Uuid, LiarsDicePlayer>,
}

#[derive(Debug)]
pub struct LiarsDicePlayer {
    pub id: Uuid,
    pub dice: Vec<Dice>,
    pub last_call: Option<DiceCall>,
}

impl LiarsDiceGame {
    pub fn new(mut rng: ChaCha12Rng, starting_dice: u8, player_ids: &[Uuid]) -> Self {
        let original_player_order: Vec<Uuid> = player_ids.iter().cloned().collect();
        let mut players = BTreeMap::new();
        for p in player_ids {
            players.insert(
                *p,
                LiarsDicePlayer {
                    id: *p,
                    dice: (0..starting_dice).map(|_| Dice::roll(&mut rng)).collect(),
                    last_call: None,
                },
            );
        }
        assert!(starting_dice > 0);
        assert!(player_ids.len() > 1);
        assert_eq!(players.len(), player_ids.len(), "Player IDs must be unique");
        Self {
            original_player_order,
            rng,
            starting_dice,
            players,
            current_player: None,
            current_players: vec![],
        }
    }

    /// All players get their dice back
    pub fn return_all_dice_for_new_game(&mut self) {
        for (_, player) in self.players.iter_mut() {
            player.dice.clear();
            player
                .dice
                .extend((0..self.starting_dice).map(|_| Dice::roll(&mut self.rng)));
            player.last_call = None;
        }
        self.current_players = self.original_player_order.clone();
        self.current_player = self.current_players.choose(&mut self.rng).cloned();
    }

    pub fn start_next_round(&mut self) {
        for id in &self.current_players {
            let player = self.players.get_mut(id).unwrap();
            for d in player.dice.iter_mut() {
                *d = Dice::roll(&mut self.rng);
            }
            player.last_call = None
        }
    }

    /// Returns the calls of the previous players, ordered by recency
    pub fn previous_calls(&self) -> Vec<(Uuid, usize, Option<DiceCall>)> {
        let mut player_ids: Vec<_> = self.current_players.iter().collect();
        // Player IDs rotate right, so we want to reverse the order
        player_ids.reverse();
        // Now we rotate the player IDs until current player is last
        while let Some(last) = player_ids.last()
            && *last != self.current_player.as_ref().unwrap()
        {
            let first = player_ids.remove(0);
            player_ids.push(first);
        }
        player_ids
            .into_iter()
            .map(|p| {
                let player = self.players.get(p).unwrap();
                (*p, player.dice.len(), player.last_call)
            })
            .collect()
    }

    pub fn player_dices(&self) -> BTreeMap<Uuid, Vec<Dice>> {
        self.players
            .iter()
            .map(|(id, player)| (*id, player.dice.clone()))
            .collect()
    }

    pub fn pick_roller_or_next(&self, who: &Uuid) -> RollerOrNext {
        self.current_players
            .iter()
            .enumerate()
            .filter(|(_, id)| id == &who)
            .map(|(index, player_id)| RollerOrNext {
                index,
                player_id: *player_id,
            })
            .next()
            .unwrap()
    }

    pub fn remove_dice_from_player(&mut self, player_id: &Uuid) {
        let player = self.players.get_mut(player_id).unwrap();
        player.dice.pop();
        if player.dice.is_empty() {
            self.current_players
                .retain(|candidate| candidate != player_id);
        }
    }

    pub fn rotate_player(&mut self) {
        let player_id = self.current_player.as_ref().unwrap().clone();
        let (index, _) = self
            .current_players
            .iter()
            .enumerate()
            .filter(|(_, id)| *id == &player_id)
            .next()
            .unwrap();
        let next_index = (index + 1) % self.current_players.len();
        let player_id = self.current_players[next_index];
        self.current_player = Some(player_id);
    }

    pub fn get_state(&mut self) -> LiarsDiceState<'_> {
        match &self.current_player {
            None => LiarsDiceState::Start(GameStartState { game: self }),
            Some(player_id) => {
                // There is a player, so are there players with remaining dice?
                let players_with_dice = self
                    .players
                    .iter()
                    .map(|(_, p)| p.dice.len())
                    .filter(|d| *d > 0)
                    .count();
                match players_with_dice {
                    1 => LiarsDiceState::GameComplete(GameCompleteState {
                        winner_id: Default::default(),
                        game: self,
                    }),
                    0 => unreachable!(),
                    _ => {
                        let player_id = *player_id;
                        LiarsDiceState::PlayerTurn(PlayerTurnState {
                            game: self,
                            player_id,
                        })
                    }
                }
            }
        }
    }
}

pub struct RollerOrNext {
    pub index: usize,
    pub player_id: Uuid,
}

impl RollerOrNext {
    pub fn set_correct_player(self, game: &mut LiarsDiceGame) {
        if game.current_players.contains(&self.player_id) {
            game.current_player = Some(self.player_id);
        } else {
            // This is an oversimplification, as many people can be removed
            let index = self.index % game.current_players.len();
            game.current_player = Some(game.current_players[index]);
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Dice(u8);

impl Dice {
    pub fn roll(rng: &mut ChaCha12Rng) -> Self {
        Dice(rng.random_range(1..=6))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DiceCall {
    Bullshit,
    SpotOn,
    Increase { count: usize, value: Dice },
}

pub struct CallOutcome {
    player_dices: BTreeMap<Uuid, Vec<Dice>>,
    tally: BTreeMap<Dice, usize>,
    correct_call: bool,
}
