//! Creates a Result-like enum with custom variant names.
//!
//! # Example
//!
//! ```
//! use alias_option_macro::alias_result;
//!
//! alias_result!(Response, Success, Failure);
//!
//! let success: Response<i32, String> = Response::Success(42);
//! let error: Response<i32, String> = Response::Failure("failed".to_string());
//!
//! assert!(success.is_success());
//! assert!(error.is_failure());
//! assert_eq!(success.as_success(), Some(&42));
//! ```
//!
//! # Generated Methods
//!
//! ```
//! # use alias_option_macro::alias_result;
//! # alias_result!(Response, Success, Failure);
//! let mut val: Response<i32, String> = Response::Success(10);
//!
//! assert!(val.is_success());
//! assert_eq!(val.as_success(), Some(&10));
//! assert_eq!(val.as_failure(), None);
//!
//! let doubled = val.map(|x| x * 2);
//! assert_eq!(doubled.unwrap(), 20);
//! ```
//!
//! # Conversions
//!
//! ```
//! # use alias_option_macro::alias_result;
//! # alias_result!(Response, Success, Failure);
//! let from_ok: Response<i32, String> = Ok(42).into();
//! let from_err: Response<i32, String> = Err("failed".to_string()).into();
//!
//! assert_eq!(from_ok, Response::Success(42));
//! assert!(from_err.is_failure());
//!
//! let to_result: Result<i32, String> = Response::Success(42).into();
//! assert_eq!(to_result, Ok(42));
//! ```
#[macro_export]
macro_rules! alias_result {
    ($type_name:ident, $ok_variant:ident, $err_variant:ident) => {
        paste::paste! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $type_name<T, E> {
            $ok_variant(T),
            $err_variant(E),
        }

        impl<T, E> $type_name<T, E> {
            pub fn [<is_ $ok_variant:lower>](&self) -> bool {
                matches!(self, $type_name::$ok_variant(_))
            }

            pub fn [<is_ $err_variant:lower>](&self) -> bool {
                matches!(self, $type_name::$err_variant(_))
            }

            pub fn [<as_ $ok_variant:lower>](&self) -> Option<&T> {
                match self {
                    $type_name::$ok_variant(v) => Some(v),
                    _ => None,
                }
            }

            pub fn [<as_ $ok_variant:lower _mut>](&mut self) -> Option<&mut T> {
                match self {
                    $type_name::$ok_variant(v) => Some(v),
                    _ => None,
                }
            }

            pub fn [<as_ $err_variant:lower>](&self) -> Option<&E> {
                match self {
                    $type_name::$err_variant(e) => Some(e),
                    _ => None,
                }
            }

            pub fn [<as_ $err_variant:lower _mut>](&mut self) -> Option<&mut E> {
                match self {
                    $type_name::$err_variant(e) => Some(e),
                    _ => None,
                }
            }

            pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> $type_name<U, E> {
                match self {
                    $type_name::$ok_variant(v) => $type_name::$ok_variant(f(v)),
                    $type_name::$err_variant(e) => $type_name::$err_variant(e),
                }
            }

            pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> $type_name<T, F> {
                match self {
                    $type_name::$ok_variant(v) => $type_name::$ok_variant(v),
                    $type_name::$err_variant(e) => $type_name::$err_variant(op(e)),
                }
            }

            pub fn unwrap(self) -> T {
                match self {
                    $type_name::$ok_variant(v) => v,
                    $type_name::$err_variant(_) => {
                        panic!("called `unwrap()` on an `{}`", stringify!($err_variant))
                    }
                }
            }

            pub fn unwrap_or(self, default: T) -> T {
                match self {
                    $type_name::$ok_variant(v) => v,
                    $type_name::$err_variant(_) => default,
                }
            }

            pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
                match self {
                    $type_name::$ok_variant(v) => v,
                    $type_name::$err_variant(e) => op(e),
                }
            }
        }

        impl<T, E> From<Result<T, E>> for $type_name<T, E> {
            fn from(result: Result<T, E>) -> Self {
                match result {
                    Ok(v) => $type_name::$ok_variant(v),
                    Err(e) => $type_name::$err_variant(e),
                }
            }
        }

        impl<T, E> From<$type_name<T, E>> for Result<T, E> {
            fn from(val: $type_name<T, E>) -> Self {
                match val {
                    $type_name::$ok_variant(v) => Ok(v),
                    $type_name::$err_variant(e) => Err(e),
                }
            }
        }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn size_equivalence() {
        use std::num::{NonZeroU32, NonZeroU64};

        alias_result!(Response, Success, Failure);

        assert_eq!(
            std::mem::size_of::<Response<NonZeroU32, String>>(),
            std::mem::size_of::<Result<NonZeroU32, String>>()
        );
        assert_eq!(
            std::mem::size_of::<Response<NonZeroU32, NonZeroU64>>(),
            std::mem::size_of::<Result<NonZeroU32, NonZeroU64>>()
        );
        assert_eq!(
            std::mem::size_of::<Response<NonZeroU32, i32>>(),
            std::mem::size_of::<Result<NonZeroU32, i32>>()
        );
    }
}
