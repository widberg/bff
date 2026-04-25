use super::name_target::NameTarget;

pub trait NameHashFunction {
    type Target: NameTarget;
    fn hash(bytes: &[u8]) -> Self::Target;
}
