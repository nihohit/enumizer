/// Creates a Result-like enum with custom variant names.
///
/// See [`examples::ResultExample`](crate::examples::ResultExample) for a generated example.
///
/// # Example
///
/// ```
/// use enumizer::alias_result;
///
/// alias_result!(Response, Success, Failure);
///
/// let success: Response<i32, String> = Response::Success(42);
/// let error: Response<i32, String> = Response::Failure("failed".to_string());
///
/// assert!(success.is_success());
/// assert!(error.is_failure());
/// assert_eq!(success.as_success(), Some(&42));
/// ```
///
/// # Generated Methods
///
/// ```
/// use enumizer::alias_result;
/// alias_result!(Response, Success, Failure);
/// let mut val: Response<i32, String> = Response::Success(10);
///
/// assert!(val.is_success());
/// assert_eq!(val.as_success(), Some(&10));
/// assert_eq!(val.as_failure(), None);
///
/// let doubled = val.map(|x| x * 2);
/// assert_eq!(doubled.unwrap(), 20);
/// ```
///
/// # Conversions
///
/// The generated type can be easily converted to and from `Result<T>`.
///
/// ```
/// use enumizer::alias_result;
/// alias_result!(Response, Success, Failure);
/// let from_ok: Response<i32, String> = Ok(42).into();
/// let from_err: Response<i32, String> = Err("failed".to_string()).into();
///
/// assert_eq!(from_ok, Response::Success(42));
/// assert!(from_err.is_failure());
///
/// let to_result: Result<i32, String> = Response::Success(42).into();
/// assert_eq!(to_result, Ok(42));
/// ```
///
/// # Try Trait Support (Nightly Only)
///
/// Add `implement_try` to enable the `?` operator for early returns.
/// Requires nightly Rust with `#![feature(try_trait_v2)]`.
///
/// ```ignore
/// #![feature(try_trait_v2)]
/// use enumizer::alias_result;
///
/// alias_result!(Response, Success, Failure, implement_try);
///
/// fn try_example(res1: Response<i32, String>, res2: Response<i32, String>) -> Response<i32, String> {
///     let x = res1?;
///     let y = res2?;
///     Response::Success(x * y)
/// }
///
/// assert_eq!(try_example(Response::Success(5), Response::Success(15)), Response::Success(75));
/// assert_eq!(try_example(Response::Failure("error".into()), Response::Success(15)), Response::Failure("error".into()));
/// ```
///
/// # Custom Traits
///
/// You can specify custom traits to derive instead of the default set.
/// Use the `traits:` keyword followed by a list of trait names in brackets.
///
/// ```
/// use enumizer::alias_result;
/// alias_result!(CustomResult, Ok, Err, traits: [Debug, Clone, serde::Serialize, serde::Deserialize]);
/// let val: CustomResult<i32, String> = CustomResult::Ok(42);
/// assert_eq!(format!("{:?}", val), "Ok(42)");
/// assert_eq!(val.clone().unwrap(), 42);
/// let json = serde_json::to_string(&val).unwrap();
/// assert_eq!(json, r#"{"Ok":42}"#);
/// ```
#[macro_export]
macro_rules! alias_result {
    ($type_name:ident, $ok_variant:ident, $err_variant:ident) => {
        $crate::alias_result!($type_name, $ok_variant, $err_variant, [Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash]);
    };
    ($type_name:ident, $ok_variant:ident, $err_variant:ident, traits: [$($trait:path),*]) => {
        $crate::alias_result!($type_name, $ok_variant, $err_variant, [$($trait),*]);
    };
    ($type_name:ident, $ok_variant:ident, $err_variant:ident, implement_try) => {
        $crate::alias_result!($type_name, $ok_variant, $err_variant, [Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash], implement_try);
    };
    ($type_name:ident, $ok_variant:ident, $err_variant:ident, traits: [$($trait:path),*], implement_try) => {
        $crate::alias_result!($type_name, $ok_variant, $err_variant, [$($trait),*], implement_try);
    };
    ($type_name:ident, $ok_variant:ident, $err_variant:ident, [$($trait:path),*]) => {
        $crate::alias_result!($type_name, $ok_variant, $err_variant, [$($trait),*], );
    };
    ($type_name:ident, $ok_variant:ident, $err_variant:ident, [$($trait:path),*], $($implement_try:ident)?) => {
        paste::paste! {
        #[derive($($trait),*)]
        pub enum $type_name<T, E> {
            $ok_variant(T),
            $err_variant(E),
        }

        impl<T, E> $type_name<T, E> {
            /// Behaves like [`Result::is_ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok)
            pub fn [<is_ $ok_variant:lower>](&self) -> bool {
                matches!(self, $type_name::$ok_variant(_))
            }

            /// Behaves like [`Result::is_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err)
            pub fn [<is_ $err_variant:lower>](&self) -> bool {
                matches!(self, $type_name::$err_variant(_))
            }

            /// Behaves like [`Result::as_ref`](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_ref) for the Ok variant
            pub fn [<as_ $ok_variant:lower>](&self) -> Option<&T> {
                match self {
                    $type_name::$ok_variant(v) => Some(v),
                    _ => None,
                }
            }

            /// Behaves like [`Result::as_mut`](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_mut) for the Ok variant
            pub fn [<as_ $ok_variant:lower _mut>](&mut self) -> Option<&mut T> {
                match self {
                    $type_name::$ok_variant(v) => Some(v),
                    _ => None,
                }
            }

            /// Behaves like [`Result::as_ref`](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_ref) for the Err variant
            pub fn [<as_ $err_variant:lower>](&self) -> Option<&E> {
                match self {
                    $type_name::$err_variant(e) => Some(e),
                    _ => None,
                }
            }

            /// Behaves like [`Result::as_mut`](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_mut) for the Err variant
            pub fn [<as_ $err_variant:lower _mut>](&mut self) -> Option<&mut E> {
                match self {
                    $type_name::$err_variant(e) => Some(e),
                    _ => None,
                }
            }

            /// Behaves like [`Result::map`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map)
            pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> $type_name<U, E> {
                match self {
                    $type_name::$ok_variant(v) => $type_name::$ok_variant(f(v)),
                    $type_name::$err_variant(e) => $type_name::$err_variant(e),
                }
            }

            /// Behaves like [`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)
            pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> $type_name<T, F> {
                match self {
                    $type_name::$ok_variant(v) => $type_name::$ok_variant(v),
                    $type_name::$err_variant(e) => $type_name::$err_variant(op(e)),
                }
            }

            /// Behaves like [`Result::unwrap`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap)
            pub fn unwrap(self) -> T {
                match self {
                    $type_name::$ok_variant(v) => v,
                    $type_name::$err_variant(_) => {
                        panic!("called `unwrap()` on an `{}`", stringify!($err_variant))
                    }
                }
            }

            /// Behaves like [`Result::unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or)
            pub fn unwrap_or(self, default: T) -> T {
                match self {
                    $type_name::$ok_variant(v) => v,
                    $type_name::$err_variant(_) => default,
                }
            }

            /// Behaves like [`Result::unwrap_or_else`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else)
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

        $(
            let _ = stringify!($implement_try);
            paste::paste! {
                impl<T, E> std::ops::Try for $type_name<T, E> {
                    type Output = T;
                    type Residual = $type_name<std::convert::Infallible, E>;

                    fn from_output(output: Self::Output) -> Self {
                        $type_name::$ok_variant(output)
                    }

                    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
                        match self {
                            $type_name::$ok_variant(v) => std::ops::ControlFlow::Continue(v),
                            $type_name::$err_variant(e) => std::ops::ControlFlow::Break($type_name::$err_variant(e)),
                        }
                    }
                }

                impl<T, E> std::ops::FromResidual for $type_name<T, E> {
                    fn from_residual(residual: $type_name<std::convert::Infallible, E>) -> Self {
                        match residual {
                            $type_name::$err_variant(e) => $type_name::$err_variant(e),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        )?
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
