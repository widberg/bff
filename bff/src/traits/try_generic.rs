/// Turns a generic class into a specific one by using the substitute's values
// this is just a worse version of tryintoversionplatform
pub trait TryIntoSpecific<T>: Sized {
    type Error;
    fn try_into_specific(self, substitute: T) -> Result<T, Self::Error>;
}

/// Creates a specific class by using the generic one as a base, replacing missing values with the ones from the substitute
pub trait TryFromGenericSubstitute<T, U>: Sized {
    type Error;
    fn try_from_generic_substitute(generic: T, substitute: U) -> Result<Self, Self::Error>;
}

impl<T, U> TryIntoSpecific<U> for T
where
    U: TryFromGenericSubstitute<T, U>,
{
    type Error = U::Error;

    #[inline]
    fn try_into_specific(self, substitute: U) -> Result<U, U::Error> {
        U::try_from_generic_substitute(self, substitute)
    }
}
