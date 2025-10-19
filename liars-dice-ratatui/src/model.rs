use crate::components::player_table::PlayerTableComponent;
use crate::tuirealm_data::{Id, Msg, UserEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use std::time::Duration;
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge};
use tuirealm::{Application, EventListenerCfg};

pub struct Model<T>
where
    T: TerminalAdapter,
{
    pub app: Application<Id, Msg, UserEvent>,
    pub terminal: TerminalBridge<T>,
    pub quit: bool,
    pub redraw: bool,
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    pub fn do_redraw(&mut self) {
        self.terminal
            .draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Fill(1), Constraint::Fill(2)]);
                self.app.view(&Id::PlayerTable, frame, frame.area());
            })
            .unwrap();
    }
}

impl Default for Model<CrosstermTerminalAdapter> {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init_crossterm().expect("Cannot initialize terminal"),
        }
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
        app.mount(
            Id::PlayerTable,
            Box::new(PlayerTableComponent::default()),
            vec![],
        )
        .unwrap();
        app
    }

    fn redraw(&mut self) {
        self.redraw = false;
    }
}
