// this should be const https://github.com/rust-lang/rust/issues/67792
pub trait NameHashFunction {
    type Target;
    fn hash(bytes: &[u8]) -> Self::Target;
    fn hash_options(bytes: &[u8], starting: Self::Target) -> Self::Target;
}
