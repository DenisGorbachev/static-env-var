//! This crate provides a [`static_env_var!`] macro for loading the environment variables statically in a `LazyLock`.

#![deny(unused_crate_dependencies)]

#[macro_export]
macro_rules! static_env_var {
    ($vis:vis $name:ident) => {
        $vis static $name: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::env::var(stringify!($name)).expect(concat!(stringify!($name), " must be set")));
    };
}
