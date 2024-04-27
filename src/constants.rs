#[cfg(feature = "develop")]
pub static DEFAULT_SERVICE: &str = "develop";
#[cfg(not(feature = "develop"))]
pub static DEFAULT_SERVICE: &str = "default";
