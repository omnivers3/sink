/// ISource
pub trait ISource {
    type TOutput;
    type THandle = Self;

    fn bind(self, sink: impl ISink) -> Self::THandle;
}