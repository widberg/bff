pub trait TryFromGenericSubstitute<T, U>: Sized {
    type Error;
    fn try_from_generic_substitute(value: T, substitute: U) -> Result<Self, Self::Error>;
}
