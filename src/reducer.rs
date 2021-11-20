pub type Reducer<State, Action> = fn(&State, &Action) -> State;

#[macro_export]

macro_rules! combine_reducers {
  ($state:ty, $action:ty, $reducer:ident) => {
    $reducer
  };
  ($state:ty, $action:ty, $first:ident, $($second:ident),+) => (
    | state: &$state, action: $action| -> $state {
      (combine_reducers!($state, $action, $($second),+))(&$first(state, action), action)
    }
  )
}