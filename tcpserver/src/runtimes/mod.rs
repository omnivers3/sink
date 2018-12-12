pub mod concatview;
pub mod eventstore;

pub mod domain;
// pub mod elm;
// pub mod threaded;
// pub mod unthreaded;

use super::stdio::*;

use sink::*;
use std::cell::RefCell;

pub struct ElmModelRefCell<TModel>
where
    TModel: ElmModel,
{
    model: RefCell<TModel>,
}

impl<TModel> ElmModelRefCell<TModel>
where
    TModel: ElmModel,
{
    pub fn new(model: TModel) -> Self {
        ElmModelRefCell {
            model: RefCell::new(model),
        }
    }
}

impl<TModel> Default for ElmModelRefCell<TModel>
where
    TModel: Default + ElmModel,
{
    fn default() -> Self {
        Self::new(TModel::default())
    }
}

impl<TModel> Sink for ElmModelRefCell<TModel>
where
    TModel: ElmModel,
{
    type TInput = TModel::TEvents;
    type TResult = ();

    fn send(&self, event: Self::TInput) -> Self::TResult {
        self.model.borrow_mut().update(event);
    }
}

pub struct Model {
    eventstore: eventstore::Model<StdinEvents>,// Vec<StdinEvents>,
    // concatview: concatview::Model,// String::default(),
}

impl Default for Model {
    fn default() -> Self {
        Model {
            eventstore: eventstore::Model::default(),
            // concatview: concatview::Model::default(),
        }
    }
}

pub trait ElmModel {
    type TEvents;

    fn update(&mut self, event: Self::TEvents);
}

impl ElmModel for Model {
    type TEvents = StdinEvents;

    fn update(&mut self, event: Self::TEvents) {
        self.eventstore.update(event.clone());
        // self.concatview.update(event);
    }
}

// type ElmRefCell<T: ElmModel> = RefCell<T>;

pub fn parse_stdin(input: StdinEvents) -> Option<concatview::Events> {
    match input {
        StdinEvents::LineReceived (_, line) => {
            if line.len() % 10 == 0 {
                None
            } else if line.len() % 2 == 0 {
                Some (concatview::Events::Bar(line))
            } else {
                Some (concatview::Events::Foo(line))
            }
        }
        _ => None
    }
}
