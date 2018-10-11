use lib::core::marker::PhantomData;
use sink::*;

#[derive(Debug)]
pub struct IdentitySystemHandle<T> {
    _t: PhantomData<T>,
    pub system: IdentitySystem<T>,
}

impl<T> IdentitySystemHandle<T> {
    pub fn new(system: IdentitySystem<T>) -> Self {
        IdentitySystemHandle {
            _t: PhantomData,
            system,
        }
    }

    pub fn shutdown(self) {
        println!("Terminated Identity System");
    }
}

#[derive(Debug)]
pub struct IdentitySystem<T> {
    _t: PhantomData<T>,
}

impl<T> IdentitySystem<T> {
    pub fn new() -> Self {
        IdentitySystem {
            _t: PhantomData,
        }
    }
}

impl<T> ISystem for IdentitySystem<T> {
    type TContext = ();
    type THandle = IdentitySystemHandle<T>;

    fn start(self, ctx: Self::TContext) -> Self::THandle {
        println!("Started Identity System");
        IdentitySystemHandle::new(self)
    }
}

impl<T> ISink for IdentitySystem<T> {
    type TInput = T;
    type TResult = T;

    fn handle(&self, input: Self::TInput) -> Self::TResult {
        println!("Identity System Got Input");
        // TODO: Send to source
        input
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn make_simple_system() {
        let i = IdentitySystem::new();

        println!("Identity: {:?}", i);

        let handle = i.start(());

        let result = handle.system.handle(10);

        println!("Result: {:?}", result);

        handle.shutdown();

        

        assert!(false);
    }
}