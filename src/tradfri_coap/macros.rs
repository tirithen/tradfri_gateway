#[macro_export]
macro_rules! impl_from {
    ($t:ty) => {
        impl From<$t> for super::Error {
            fn from(err: $t) -> super::Error {
                super::Error {
                    cause: format!("{}", err),
                }
            }
        }
    };
}
