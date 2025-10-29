use crate::tuirealm_data::{Msg, UserEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::Key;
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent, State};

pub struct ChoicePanel {}

impl Default for ChoicePanel {
    fn default() -> Self {
        ChoicePanel {}
    }
}

impl MockComponent for ChoicePanel {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .split(area);
        let p = ratatui::widgets::Paragraph::new("Choice Panel - Top");
        frame.render_widget(p, chunks[0]);
        let p = ratatui::widgets::Paragraph::new("Choice Panel - Bottom");
        frame.render_widget(p, chunks[1]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        todo!()
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Focus => {}
            Attribute::Alignment
            | Attribute::Background
            | Attribute::Borders
            | Attribute::Color
            | Attribute::Content
            | Attribute::Dataset
            | Attribute::Direction
            | Attribute::Disabled
            | Attribute::Display
            | Attribute::FocusStyle
            | Attribute::Foreground
            | Attribute::Height
            | Attribute::HighlightedStr
            | Attribute::HighlightedColor
            | Attribute::InputLength
            | Attribute::InputType
            | Attribute::Layout
            | Attribute::Palette
            | Attribute::Rewind
            | Attribute::Shape
            | Attribute::Scroll
            | Attribute::ScrollStep
            | Attribute::Style
            | Attribute::Text
            | Attribute::TextAlign
            | Attribute::TextProps
            | Attribute::TextWrap
            | Attribute::Title
            | Attribute::Value
            | Attribute::Width
            | Attribute::Custom(_) => {
                panic!("Received attribute {attr:?} with value {value:?}")
            }
        }
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
        match ev {
            Event::Keyboard(k) => {
                if k.code == Key::Char('q') {
                    Some(Msg::AppClose)
                } else {
                    None
                }
            }
            Event::Tick | Event::WindowResize(_, _) => Some(Msg::Refresh),
            _ => None,
        }
    }
}
