use lib::core::marker::PhantomData;

use super::*;

#[derive(Clone)]
pub enum AsyncStatus<TResult> {
    Pending,
    Ready(TResult),
}

pub trait IAsyncContext {
    type TResult;

    fn poll(self) -> AsyncStatus<Self::TResult>;
}

pub trait IAsyncSink<'a> {
    type TInput;
    type TResult;

    // fn handle(&'a self, input: Self::TInput) -> &'a IAsyncContext<TResult=Self::TResult>;
    fn send(&self, input: Self::TInput) -> &'a IAsyncContext<TResult=Self::TResult>;
}

pub struct Immediate<TInput, TResult>
where
    TResult: Clone,
{
    result: TResult,
    _input: PhantomData<TInput>,
}

impl<TInput, TResult> Immediate<TInput, TResult>
where
    TResult: Clone,
{
    pub fn new(result: TResult) -> Self {
        Immediate {
            result,
            _input: PhantomData,
        }
    }
}

impl<TInput, TResult> IAsyncContext for Immediate<TInput, TResult>
where
    TResult: Clone,
{
    type TResult = TResult;

    fn poll(self) -> AsyncStatus<TResult> {
        AsyncStatus::Ready(self.result.clone())
    }
}

impl<'a, TInput, TResult> IAsyncSink<'a> for Immediate<TInput, TResult>
where
    TResult: Clone,
{
    type TInput = TInput;
    type TResult = TResult;

    fn send(&self, input: Self::TInput) -> &'a IAsyncContext<TResult=TResult> {
    // fn handle(&self, input: Self::TInput) -> Immediate<TInput, TResult> {
        // (*self as IAsyncContext)
        self
    }
}

pub struct AsyncSink {
    target: TSink,
}

pub trait IntoAsync {
    type TInput;
    type TResult;

    // fn async<T>(self, IAsyncSink<TInput=Self::TInput, TResult=Self::TResult>) -> T where T: &'a IAsyncSink<TInput=Self::TInput, TResult=Self::TResult> + Default;
    fn async<T>(self, IAsyncSink<TInput=Self::TInput, TResult=Self::TResult>) -> AsyncSink;
}

impl<TSink, TInput, TResult> IntoAsync for TSink
where
    Self: Sized,
    TSink: ISink<TInput=TInput, TResult=TResult>,
{
    type TInput = TInput;
    type TResult = TResult;

    fn async<T>(self, ) -> T
    where
        T: IAsyncSink<TInput=TInput, TResult=TResult> + Default
    {
        T::default()
    }
}

#[cfg(test)]
mod asyncsink_tests {
    use super::*;
    use super::sink::*;

    #[test]
    fn should_wrap_sink_with_async() {
        let world = World::
        let s = Sink::new(|_: ()| ());
        let a = s.async(Immediate::new(10));
        let ctx = a.send(());
        let r = ctx.poll();

        // let s: Immediate<_, _> = s.async();
        // let async_context = s.handle(());
        // let r = async_context.poll();
        // let r = a.poll();

        // println!("{:?}", r);

        assert!(false);

        // let _s: Immediate<_, _> = s.async();
        // let _s = _s.async(|item| Immediate::new(()));

        // let _s = _s.async(|item| {});
    }
}

// impl<'a, TInput, TResult, TAsyncContext> IAsyncSink<'a> for Immediate<TInput, TResult>
// where
//     TAsyncContext: IAsyncContext<'a, TResult=TResult> + Sized,
// {
//     type TInput = TInput;
//     type TResult = TResult;

//     // fn handle(&self, input: Self::TInput) -> &'a IAsyncContext<'a, TResult=TResult> {
//     fn handle(&self, input: Self::TInput) -> TAsyncContext {
//         // Immediate::new(self.value)
//         self.clone()
//     }
// }

// pub struct AsyncSink<'a, FContext, TSink, TInput, TResult>
// where
//     FContext: Fn(TInput) -> impl IAsyncContext<'a, TResult=TResult>,
// {
//     target: TSink,
//     ctx_factory: FContext,
//     _input: PhantomData<TInput>,
// }

// pub trait IAsyncHandle<T> {
//     fn poll(&self) -> AsyncStatus<T>;
// }

// pub trait IAsyncSink {
//     type TInput;
//     type TResult;

//     fn handle(&self, event: Self::TInput) -> IAsyncHandle<Self::TResult>;
// }

// impl<T, TInput, TResult> ISink for T
// where
//     T: IAsyncSink<TInput=TInput, TResult=TResult>,
// {
//     type TInput = TInput;
//     type TResult = &IAsyncHandle<TResult>;

//     fn handle(&self, event: TInput) -> &IAsyncHandle<TResult> {
//         (self as &IAsyncSink<TInput=TInput, TResult=TResult>).handle(event)
//     }
// }
