use std::fmt;

#[cfg(feature = "extras")]
use ::{Path, Root};
#[cfg(feature = "extras")]
pub use serde_json::value::RawValue;
#[cfg(feature = "extras")]
use validation::{Error, Validate};

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(feature = "extras")]
pub type Extras = Option<::std::boxed::Box<RawValue>>;

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(not(feature = "extras"))]
pub type Extras = Void;

/// Type representing no user-defined data.
#[derive(Clone, Default, Serialize, Deserialize, Validate)]
pub struct Void {
    #[serde(default, skip_serializing)]
    _allow_unknown_fields: (),
}

impl fmt::Debug for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}

impl fmt::Display for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}

#[cfg(feature = "extras")]
impl Validate for ::std::boxed::Box<RawValue> {
    fn validate_minimally<P, R>(&self, _: &Root, _: P, _: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        // nop
    }

    fn validate_completely<P, R>(&self, _: &Root, _: P, _: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        // nop
    }
}
