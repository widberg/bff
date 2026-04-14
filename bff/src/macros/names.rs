macro_rules! names {
    (
        styles: [$($style:ident($style_transform:expr)),* $(,)?],
        names: [
            $($name:ident($name_style:ident, $name_hash_function:ty)),* $(,)?
        ]
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum NameStyle {
            $($style,)*
        }

        pub const fn name_style_transform(style: NameStyle) -> fn(&str) -> String {
            match style {
                $(NameStyle::$style => $style_transform,)*
            }
        }

        #[inline]
        pub fn apply_name_style<S: AsRef<str>>(string: S, style: NameStyle) -> String {
            name_style_transform(style)(string.as_ref())
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum NameType {
            $($name,)*
        }
    };
}

pub(crate) use names;
