macro_rules! trivial_class_generic {
    ($class:ident, $generic:ident) => {
        impl From<$class> for $generic {
            fn from(class: $class) -> $generic {
                $generic {
                    class_name: class.class_name,
                    name: class.name,
                    link_name: class.link_name,
                    link_header: class.link_header.into(),
                    body: class.body.into(),
                }
            }
        }

        impl crate::traits::TryFromGenericSubstitute<$generic, $class> for $class {
            type Error = crate::error::Error;

            fn try_from_generic_substitute(
                generic: $generic,
                substitute: $class,
            ) -> crate::BffResult<$class> {
                use crate::traits::TryIntoSpecific;
                Ok($class {
                    class_name: generic.class_name,
                    name: generic.name,
                    link_name: generic.link_name,
                    link_header: generic
                        .link_header
                        .try_into_specific(substitute.link_header)?,
                    body: generic.body.try_into_specific(substitute.body)?,
                })
            }
        }
    };
}

pub(crate) use trivial_class_generic;
