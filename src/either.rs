//! Creates an Either-like enum with custom variant names.
//!
//! # Example
//!
//! ```
//! use enumizer::alias_either;
//!
//! alias_either!(Choice, Primary, Secondary);
//!
//! let primary: Choice<i32, String> = Choice::Primary(42);
//! let secondary: Choice<i32, String> = Choice::Secondary("text".to_string());
//!
//! assert!(primary.is_primary());
//! assert!(secondary.is_secondary());
//! assert_eq!(primary.as_primary(), Some(&42));
//! ```
//!
//! # Generated Methods
//!
//! ```
//! use enumizer::alias_either;
//! alias_either!(Choice, Primary, Secondary);
//! let mut val: Choice<i32, String> = Choice::Primary(10);
//!
//! assert!(val.is_primary());
//! assert_eq!(val.as_primary(), Some(&10));
//! assert_eq!(val.as_secondary(), None);
//!
//! let doubled = val.map_primary(|x| x * 2);
//! assert_eq!(doubled.as_primary(), Some(&20));
//! ```
#[macro_export]
macro_rules! alias_either {
    ($type_name:ident, $left_variant:ident, $right_variant:ident) => {
        paste::paste! {
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum $type_name<L, R> {
			$left_variant(L),
			$right_variant(R),
		}

		impl<L, R> $type_name<L, R> {
			pub fn [<is_ $left_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$left_variant(_))
			}

			pub fn [<is_ $right_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$right_variant(_))
			}

			pub fn [<as_ $left_variant:lower>](&self) -> Option<&L> {
				match self {
					$type_name::$left_variant(v) => Some(v),
					_ => None,
				}
			}

			pub fn [<as_ $left_variant:lower _mut>](&mut self) -> Option<&mut L> {
				match self {
					$type_name::$left_variant(v) => Some(v),
					_ => None,
				}
			}

			pub fn [<as_ $right_variant:lower>](&self) -> Option<&R> {
				match self {
					$type_name::$right_variant(v) => Some(v),
					_ => None,
				}
			}

			pub fn [<as_ $right_variant:lower _mut>](&mut self) -> Option<&mut R> {
				match self {
					$type_name::$right_variant(v) => Some(v),
					_ => None,
				}
			}

			pub fn [<map_ $left_variant:lower>]<T, F: FnOnce(L) -> T>(self, f: F) -> $type_name<T, R> {
				match self {
					$type_name::$left_variant(v) => $type_name::$left_variant(f(v)),
					$type_name::$right_variant(v) => $type_name::$right_variant(v),
				}
			}

			pub fn [<map_ $right_variant:lower>]<T, F: FnOnce(R) -> T>(self, f: F) -> $type_name<L, T> {
				match self {
					$type_name::$left_variant(v) => $type_name::$left_variant(v),
					$type_name::$right_variant(v) => $type_name::$right_variant(f(v)),
				}
			}

			pub fn [<unwrap_ $left_variant:lower>](self) -> L {
				match self {
					$type_name::$left_variant(v) => v,
					$type_name::$right_variant(_) => {
						panic!("called `unwrap_{}()` on a `{}`", stringify!([<$left_variant:lower>]), stringify!($right_variant))
					}
				}
			}

			pub fn [<unwrap_ $right_variant:lower>](self) -> R {
				match self {
					$type_name::$right_variant(v) => v,
					$type_name::$left_variant(_) => {
						panic!("called `unwrap_{}()` on a `{}`", stringify!([<$right_variant:lower>]), stringify!($left_variant))
					}
				}
			}
		}
        }
    };
}
