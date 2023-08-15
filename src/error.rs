#[cfg(feature = "alloc")]
#[allow(clippy::module_name_repetitions)]
pub type BoxError = ::alloc::boxed::Box<dyn std::error::Error + Send + Sync + 'static>;
#[cfg(feature = "alloc")]
pub type BoxResult<T> = Result<T, BoxError>;
