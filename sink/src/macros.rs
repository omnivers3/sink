
// TODO: Remvoe names from sinks as duplicate types would break dispatcher side anyhow

#[macro_export(local_inner_macros)]
macro_rules! _ctx_struct {
    // Completed macro accumulation - No source specified
    (@struct $_index:expr, () -> {()} {$(($index:expr, $name:ident | $input:ty | $handler:expr))*}) => {
        pub use sink::{ Dispatcher, Sink, Source };

        struct Context<'a> {
            $($name: &'a Sink<TInput=$input, TResult=()>),*
        }

        impl<'a> Context<'a> {
            pub fn new($($name: &'a Sink<TInput=$input, TResult=()>),*) -> Self {
                Context {
                    $($name),*
                }
            }
        }
    };

    // Completed macro accumulation - With source
    (@struct $_index:expr, () -> {($source_ty:ty | $source_handler:expr)} {$(($index:expr, $name:ident | $input:ty | $handler:expr))*}) => {
        pub use sink::{ Dispatcher, Sink, Source };

        struct Context<'a> {
            source: &'a Source<TOutput=$source_ty>,
            $($name: &'a Sink<TInput=$input, TResult=()>),*
        }

        impl<'a> Context<'a> {
            pub fn new(source: &'a Source<TOutput=$source_ty>, $($name: &'a Sink<TInput=$input, TResult=()>),*) -> Self {
                Context {
                    source,
                    $($name),*
                }
            }
        }

        impl<'a> Source for Context<'a> {
            type TOutput = $source_ty;

            fn next(&self) -> Self::TOutput {
                self.source.next()
            }
        }
    };

    // Last element in the recursion - Source
    (@struct $index:expr, (source: $input:ty = $handler:expr) -> {$source:tt} {$($output:tt)*}) => {
        _ctx_struct!(@struct $index, () -> {($input | $handler)} {$($output)*})
    };

    // Element with subsequent elements - Source
    (@struct $index:expr, (source: $input:ty = $handler:expr, $($next:tt)*) -> {$source:tt} {$($output:tt)*}) => {
        _ctx_struct!(@struct $index, ($($next)*) -> {($input | $handler)} {$($output)*})
    };

    // Last element in the recursion
    (@struct $index:expr, ($name:ident: $input:ty = $handler:expr) -> {$source:tt} {$($output:tt)*}) => {
        _ctx_struct!(@struct $index + 1usize, () -> {$source} {$($output)* ($index, $name | $input | $handler)})
    };

    // Element with subsequent elements
    (@struct $index:expr, ($name:ident: $input:ty = $handler:expr, $($next:tt)*) -> {$source:tt} {$($output:tt)*}) => {
        _ctx_struct!(@struct $index + 1usize, ($($next)*) -> {$source} {$($output)* ($index, $name | $input | $handler)})
    };

    // Expand to a dispatcher trait impl
    (@item $index:expr, $name:ident | $input:ty = $handler:expr) => {{
        impl<'a> Dispatcher<$input> for Context<'a> {
            fn dispatch(&self, input: $input) {
                // println!("Dispatcher[{:?} | {:?}]: {:?}", $index, stringify!($name), input);
                self.$name.send(input)
            }
        }
    }};

    // Fall out of macro recursion
    (@disp $_index:expr, ()) => {};

    // Last element in the recursion - Source
    (@disp $index:expr, (source: $input:ty = $handler:expr)) => {
        _ctx_struct!(@disp $index, ())
    };

    // Element with subsequent elements - Source
    (@disp $index:expr, (source: $input:ty = $handler:expr, $($next:tt)*)) => {
        _ctx_struct!(@disp $index, ($($next)*))
    };

    // Last element in the recursion
    (@disp $index:expr, ($name:ident: $input:ty = $handler:expr)) => {
        _ctx_struct!(@item $index, $name | $input = $handler);
        _ctx_struct!(@disp $index + 1usize, ())
    };

    // Element with subsequent elements
    (@disp $index:expr, ($name:ident: $input:ty = $handler:expr, $($next:tt)*)) => {
        _ctx_struct!(@item $index, $name | $input = $handler);
        _ctx_struct!(@disp $index + 1usize, ($($next)*))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! ctx_struct {
    ($($input:tt)*) => {
        _ctx_struct!(@struct 0usize, ($($input)*) -> {()} {});
        _ctx_struct!(@disp 0usize, ($($input)*));
    };
}

#[macro_export(local_inner_macros)]
macro_rules! _ctx {
    (@ctx $_index:expr, () -> {()} {$(($index:expr, $name:ident | $input:ty | $handler:expr))*}) => {
        Context::new($(&$handler),*)
    };

    (@ctx $_index:expr, () -> {$source:tt} {$(($index:expr, $name:ident | $input:ty | $handler:expr))*}) => {
        Context::new(&$source, $(&$handler),*)
    };

    // Last element in the recursion - Source
    (@ctx $index:expr, (source: $input:ty = $handler:expr) -> {$source:tt} {$($output:tt)*}) => {
        _ctx!(@ctx $index, () -> {$handler} {$($output)*})
    };

    // Element with subsequent elements - Source
    (@ctx $index:expr, (source: $input:ty = $handler:expr, $($next:tt)*) -> {$source:tt} {$($output:tt)*}) => {
        _ctx!(@ctx $index, ($($next)*) -> {$handler} {$($output)*})
    };

    // Last element in the recursion
    (@ctx $index:expr, ($name:ident: $input:ty = $handler:expr) -> {$source:tt} {$($output:tt)*}) => {
        _ctx!(@ctx $index + 1usize, () -> {$source} {$($output)* ($index, $name | $input | $handler)})
    };

    // Element with subsequent elements
    (@ctx $index:expr, ($name:ident: $input:ty = $handler:expr, $($next:tt)*) -> {$source:tt} {$($output:tt)*}) => {
        _ctx!(@ctx $index + 1usize, ($($next)*) -> {$source} {$($output)* ($index, $name | $input | $handler)})
    };
}

#[macro_export(local_inner_macros)]
macro_rules! ctx {
    ($($input:tt)*) => {
        {
            ctx_struct!($($input)*);
            _ctx!(@ctx 0usize, ($($input)*) -> {()} {})
        }
    };
}