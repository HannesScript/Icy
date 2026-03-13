use std::cell::RefCell;
use std::rc::Rc;

type Listener<T> = Box<dyn Fn(&T)>;

pub struct IcyState<T> {
    value: T,
    listeners: Vec<Listener<T>>,
}

impl<T> IcyState<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            listeners: vec![],
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        self.notify();
    }

    pub fn update<F>(&mut self, mutator: F)
    where
        F: FnOnce(&mut T),
    {
        mutator(&mut self.value);
        self.notify();
    }

    pub fn subscribe<F>(&mut self, listener: F)
    where
        F: Fn(&T) + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    fn notify(&self) {
        for listener in &self.listeners {
            listener(&self.value);
        }
    }
}

pub fn new_state<T>(initial: T) -> IcyState<T> {
    IcyState::new(initial)
}

pub fn shared_state<T>(initial: T) -> Rc<RefCell<IcyState<T>>> {
    Rc::new(RefCell::new(IcyState::new(initial)))
}
