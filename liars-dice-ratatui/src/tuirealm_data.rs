#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Refresh,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ChoicePanel,
    PlayerTable,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UserEvent {}
