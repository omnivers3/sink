
    pub trait IPure {
        fn 
    }

    pub enum System<'a, TInput, TOutput> {
        Pure (&'a Fn(TInput) -> TOutput),
        Stateful (&'a FnMut())
    }

    pub struct Deterministic<TState> {
        state: TState,
    }

    pub struct NonDeterministic<TState> {
        state: RefCell<TState>,
    }