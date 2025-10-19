use crate::tuirealm_data::{Msg, UserEvent};
use ratatui::layout::Rect;
use ratatui::Frame;
use tui_realm_stdlib::Table as TableComponent;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{TableBuilder, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent, State};

pub struct PlayerTableComponent {
    component: TableComponent,
}

impl Default for PlayerTableComponent {
    fn default() -> Self {
        let mut component = TableComponent::default();
        let mut table = TableBuilder::default()
            .add_col(TextSpan::new("Player"))
            .add_col(TextSpan::new("Dice"))
            .add_col(TextSpan::new("Call"))
            .build();
        table.push(vec!["Player 1".into(), "5".into(), "None".into()]);
        table.push(vec!["Player 2".into(), "3".into(), "4 5".into()]);
        component.attr(Attribute::Content, AttrValue::Table(table));
        PlayerTableComponent { component }
    }
}

impl Component<Msg, UserEvent> for PlayerTableComponent {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        match ev {
            _ => None,
        }
    }
}

impl MockComponent for PlayerTableComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value)
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}
