use crate::components::choice_panel::ChoicePanel;
use crate::components::player_table::PlayerTableComponent;
use crate::tuirealm_data::{Id, Msg, UserEvent};
use liars_dice_lib::LiarsDiceGame;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha12Rng;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use std::time::Duration;
use tuirealm::props::{TableBuilder, TextSpan};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge};
use tuirealm::{Application, AttrValue, Attribute, EventListenerCfg, Update};
use uuid::Uuid;

pub struct Model<T>
where
    T: TerminalAdapter,
{
    pub app: Application<Id, Msg, UserEvent>,
    pub terminal: TerminalBridge<T>,
    pub game: LiarsDiceGame,
    pub quit: bool,
    pub redraw: bool,
}

impl Default for Model<CrosstermTerminalAdapter> {
    fn default() -> Self {
        let mut terminal = TerminalBridge::init_crossterm().expect("Cannot initialize terminal");
        terminal.enable_raw_mode().expect("Cannot enable raw mode");
        terminal
            .enter_alternate_screen()
            .expect("Cannot enter alternate screen");
        terminal.clear_screen().expect("Cannot clear screen");
        let rng = ChaCha12Rng::seed_from_u64(123);
        let players = [
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        ];
        let game = LiarsDiceGame::new(rng, 5, &players);
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal,
            game,
        }
    }
}

impl<T> Drop for Model<T>
where
    T: TerminalAdapter,
{
    fn drop(&mut self) {
        self.terminal.leave_alternate_screen().unwrap();
        self.terminal.disable_raw_mode().unwrap();
        self.terminal.disable_mouse_capture().unwrap();
    }
}

impl<T> Update<Msg> for Model<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        todo!()
    }
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    fn init_app() -> Application<Id, Msg, UserEvent> {
        let mut app: Application<Id, Msg, UserEvent> = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
        app.mount(Id::ChoicePanel, Box::new(ChoicePanel::default()), vec![])
            .unwrap();
        app.mount(
            Id::PlayerTable,
            Box::new(PlayerTableComponent::default()),
            vec![],
        )
        .unwrap();
        app.active(&Id::ChoicePanel).unwrap();
        app
    }

    pub fn do_redraw(&mut self) {
        self.terminal
            .draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Fill(2), Constraint::Fill(1)])
                    .split(frame.area());
                let players = self.game.player_dices();
                let mut table = TableBuilder::default()
                    .add_col("Player ID".into())
                    .add_col("Dice".into())
                    .build();
                for (p, dice) in players {
                    table.push(vec![
                        TextSpan::new(p),
                        TextSpan::new(
                            dice.into_iter()
                                .map(|d| match d.0 {
                                    1 => "⚀1",
                                    2 => "⚁2",
                                    3 => "⚂3",
                                    4 => "⚃4",
                                    5 => "⚄5",
                                    6 => "⚅6",
                                    _ => unreachable!(),
                                })
                                .collect::<Vec<&str>>()
                                .join(" "),
                        ),
                    ])
                }
                self.app
                    .attr(
                        &Id::PlayerTable,
                        Attribute::Content,
                        AttrValue::Table(table),
                    )
                    .unwrap();
                self.app.view(&Id::ChoicePanel, frame, chunks[0]);
                self.app.view(&Id::PlayerTable, frame, chunks[1]);
            })
            .unwrap();
        self.redraw = false;
    }
}
