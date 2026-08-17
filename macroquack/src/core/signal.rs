// struct DynObjectCallbackEvent<T>
// where T : GameLoop + Sized
// {value: T}

pub enum Signal {
    EmptyCallback,
    IntCallback(i32),
    BoolCallback(bool),
    LoadApp(String),
}

#[derive(Default)]
pub struct SignalDispatcher {
    subscribers: Vec<Box<dyn Fn(&Signal)>>,
}

impl SignalDispatcher {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
    // Register a new listener
    pub fn connect(&mut self, listener: impl Fn(&Signal) + 'static) {
        self.subscribers.push(Box::new(listener));
    }

    pub fn emit(&self, event: &Signal) {
        for listener in &self.subscribers {
            listener(event);
        }
    }
}
