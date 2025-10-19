use crate::state::{GameCompleteState, GameStartState, LiarsDiceState, PlayerTurnState};
use crate::{Dice, LiarsDiceGame};
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::collections::BTreeMap;
use uuid::Uuid;

#[test]
fn test_game() {
    let rng = ChaCha12Rng::seed_from_u64(123);
    let players = [
        Uuid::parse_str("EDD38087-18EA-46F8-AF87-AE41E8624E58").unwrap(),
        Uuid::parse_str("6676526B-926E-4413-96A8-A4742071BE8C").unwrap(),
        Uuid::parse_str("8CBBB149-C524-4309-855E-BFBCFD43BD8D").unwrap(),
    ];
    let mut game = LiarsDiceGame::new(rng, 3, &players);
    let state = game.get_state();
    state
        .expect_start()
        .expect("Game didnt start in start state")
        .initialise_game();
    assert_eq!(
        BTreeMap::from_iter(game.player_dices().into_iter()),
        BTreeMap::from([
            (players[0], vec![Dice(1), Dice(5), Dice(4)]),
            (players[1], vec![Dice(4), Dice(5), Dice(6)]),
            (players[2], vec![Dice(4), Dice(2), Dice(6)]),
        ])
    );
    let turn = game.get_state().expect_player_turn().unwrap();
    assert_eq!(
        turn.game.current_players,
        vec![players[1], players[2], players[0]]
    );
    assert_eq!(turn.player_id, players[0]);
    assert_eq!(
        turn.game.previous_calls(),
        vec![
            (players[2], 3, None),
            (players[1], 3, None),
            (players[0], 3, None),
        ]
    );
}

impl<'a> LiarsDiceState<'a> {
    pub fn expect_player_turn(self) -> Result<PlayerTurnState<'a>, LiarsDiceState<'a>> {
        match self {
            LiarsDiceState::PlayerTurn(pt) => Ok(pt),
            _ => Err(self),
        }
    }

    pub fn expect_start(self) -> Result<GameStartState<'a>, LiarsDiceState<'a>> {
        match self {
            LiarsDiceState::Start(ss) => Ok(ss),
            _ => Err(self),
        }
    }

    pub fn expect_complete(self) -> Result<GameCompleteState<'a>, LiarsDiceState<'a>> {
        match self {
            LiarsDiceState::GameComplete(gc) => Ok(gc),
            _ => Err(self),
        }
    }
}
