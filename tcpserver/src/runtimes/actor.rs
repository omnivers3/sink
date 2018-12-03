
// pub struct Actor<'a, TContext> {
//     ctx: &'a TContext,
// }

// impl<'a, TContext> Actor<'a, TContext> {
//     pub fn new(ctx: &'a TContext) -> Self {
//         Actor {
//             ctx,
//         }
//     }
// }

// // pub trait ActorDef<TContext> {
// //     fn bind<'a>(self, ctx: &'a TContext) -> Actor<'a, TContext>;
// // }

// // impl<TContext> ActorDef<TContext> for StdinLineReader
// // where
// //     TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
// // {
// //     fn bind<'a>(self, ctx: &'a TContext) -> Actor<'a, TContext> {
// //         Actor::new(ctx)
// //     }
// // }

// pub trait ActorDef<TContext>
// where
//     Self: Sized,
// {
//     fn bind<'a>(self, ctx: &'a TContext) -> Actor<'a, TContext> {
//         Actor::new(ctx)
//     }

//     fn run<'a>(&self, ctx: &'a TContext);
// }

// impl<TContext> ActorDef<TContext> for StdinLineReader
// where
//     TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
// {
//     fn run<'a>(&self, ctx: &'a TContext) {
//         ctx.dispatch(trace!("blocking on stdin"));
//         ctx.dispatch(StdinEvents::Listening);
//         let lock = self.stdin.lock();
//         for line in lock.lines() {
//             match line {
//                 Err (err) => {
//                     ctx.dispatch(error!("error reading stdin: {:?}", err));
//                     break;
//                 }
//                 Ok (line) => {
//                     ctx.dispatch(trace!("received line [{:?}]", line));
//                     ctx.dispatch(StdinEvents::LineReceived (line));
//                 }
//             }
//         }
//         ctx.dispatch(StdinEvents::Paused);
//     }
// }
