use log::{debug};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

// TODO(dmi): @incomplete It should return a vector of strings,
// so we can reference it while parsing other tables.

pub fn debug_user_strings(data: &[u8]) -> Result<()> {
	Ok(())
}
