use super::sink::*;

pub enum Sink2Signal<TSink0, TSink1>
where
    TSink0: Sink,
    TSink1: Sink,
{
    Sink0 (TSink0::TInput),
    Sink1 (TSink1::TInput),
}

pub enum Sink2Result<TSink0, TSink1>
where
    TSink0: Sink,
    TSink1: Sink,
{
    Sink0 (TSink0::TResult),
    Sink1 (TSink1::TResult),
}

pub struct Sink2<'a, 'b, TSink0, TSink1>(pub &'a TSink0, pub &'b TSink1);

// impl<'a, 'b, TSink0, TSink1, TTarget> From<Sink2<'b, 'a, TSink1, TSink0>> for TTarget {
//     fn from(source: Sink2<'b, 'a, TSink1, TSink0>) -> Sink2<'a, 'b, TSink0, TSink1> {
//         let (sink1, sink0) = source;
//         Sink2(sink0, sink1)
//     }
// }

impl<'a, 'b, TSink0, TSink1> Sink2<'a, 'b, TSink0, TSink1> {
    pub fn new(sink0: &'a TSink0, sink1: &'b TSink1) -> Self {
        Sink2 (sink0, sink1)
    }

    pub fn spread(&self) -> (&TSink0, &TSink1) {
        (self.0, self.1)
    }

    pub fn swap(&self) -> Sink2<'b, 'a, TSink1, TSink0> {
        Sink2::new(self.1, self.0)
    }
}

impl<'a, 'b, TSink0, TSink1> Sink for Sink2<'a, 'b, TSink0, TSink1>
where
    TSink0: Sink,
    TSink1: Sink,
{
    type TInput = Sink2Signal<TSink0, TSink1>;
    type TResult = Sink2Result<TSink0, TSink1>;

    fn send(&self, input: Sink2Signal<TSink0, TSink1>) -> Self::TResult {
        match input {
            Sink2Signal::Sink0 (input) => Sink2Result::Sink0(self.0.send(input)),
            Sink2Signal::Sink1 (input) => Sink2Result::Sink1(self.1.send(input)), 
        }
    }
}