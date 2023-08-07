#[cfg(feature = "alloc")]
pub type BoxError = ::alloc::boxed::Box<dyn std::error::Error + Send + Sync + 'static>;
#[cfg(feature = "alloc")]
pub type BoxResult<T> = Result<T, BoxError>;
