use crate::model::Model;
use crate::tuirealm_data::Msg;
use tuirealm::PollStrategy;

pub mod components;
pub mod model;
mod tuirealm_data;

pub fn main() {
    let mut model = Model::default();
    while !model.quit {
        if let Ok(m) = model.app.tick(PollStrategy::Once) {
            for message in m {
                match message {
                    Msg::AppClose => {
                        model.quit = true;
                    }
                }
            }
        }
        if model.redraw {
            model.do_redraw();
        }
    }
}
