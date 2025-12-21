/// Creates an Option-like enum with custom variant names.
///
/// See [`examples::OptionExample`](crate::examples::OptionExample) for a generated example.
///
/// # Example
///
/// ```
/// use enumizer::alias_option;
///
/// alias_option!(Value, Found, Searching);
///
/// let searching: Value<i32> = Value::Searching;
/// let found = Value::Found(42);
///
/// assert!(searching.is_searching());
/// assert!(!searching.is_found());
/// assert!(found.is_found());
/// assert_eq!(found.as_found(), Some(&42));
/// ```
///
/// # Generated Methods
///
/// ```
/// use enumizer::alias_option;
/// alias_option!(Value, Found, Searching);
/// let mut val = Value::Found(10);
///
/// // Check variants
/// assert!(val.is_found());
///
/// // Get references
/// assert_eq!(val.as_found(), Some(&10));
/// assert_eq!(val.as_found_mut(), Some(&mut 10));
///
/// // Transform
/// let doubled = val.map(|x| x * 2);
/// assert_eq!(doubled.unwrap(), 20);
///
/// // Unwrap variants
/// assert_eq!(Value::Found(5).unwrap_or(0), 5);
/// assert_eq!(Value::<i32>::Searching.unwrap_or(5), 5);
/// ```
///
/// # Conversions
///
/// The generated type can be easily converted to and from `Option<T>`.
///
/// ```
/// use enumizer::alias_option;
/// alias_option!(Value, Found, Searching);
/// let from_some: Value<i32> = Some(42).into();
/// let from_none: Value<i32> = None.into();
///
/// assert_eq!(from_some, Value::Found(42));
/// assert_eq!(from_none, Value::Searching);
///
/// let to_option: Option<i32> = Value::Found(42).into();
/// assert_eq!(to_option, Some(42));
/// ```
///
/// # Conditional Checks
///
/// ```
/// use enumizer::alias_option;
/// alias_option!(Value, Found, Searching);
/// let val = Value::Found(42);
/// assert!(val.is_found_and(|&x| x > 40));
/// assert!(!val.is_found_and(|&x| x < 40));
/// assert!(!Value::<i32>::Searching.is_found_and(|&x| x > 40));
///
/// assert!(Value::<i32>::Searching.is_searching_or(|&x| x > 40));
/// assert!(val.is_searching_or(|&x| x > 40));
/// assert!(!val.is_searching_or(|&x| x < 40));
/// ```
///
/// # Custom Traits
///
/// You can specify custom traits to derive instead of the default set.
/// Use the `traits:` keyword followed by a list of trait names in brackets.
///
/// ```
/// use enumizer::alias_option;
/// alias_option!(CustomOption, Present, Absent, traits: [Debug, Clone, serde::Serialize, serde::Deserialize]);
/// let val = CustomOption::Present(42);
/// assert_eq!(format!("{:?}", val), "Present(42)");
/// assert_eq!(val.clone().unwrap(), 42);
/// let json = serde_json::to_string(&val).unwrap();
/// assert_eq!(json, r#"{"Present":42}"#);
/// ```
///
/// # Try Trait Support (Nightly Only)
///
/// Add `implement_try` to enable the `?` operator for early returns.
/// Requires nightly Rust with `#![feature(try_trait_v2)]`.
///
/// ```ignore
/// #![feature(try_trait_v2)]
/// use enumizer::alias_option;
///
/// alias_option!(Value, Found, Searching, implement_try);
///
/// fn try_example(val1: Value<i32>, val2: Value<i32>) -> Value<i32> {
///     let x = val1?;
///     let y = val2?;
///     Value::Found(x + y)
/// }
///
/// assert_eq!(try_example(Value::Found(10), Value::Found(20)), Value::Found(30));
/// assert_eq!(try_example(Value::Searching, Value::Found(20)), Value::Searching);
/// ```

#[macro_export]
macro_rules! alias_option {
    ($type_name:ident, $some_variant:ident, $none_variant:ident) => {
        $crate::alias_option!($type_name, $some_variant, $none_variant, [Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash]);
    };
    ($type_name:ident, $some_variant:ident, $none_variant:ident, traits: [$($trait:path),*]) => {
        $crate::alias_option!($type_name, $some_variant, $none_variant, [$($trait),*]);
    };
    ($type_name:ident, $some_variant:ident, $none_variant:ident, implement_try) => {
        $crate::alias_option!($type_name, $some_variant, $none_variant, [Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash], implement_try);
    };
    ($type_name:ident, $some_variant:ident, $none_variant:ident, traits: [$($trait:path),*], implement_try) => {
        $crate::alias_option!($type_name, $some_variant, $none_variant, [$($trait),*], implement_try);
    };
    ($type_name:ident, $some_variant:ident, $none_variant:ident, [$($trait:path),*]) => {
        $crate::alias_option!($type_name, $some_variant, $none_variant, [$($trait),*], );
    };
    ($type_name:ident, $some_variant:ident, $none_variant:ident, [$($trait:path),*], $($implement_try:ident)?) => {
      paste::paste! {
		#[derive($($trait),*)]
		pub enum $type_name<T> {
			$none_variant,
			$some_variant(T),
		}

		impl<T> $type_name<T> {
			/// Behaves like [`Option::is_none`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none)
			pub fn [<is_ $none_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$none_variant)
			}

			/// Behaves like [`Option::is_some`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some)
			pub fn [<is_ $some_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$some_variant(_))
			}

			/// Behaves like [`Option::is_some_and`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some_and)
			pub fn [<is_ $some_variant:lower _and>]<F: FnOnce(&T) -> bool>(&self, f: F) -> bool {
				match self {
					$type_name::$some_variant(v) => f(v),
					_ => false,
				}
			}

			/// Behaves like [`Option::is_none_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none_or)
			pub fn [<is_ $none_variant:lower _or>]<F: FnOnce(&T) -> bool>(&self, f: F) -> bool {
				match self {
					$type_name::$none_variant => true,
					$type_name::$some_variant(v) => f(v),
				}
			}

			/// Behaves like [`Option::as_ref`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref)
			pub fn [<as_ $some_variant:lower>](&self) -> Option<&T> {
				match self {
					$type_name::$some_variant(v) => Some(v),
					_ => None,
				}
			}

			/// Behaves like [`Option::as_mut`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut)
			pub fn [<as_ $some_variant:lower _mut>](&mut self) -> Option<&mut T> {
				match self {
					$type_name::$some_variant(v) => Some(v),
					_ => None,
				}
			}

			/// Behaves like [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
			pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> $type_name<U> {
				match self {
					$type_name::$some_variant(v) => $type_name::$some_variant(f(v)),
					$type_name::$none_variant => $type_name::$none_variant,
				}
			}

			/// Behaves like [`Option::unwrap`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap)
			pub fn unwrap(self) -> T {
				match self {
					$type_name::$some_variant(v) => v,
					$type_name::$none_variant => {
						panic!("called `unwrap()` on a `{}`", stringify!($none_variant))
					}
				}
			}

			/// Behaves like [`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)
			pub fn unwrap_or(self, default: T) -> T {
				match self {
					$type_name::$some_variant(v) => v,
					$type_name::$none_variant => default,
				}
			}

			/// Behaves like [`Option::unwrap_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else)
			pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
				match self {
					$type_name::$some_variant(v) => v,
					$type_name::$none_variant => f(),
				}
			}
		}

		impl<T> From<Option<T>> for $type_name<T> {
			fn from(opt: Option<T>) -> Self {
				match opt {
					Some(v) => $type_name::$some_variant(v),
					None => $type_name::$none_variant,
				}
			}
		}

		impl<T> From<$type_name<T>> for Option<T> {
			fn from(val: $type_name<T>) -> Self {
				match val {
					$type_name::$some_variant(v) => Some(v),
					$type_name::$none_variant => None,
				}
			}
		}
        }

        $(
            let _ = stringify!($implement_try);
            paste::paste! {
                impl<T> std::ops::Try for $type_name<T> {
                    type Output = T;
                    type Residual = $type_name<std::convert::Infallible>;

                    fn from_output(output: Self::Output) -> Self {
                        $type_name::$some_variant(output)
                    }

                    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
                        match self {
                            $type_name::$some_variant(v) => std::ops::ControlFlow::Continue(v),
                            $type_name::$none_variant => std::ops::ControlFlow::Break($type_name::$none_variant),
                        }
                    }
                }

                impl<T> std::ops::FromResidual for $type_name<T> {
                    fn from_residual(_: $type_name<std::convert::Infallible>) -> Self {
                        $type_name::$none_variant
                    }
                }
            }
        )?
    }
  }

#[cfg(test)]
mod tests {
    #[test]
    fn size_equivalence() {
        use std::num::NonZeroU32;
        alias_option!(Value, Found, Searching);
        assert_eq!(
            std::mem::size_of::<Value<i32>>(),
            std::mem::size_of::<Option<i32>>()
        );
        assert_eq!(
            std::mem::size_of::<Value<NonZeroU32>>(),
            std::mem::size_of::<Option<NonZeroU32>>()
        );
    }
}
