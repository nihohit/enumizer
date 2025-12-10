//! Creates an Option-like enum with custom variant names.
//!
//! # Example
//!
//! ```
//! use alias_option_macro::alias_option;
//!
//! alias_option!(Sampler, Leader, Receiver);
//!
//! let leader: Sampler<i32> = Sampler::Leader;
//! let receiver = Sampler::Receiver(42);
//!
//! assert!(leader.is_leader());
//! assert!(!leader.is_receiver());
//! assert!(receiver.is_receiver());
//! assert_eq!(receiver.as_receiver(), Some(&42));
//! ```
//!
//! # Generated Methods
//!
//! ```
//! # use alias_option_macro::alias_option;
//! # alias_option!(Sampler, Leader, Receiver);
//! let mut val = Sampler::Receiver(10);
//!
//! // Check variants
//! assert!(val.is_receiver());
//!
//! // Get references
//! assert_eq!(val.as_receiver(), Some(&10));
//! assert_eq!(val.as_receiver_mut(), Some(&mut 10));
//!
//! // Transform
//! let doubled = val.map(|x| x * 2);
//! assert_eq!(doubled.unwrap(), 20);
//!
//! // Unwrap variants
//! assert_eq!(Sampler::Receiver(5).unwrap_or(0), 5);
//! assert_eq!(Sampler::<i32>::Leader.unwrap_or(5), 5);
//! ```
//!
//! # Conversions
//!
//! ```
//! # use alias_option_macro::alias_option;
//! # alias_option!(Sampler, Leader, Receiver);
//! let from_some: Sampler<i32> = Some(42).into();
//! let from_none: Sampler<i32> = None.into();
//!
//! assert_eq!(from_some, Sampler::Receiver(42));
//! assert_eq!(from_none, Sampler::Leader);
//!
//! let to_option: Option<i32> = Sampler::Receiver(42).into();
//! assert_eq!(to_option, Some(42));
//! ```
//!
//! # Conditional Checks
//!
//! ```
//! # use alias_option_macro::alias_option;
//! # alias_option!(Sampler, Leader, Receiver);
//! let val = Sampler::Receiver(42);
//! assert!(val.is_receiver_and(|&x| x > 40));
//! assert!(!val.is_receiver_and(|&x| x < 40));
//! assert!(!Sampler::<i32>::Leader.is_receiver_and(|&x| x > 40));
//!
//! assert!(Sampler::<i32>::Leader.is_leader_or(|&x| x > 40));
//! assert!(val.is_leader_or(|&x| x > 40));
//! assert!(!val.is_leader_or(|&x| x < 40));
//! ```

#[macro_export]
macro_rules! alias_option {
    ($type_name:ident, $none_variant:ident, $some_variant:ident) => {
      paste::paste! {
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum $type_name<T> {
			$none_variant,
			$some_variant(T),
		}

		impl<T> $type_name<T> {
			pub fn [<is_ $none_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$none_variant)
			}

			pub fn [<is_ $some_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$some_variant(_))
			}

			pub fn [<is_ $some_variant:lower _and>]<F: FnOnce(&T) -> bool>(&self, f: F) -> bool {
				match self {
					$type_name::$some_variant(v) => f(v),
					_ => false,
				}
			}

			pub fn [<is_ $none_variant:lower _or>]<F: FnOnce(&T) -> bool>(&self, f: F) -> bool {
				match self {
					$type_name::$none_variant => true,
					$type_name::$some_variant(v) => f(v),
				}
			}

			pub fn [<as_ $some_variant:lower>](&self) -> Option<&T> {
				match self {
					$type_name::$some_variant(v) => Some(v),
					_ => None,
				}
			}

			pub fn [<as_ $some_variant:lower _mut>](&mut self) -> Option<&mut T> {
				match self {
					$type_name::$some_variant(v) => Some(v),
					_ => None,
				}
			}

			pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> $type_name<U> {
				match self {
					$type_name::$some_variant(v) => $type_name::$some_variant(f(v)),
					$type_name::$none_variant => $type_name::$none_variant,
				}
			}

			pub fn unwrap(self) -> T {
				match self {
					$type_name::$some_variant(v) => v,
					$type_name::$none_variant => {
						panic!("called `unwrap()` on a `{}`", stringify!($none_variant))
					}
				}
			}

			pub fn unwrap_or(self, default: T) -> T {
				match self {
					$type_name::$some_variant(v) => v,
					$type_name::$none_variant => default,
				}
			}

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
  };
}

#[cfg(test)]
mod tests {
    #[test]
    fn size_equivalence() {
        use std::num::NonZeroU32;
        alias_option!(Sampler, Leader, Receiver);
        assert_eq!(
            std::mem::size_of::<Sampler<i32>>(),
            std::mem::size_of::<Option<i32>>()
        );
        assert_eq!(
            std::mem::size_of::<Sampler<NonZeroU32>>(),
            std::mem::size_of::<Option<NonZeroU32>>()
        );
    }
}
