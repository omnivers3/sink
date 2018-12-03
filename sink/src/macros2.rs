
#[macro_export(local_inner_macros)]
macro_rules! _ctx_struct {
    // Completed macro accumulation
    (@struct $_index:expr, () -> {$(($index:expr, $name:ident | $input:ty | $handler:expr))*}) => {
        struct Context<'a> {
            source: &'a Source<TOutput=()>,
            $($name: &'a Sink<TInput=$input, TResult=()>),*
        }

        impl<'a> Context<'a> {
            pub fn new(source: &'a Source<TOutput=()>, $($name: &'a Sink<TInput=$input, TResult=()>),*) -> Self {
                Context {
                    source,
                    $($name),*
                }
            }
        }

        impl<'a> Source for Context<'a> {
            type TOutput = ();

            fn next(&self) -> Self::TOutput {
                self.source.next()
            }
        }
    };

    // Last element in the recursion
    (@struct $index:expr, ($name:ident: $input:ty = $handler:expr) -> {$($output:tt)*}) => {
        _ctx_struct!(@struct $index + 1usize, () -> {$($output)* ($index, $name | $input | $handler)})
    };

    // Element with subsequent elements
    (@struct $index:expr, ($name:ident: $input:ty = $handler:expr, $($next:tt)*) -> {$($output:tt)*}) => {
        _ctx_struct!(@struct $index + 1usize, ($($next)*) -> {$($output)* ($index, $name | $input | $handler)})
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
    // Context has a non unit source
    (source: $source_ty:ty, $($input:tt)*) => {
        _ctx_struct!(@struct 0usize, ($($input)*) -> $source_ty {});
        _ctx_struct!(@disp 0usize, ($($input)*));
    };

    // Default context to a unit source
    ($($input:tt)*) => {
        _ctx_struct!(@struct 0usize, ($($input)*) -> {});
        _ctx_struct!(@disp 0usize, ($($input)*));
    };
}

#[macro_export(local_inner_macros)]
macro_rules! _ctx {
    (@ctx $_index:expr, () -> {$(($index:expr, $name:ident | $input:ty | $handler:expr))*}) => {
        Context::new(source: UnitSource::new(), $(&$handler),*)
    };

    (@ctx $index:expr, ($name:ident: $input:ty = $handler:expr) -> {$($output:tt)*}) => {
        _ctx!(@ctx $index + 1usize, () -> {$($output)* ($index, $name | $input | $handler)})
    };

    (@ctx $index:expr, ($name:ident: $input:ty = $handler:expr, $($next:tt)*) -> {$($output:tt)*}) => {
        _ctx!(@ctx $index + 1usize, ($($next)*) -> {$($output)* ($index, $name | $input | $handler)})
    };
}

#[macro_export(local_inner_macros)]
macro_rules! ctx {
    // Context has a non unit source
    (source: $source_ty:ty = $source:expr, $($input:tt)*) => {
        {
            ctx_struct!($source_ty $($input)*);
            _ctx!(@ctx 0usize, ($($input)*) -> {})
        }
    };

    // Default context to a unit source
    ($($input:tt)*) => {
        {
            ctx_struct!($($input)*);
            _ctx!(@ctx 0usize, ($($input)*) -> {})
        }
    };
}