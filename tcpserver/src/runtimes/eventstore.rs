use super::{ ElmModel };

pub struct Model<TEvents> {
    inner: Vec<TEvents>
}

impl<TEvents> Default for Model<TEvents> {
    fn default() -> Self {
        Model {
            inner: Vec::new(),
        }
    }
}

impl<TEvents> ElmModel for Model<TEvents> {
    type TEvents = TEvents;

    fn update(&mut self, event: Self::TEvents) {
        self.inner.push(event);
    }
}