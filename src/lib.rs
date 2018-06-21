//! This crate aims to provide an abstraction for a thing which can be sent values
//! and, immediately, return a Result indicating success / failure of receipt.
//! 
//! As a base primitive this should enable a message oriented variant of the
//! inbound params to the familiar imperitive Result 'and_then' composition pattern.
//!
//! Main sink trait representing a receiver which immediately responds to send with
//! a Result<TResult, TError> enabling implementations to encapsulate both sync and
//! async processing with a sync response.

/// The ISink trait acts as a target to which many values can be sent
pub trait ISink
{
    type TInput;
    type TResult;
    type TError;

    fn send(&self, input: Self::TInput) -> Result<Self::TResult, Self::TError>;
}

/// Sink is a simple struct which captures a provided handler function and routes
/// sent data into that handler
pub struct Sink<'a, TInput, TResult, TError> {
    handler: Box<Fn(TInput) -> Result<TResult, TError> + 'a>,
}

impl <'a, TInput, TResult, TError> Sink<'a, TInput, TResult, TError> {
    /// Builds a Sink using the provided handler
    pub fn new<F: 'a>(handler: F) -> Self where
        F: Fn(TInput) -> Result<TResult, TError> + 'a
    {
        Sink {
            handler: Box::new(handler),
        }
    }
}

impl <'a, TInput, TResult, TError> ISink for Sink<'a, TInput, TResult, TError>
{
    type TInput = TInput;
    type TResult = TResult;
    type TError = TError;

    fn send(&self, input: <Self as ISink>::TInput) -> Result<<Self as ISink>::TResult, <Self as ISink>::TError> {
        (self.handler)(input)
    }
}

#[cfg(test)]
mod sink_tests {
    use super::*;

    #[test]
    fn should_send_single_item_to_sink() {
        let sink = Sink::<(), (), ()>::new(| _item | {
            Ok (())
        });

        sink.send(()).unwrap();
    }

    #[test]
    fn should_send_multiple_items_to_sink() {
        let sink = Sink::<(), (), ()>::new(| _item | {
            Ok (())
        });

        sink.send(()).unwrap();
        sink.send(()).unwrap();
    }
}

/// Sink implementation which owns an internal state that is made available to
/// the provided handler when values are sent to it
pub struct StatefulSink<'a, TState, TInput, TResult, TError> where
    TState: Clone,
{
    state: TState,
    handler: Box<Fn(TState, TInput) -> Result<TResult, TError> + 'a>,
}

impl <'a, TState, TInput, TResult, TError> StatefulSink<'a, TState, TInput, TResult, TError> where
    TState: Clone,
{
    /// Builds a StatefulSink using the default for TState
    pub fn new<F: 'a>(handler: F) -> Self where
        TState: Default,
        F: Fn(TState, TInput) -> Result<TResult, TError> + 'a
    {
        StatefulSink::with_state(TState::default(), handler)
    }

    /// Builds a StatefulSink using the TState provided
    pub fn with_state<F: 'a>(state: TState, handler: F) -> Self where
        F: Fn(TState, TInput) -> Result<TResult, TError> + 'a
    {
        StatefulSink {
            state: state,
            handler: Box::new(handler),
        }
    }
}

impl <'a, TState, TInput, TResult, TError> ISink for StatefulSink<'a, TState, TInput, TResult, TError> where
    TState: Clone,
{
    type TInput = TInput;
    type TResult = TResult;
    type TError = TError;

    fn send(&self, input: <Self as ISink>::TInput) -> Result<<Self as ISink>::TResult, <Self as ISink>::TError> {
        (self.handler)(self.state.to_owned(), input)
    }
}

#[cfg(test)]
mod stateful_sink_tests {
    use super::*;

    use std::cell::RefCell;

    #[test]
    fn should_send_single_item_to_statefulsink() {
        let sink = StatefulSink::<(), (), (), ()>::new(| _state, _item | {
            Ok (())
        });

        sink.send(()).unwrap();
    }

    #[test]
    fn should_send_multiple_items_to_statefulsink() {
        let sink = StatefulSink::<(), (), (), ()>::new(| _state, _item | {
            Ok (())
        });

        sink.send(()).unwrap();
        sink.send(()).unwrap();
    }

    #[test]
    fn should_update_state_on_send_given_mutable_type() {
        let initial = RefCell::new(10);

        let s = StatefulSink::<&RefCell<usize>, usize, usize, ()>::with_state(&initial, | s, item | {
            let mut value = s.borrow_mut();
            *value += item;
            Ok (value.to_owned())
        });

        assert_eq!(Ok (20), s.send(10));
        assert_eq!(Ok (40), s.send(20));
    }
}

/// Transforms incomming data from source type to the type epxected by the wrapped ISink.
/// 
/// Explicitely building the SinkMap from ::new
/// 
/// ``` rust
/// # use sink_rs::*;
/// 
/// let s = Sink::<usize, usize, ()>::new(| item | {
///    Ok(item)
/// });
/// 
/// let mut sm = SinkMap::new(s, | item: String | { item.len() });
/// 
/// assert_eq!(Ok (0), sm.send("".to_owned()));
/// assert_eq!(Ok (9), sm.send("some text".to_owned()));
/// 
/// ```
/// 
/// Using the 'map' function from any ISink
/// 
/// ``` rust
/// # use sink_rs::*;
/// 
/// let mut s = Sink::<usize, usize, ()>::new(| item | {
///     Ok(item)
/// }).map(| item: String | { item.len() });
/// 
/// assert_eq!(Ok (0), s.send("".to_owned()));
/// assert_eq!(Ok (9), s.send("some text".to_owned()));
/// ```
pub struct SinkMap<'a, TInput, UInput, TResult, TError, TSink: Sized> where
    TSink: ISink<TInput=TInput, TResult=TResult, TError=TError>,
{
    target: TSink,
    map: Box<Fn(UInput) -> TInput + 'a>,
}

impl <'a, TInput, UInput, TResult, TError, TSink> SinkMap<'a, TInput, UInput, TResult, TError, TSink> where
    TSink: ISink<TInput=TInput, TResult=TResult, TError=TError>,
{
    /// Build a new SinkMap which uses the provided map to translate the incoming values
    /// into the target's expected type and an owned target allowing the caller to decide
    /// sharing rules
    pub fn new<F: 'a>(target: TSink, map: F) -> Self where
        F: Fn(UInput) -> TInput + 'a,
    {
        SinkMap {
            target,
            map: Box::new(map),
        }
    }
}

impl <'a, TInput, UInput, TResult, TError, TSink> ISink for SinkMap<'a, TInput, UInput, TResult, TError, TSink> where
    TSink: ISink<TInput=TInput, TResult=TResult, TError=TError>
{
    type TInput = UInput;
    type TResult = TResult;
    type TError = TError;

    fn send(&self, input: <Self as ISink>::TInput) -> Result<<Self as ISink>::TResult, <Self as ISink>::TError> {
        self.target.send((self.map)(input))
    }
}

/// The ISinkMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkMap, generaling it's constructor
pub trait ISinkMap<'a, TInput, TResult, TError, TSink> where
    TSink: ISink<TInput=TInput, TResult=TResult, TError=TError>,
    Self: ISink<TInput=TInput, TResult=TResult, TError=TError>,
{
    fn map<UInput, F: Fn(UInput) -> TInput + 'a>(self, map: F) -> SinkMap<'a, TInput, UInput, TResult, TError, TSink>;
}

impl <'a, T, TInput, TResult, TError> ISinkMap<'a, TInput, TResult, TError, T> for T where
    Self: ISink<TInput=TInput, TResult=TResult, TError=TError>,
    T: ISink<TInput=TInput, TResult=TResult, TError=TError>,
{
    fn map<UInput, F: Fn(UInput) -> TInput + 'a>(self, map: F) -> SinkMap<'a, TInput, UInput, TResult, TError, T> {
        SinkMap::new(self, map)
    }
}

#[cfg(test)]
mod sink_map_tests {
    use super::*;

    #[test]
    fn should_explicitly_construct_a_sinkmap() {

        let s = Sink::<usize, usize, ()>::new(| item | {
            Ok(item)
        });

        let sm = SinkMap::new(s, | item: String | { item.len() });

        assert_eq!(Ok (0), sm.send("".to_owned()));
        assert_eq!(Ok (9), sm.send("some text".to_owned()));
    }

    #[test]
    fn should_construct_a_sinkmap_through_the_map_function() {

        let s = Sink::<usize, usize, ()>::new(| item | {
            Ok(item)
        }).map(| item: String | { item.len() });

        assert_eq!(Ok (0), s.send("".to_owned()));
        assert_eq!(Ok (9), s.send("some text".to_owned()));
    }
}