
use super::{ ElmModel };
// use std::fmt::{ Debug };

#[derive(Clone, Debug)]
pub enum Events {
    Foo (String),
    Bar (String),
}

#[derive(Clone, Debug)]
pub struct Model {
    value: String,
}

impl Model {
    pub fn value(&self) -> String {
        self.value.to_owned()
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            value: String::default(),
        }
    }
}

impl ElmModel for Model {
    type TEvents = Events;

    fn update(&mut self, event: Self::TEvents) {
        self.value = format!("{}-{:?}", self.value, event);
    }
}
