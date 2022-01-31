use std::io::Error;

use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::HWND;

pub(crate) fn show_last_error(s: &str) {
	panic!("Error while {}: {}", s, Error::last_os_error());
}

pub(crate) unsafe fn validate_bool(b: BOOL, s: &str) {
	if b == 0 {
		show_last_error(s);
	}
}

pub(crate) unsafe fn validate_hwnd(hwnd: HWND, s: &str) -> HWND {
	if hwnd.is_null() {
		show_last_error(s);
	}
	hwnd
}