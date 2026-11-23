//! Moving one field, and zeroing everything below it.

use crate::Version::{Enum::Enum as Part, Struct::Struct as Number};

/// A bump moves one field, zeroes the fields below it, and drops the
/// pre-release: a release is a clean `major.minor.patch`.
pub fn Fn(Version:Number, Part:Part) -> Number {
	match Part {
		Part::Major => Number { Major:Version.Major + 1, Minor:0, Patch:0, Pre:None },
		Part::Minor => Number { Major:Version.Major, Minor:Version.Minor + 1, Patch:0, Pre:None },
		Part::Patch => Number { Major:Version.Major, Minor:Version.Minor, Patch:Version.Patch + 1, Pre:None },
	}
}
