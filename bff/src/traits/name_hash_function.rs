use super::NameDisplay;
use super::name_target::NameTarget;

pub trait NameHashFunction {
    type Target: NameTarget;
    type Display: NameDisplay<Self::Target>;

    fn hash(bytes: &[u8]) -> Self::Target;

    fn display_from_target(target: Self::Target) -> Self::Display {
        <Self::Display as NameDisplay<Self::Target>>::from_target(target)
    }

    fn target_from_display(display: Self::Display) -> Self::Target {
        <Self::Display as NameDisplay<Self::Target>>::into_target(display)
    }

    fn parse_display(string: &str) -> Option<Self::Target> {
        string
            .parse::<Self::Display>()
            .ok()
            .map(Self::target_from_display)
    }
}
