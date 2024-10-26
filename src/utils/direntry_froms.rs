use std::fs::DirEntry;

use crate::prelude::*;

// EXAMPLE OF USING W STRUCTURE

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;
    fn try_from(val: W<&DirEntry>) -> Result<String> {
        val.0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::InvalidPath)
    }
}
