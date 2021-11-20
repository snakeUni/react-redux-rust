use crate::Store;

pub type Middleware<State, Action> = fn(&mut Store<State, Action>, Action) -> Option<Action>;
