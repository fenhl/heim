#![type_length_limit = "3825948"]

use heim_common::prelude::*;
use heim_cpu as cpu;

#[cfg(target_os = "linux")]
async fn linux_frequencies() -> Result<()> {
    let mut frequencies = cpu::os::linux::frequencies().boxed();
    while let Some(freq) = frequencies.next().await {
        dbg!(freq?);
    }

    Ok(())
}

#[heim_derive::main]
async fn main() -> Result<()> {
    let freq = cpu::frequency().await;
    dbg!(freq?);

    #[cfg(target_os = "linux")]
    linux_frequencies().await?;

    Ok(())
}
