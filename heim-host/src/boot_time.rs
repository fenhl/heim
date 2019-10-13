use heim_common::prelude::*;

use crate::{sys, Time};

/// Returns future which resolves into the system boot [Time]
/// since the UNIX epoch.
///
/// [Time]: ./struct.Time.html
pub async fn boot_time() -> Result<Time> {
    // TODO: Cache the successful value, as it can't change later
    sys::boot_time().await
}
