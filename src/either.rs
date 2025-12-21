/// Creates an Either-like enum with custom variant names.
///
/// See [`examples::EitherExample`](crate::examples::EitherExample) for a generated example.
///
/// # Example
///
/// ```
/// use enumizer::alias_either;
///
/// alias_either!(Choice, Primary, Secondary);
///
/// let primary: Choice<i32, String> = Choice::Primary(42);
/// let secondary: Choice<i32, String> = Choice::Secondary("text".to_string());
///
/// assert!(primary.is_primary());
/// assert!(secondary.is_secondary());
/// assert_eq!(primary.as_primary(), Some(&42));
/// ```
///
/// # Generated Methods
///
/// ```
/// use enumizer::alias_either;
/// alias_either!(Choice, Primary, Secondary);
/// let mut val: Choice<i32, String> = Choice::Primary(10);
///
/// assert!(val.is_primary());
/// assert_eq!(val.as_primary(), Some(&10));
/// assert_eq!(val.as_secondary(), None);
///
/// let doubled = val.map_primary(|x| x * 2);
/// assert_eq!(doubled.as_primary(), Some(&20));
/// ```
///
/// # Custom Traits
///
/// You can specify custom traits to derive instead of the default set.
/// Use the `traits:` keyword followed by a list of trait names in brackets.
///
/// ```
/// use enumizer::alias_either;
/// alias_either!(CustomEither, Left, Right, traits: [Debug, Clone, serde::Serialize, serde::Deserialize]);
/// let val: CustomEither<i32, String> = CustomEither::Left(42);
/// assert_eq!(format!("{:?}", val), "Left(42)");
/// assert_eq!(val.clone().unwrap_left(), 42);
/// let json = serde_json::to_string(&val).unwrap();
/// assert_eq!(json, r#"{"Left":42}"#);
/// ```
#[macro_export]
macro_rules! alias_either {
    ($type_name:ident, $left_variant:ident, $right_variant:ident) => {
        $crate::alias_either!($type_name, $left_variant, $right_variant, [Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash]);
    };
    ($type_name:ident, $left_variant:ident, $right_variant:ident, traits: [$($trait:path),*]) => {
        $crate::alias_either!($type_name, $left_variant, $right_variant, [$($trait),*]);
    };
    ($type_name:ident, $left_variant:ident, $right_variant:ident, [$($trait:path),*]) => {
        paste::paste! {
		#[derive($($trait),*)]
		pub enum $type_name<L, R> {
			$left_variant(L),
			$right_variant(R),
		}

		impl<L, R> $type_name<L, R> {
			/// Returns true if this is the left variant
			pub fn [<is_ $left_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$left_variant(_))
			}

			/// Returns true if this is the right variant
			pub fn [<is_ $right_variant:lower>](&self) -> bool {
				matches!(self, $type_name::$right_variant(_))
			}

			/// Returns a reference to the left value if this is the left variant
			pub fn [<as_ $left_variant:lower>](&self) -> Option<&L> {
				match self {
					$type_name::$left_variant(v) => Some(v),
					_ => None,
				}
			}

			/// Returns a mutable reference to the left value if this is the left variant
			pub fn [<as_ $left_variant:lower _mut>](&mut self) -> Option<&mut L> {
				match self {
					$type_name::$left_variant(v) => Some(v),
					_ => None,
				}
			}

			/// Returns a reference to the right value if this is the right variant
			pub fn [<as_ $right_variant:lower>](&self) -> Option<&R> {
				match self {
					$type_name::$right_variant(v) => Some(v),
					_ => None,
				}
			}

			/// Returns a mutable reference to the right value if this is the right variant
			pub fn [<as_ $right_variant:lower _mut>](&mut self) -> Option<&mut R> {
				match self {
					$type_name::$right_variant(v) => Some(v),
					_ => None,
				}
			}

			/// Maps the left value if this is the left variant
			pub fn [<map_ $left_variant:lower>]<T, F: FnOnce(L) -> T>(self, f: F) -> $type_name<T, R> {
				match self {
					$type_name::$left_variant(v) => $type_name::$left_variant(f(v)),
					$type_name::$right_variant(v) => $type_name::$right_variant(v),
				}
			}

			/// Maps the right value if this is the right variant
			pub fn [<map_ $right_variant:lower>]<T, F: FnOnce(R) -> T>(self, f: F) -> $type_name<L, T> {
				match self {
					$type_name::$left_variant(v) => $type_name::$left_variant(v),
					$type_name::$right_variant(v) => $type_name::$right_variant(f(v)),
				}
			}

			/// Unwraps the left value, panicking if this is the right variant
			pub fn [<unwrap_ $left_variant:lower>](self) -> L {
				match self {
					$type_name::$left_variant(v) => v,
					$type_name::$right_variant(_) => {
						panic!("called `unwrap_{}()` on a `{}`", stringify!([<$left_variant:lower>]), stringify!($right_variant))
					}
				}
			}

			/// Unwraps the right value, panicking if this is the left variant
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
