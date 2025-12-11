This crate provides simple macros for generating enums that are equivalent and convertible to standard library enums, with user-chosen variant names, in order to increase code legibility.
So `alias_option!(Value, Found, Searching);` becomes

```rs
enum Value<T> {
  Found(T),
  Searching
}
```

and `alias_result!(Response, Success, Failure);` becomes

```rs
enum Response<T,U> {
  Success(T),
  Failure(U)
}
```

and the generated types have all equivalent functions `is_found_and` (or most, depending on whether we implemented them yet or not :) ). That is, if `Option<T>` has `is_some_and`, `Value<T>` has `is_found_and`. And if your codebase requires actual `Result` or `Option` types, just use `into` - we implement `From<Option/Result>` and `Into<Option/Result>`.
