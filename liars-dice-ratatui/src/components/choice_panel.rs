use crate::tuirealm_data::{Msg, UserEvent};
use ratatui::layout::Rect;
use ratatui::Frame;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent, State};

pub struct ChoicePanel {}

impl MockComponent for ChoicePanel {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        todo!()
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        todo!()
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        todo!()
    }

    fn state(&self) -> State {
        todo!()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        todo!()
    }
}

impl Component<Msg, UserEvent> for ChoicePanel {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        todo!()
    }
}
