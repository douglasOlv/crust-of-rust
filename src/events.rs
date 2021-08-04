pub struct EventEmiter<T> {
    listeners: Vec<Listener<T>>,
}

impl<T> EventEmiter<T> {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    pub fn on(&mut self, name: &str, listener: fn(_: &T)) {
        let listener = Listener {
            listener,
            name: String::from(name),
            once: true,
        };
        self.listeners.push(listener)
    }
    pub fn emit(&mut self, name: &str, value: &T) {
        for item in self.listeners.iter() {
            if name == item.name {
             let func =  item.listener;
             func(value);
            }
        }
    }
}

struct Listener<T> {
    name: String,
    once: bool,
    listener: fn(_: &T),
}
