use crate::{Middleware, Reducer, Subscription, Vec};

pub struct Store<State, Action> {
  reducer: Reducer<State, Action>,
  state: State,
  middleware: Vec<Middleware<State, Action>>,
  subscriptions: Vec<Subscription<State>>,
}

impl<State, Action> Store<State, Action> {
  pub fn new(reducer: Reducer<State, Action>, initial_state: State) -> Self {
    Self {
      reducer,
      state: initial_state,
      middleware: Vec::new(),
      subscriptions: Vec::new(),
    }
  }

  pub fn get_state(&self) -> &State {
    &self.state
  }

  pub fn dispatch(&mut self, action: Action) {
    if self.middleware.is_empty() {
      self.dispatch_reducer(&action);
    } else {
      self.dispatch_middleware(0, action);
    }
  }

  pub fn dispatch_middleware(&mut self, index: usize, action: Action) {
    if index == self.middleware.len() {
      self.dispatch_reducer(&action);
      return;
    }

    let next = self.middleware[index](self, action);

    if next.is_none() {
      return;
    }

    self.dispatch_middleware(index + 1, next.unwrap())
  }

  pub fn dispatch_reducer(&mut self, action: &Action) {
    self.state = (&self.reducer)(self.get_state(), action);
    self.dispatch_subscriptions();
  }

  pub fn dispatch_subscriptions(&self) {
    for subscription in &self.subscriptions {
      subscription(self.get_state());
    }
  }

  pub fn subscribe(&mut self, callback: Subscription<State>) -> impl FnMut() -> () + '_ {
    self.subscriptions.push(callback);

    // 创建一个闭包
   move || {
      let idx = self.subscriptions.len();
      self.subscriptions.remove(idx);
    }
  }


  pub fn add_middleware(&mut self, middleware: Middleware<State, Action>) {
    self.middleware.push(middleware);
  }

  pub fn replace_reducer(&mut self, reducer: Reducer<State, Action>) {
    self.reducer = reducer;
  }
}
