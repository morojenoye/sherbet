#![no_main]
#![no_std]
extern crate rv32;
extern crate sherbet;

use rv32::{interrupt, pmp};

#[no_mangle]
extern "C" fn sherbet_user_provided_entry() {
	unsafe {
		interrupt::setup();
		pmp::setup();
		sherbet::reloc();
	}
	loop {}
}
