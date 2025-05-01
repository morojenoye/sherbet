#![no_main]
#![no_std]
extern crate sherbet;

#[no_mangle]
extern "Rust" fn sherbet_user_provided_entry() {
	loop {}
}
