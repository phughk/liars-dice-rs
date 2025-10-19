#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    PlayerTable,
}

#[derive(Debug, PartialEq)]
pub enum UserEvent {}
