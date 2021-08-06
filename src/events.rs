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
            key: String::from(name),
        };
        self.listeners.push(listener)
    }
    pub fn emit(&mut self, name: &str, value: &T) {
        for item in self.listeners.iter() {
            let Listener { key, listener } = item;
            if name == key {
                listener(value);
            }
        }
    }
}

struct Listener<T> {
    key: String,
    listener: fn(_: &T),
}
