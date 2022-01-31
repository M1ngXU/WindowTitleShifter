use std::ptr::null_mut;

use winapi::ctypes::c_int;
use winapi::shared::minwindef::HINSTANCE;
use winapi::shared::windef::HWND;
use winapi::um::commctrl::{
	TBM_SETPOS, TBM_SETRANGEMAX, TBM_SETRANGEMIN,
	TBS_AUTOTICKS, TRACKBAR_CLASS, WC_EDIT, WC_STATIC,
};
use winapi::um::winuser::{
	CreateWindowExW, ES_AUTOHSCROLL, ES_CENTER, PostMessageW, SS_LEFT,
	WM_HSCROLL, WS_CHILD, WS_EX_CLIENTEDGE, WS_VISIBLE,
};

use crate::{MARGIN, SLIDER_HEIGHT, TEXT_HEIGHT, UI_STARTING_WIDTH, WideString};
use crate::error_handling::validate_hwnd;

pub(crate) unsafe fn create_slider(
	y: c_int,
	parent: HWND,
	h_instance: HINSTANCE,
	min: isize,
	max: isize,
	start: isize,
) -> isize {
	let hwnd = validate_hwnd(
		CreateWindowExW(
			WS_EX_CLIENTEDGE,
			TRACKBAR_CLASS.convert().as_ptr(),
			"".convert().as_ptr(),
			WS_VISIBLE | WS_CHILD | TBS_AUTOTICKS,
			MARGIN,
			y,
			UI_STARTING_WIDTH,
			SLIDER_HEIGHT,
			parent,
			null_mut(),
			h_instance,
			null_mut(),
		), "creating a slider",
	);
	PostMessageW(hwnd, TBM_SETRANGEMIN, 1, min);
	PostMessageW(hwnd, TBM_SETRANGEMAX, 1, max);
	PostMessageW(hwnd, TBM_SETPOS, 1, start);

	//init text
	PostMessageW(parent, WM_HSCROLL, 0, hwnd as isize);

	hwnd as isize
}

pub(crate) unsafe fn create_label(
	y: c_int,
	parent: HWND,
	h_instance: HINSTANCE,
) -> isize {
	validate_hwnd(
		CreateWindowExW(
			0,
			WC_STATIC.convert().as_ptr(),
			"".convert().as_ptr(),
			SS_LEFT | WS_VISIBLE | WS_CHILD,
			MARGIN,
			y,
			UI_STARTING_WIDTH,
			TEXT_HEIGHT,
			parent,
			null_mut(),
			h_instance,
			null_mut(),
		), "creating a label",
	) as isize
}

pub(crate) unsafe fn create_input_box(
	y: c_int,
	starting_text: &str,
	parent: HWND,
	h_instance: HINSTANCE,
) -> isize {
	validate_hwnd(
		CreateWindowExW(
			WS_EX_CLIENTEDGE,
			WC_EDIT.convert().as_ptr(),
			starting_text.convert().as_ptr(),
			WS_VISIBLE | WS_CHILD | ES_CENTER | ES_AUTOHSCROLL,
			MARGIN,
			y,
			UI_STARTING_WIDTH,
			TEXT_HEIGHT,
			parent,
			null_mut(),
			h_instance,
			null_mut(),
		), "creating a input box",
	) as isize
}