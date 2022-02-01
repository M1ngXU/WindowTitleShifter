use std::io::Error;
use std::mem::{size_of, zeroed};
use std::ptr::null_mut;

use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HWND};
use winapi::um::commctrl::TBM_GETPOS;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
	COLOR_WINDOW, CreateWindowExW, CS_HREDRAW, CS_VREDRAW, DefWindowProcW, DispatchMessageW,
	EN_CHANGE, GetMessageW, GetSystemMetrics, GetWindowTextLengthW, GetWindowTextW, IDC_ARROW,
	LoadCursorW, MSG, PostQuitMessage, RegisterClassExW, SendMessageW, SetTimer, SetWindowTextW,
	ShowWindow, SM_CXBORDER, SM_CXMENUSIZE, SM_CYMENUSIZE, SM_CYSIZE, SW_SHOW, TranslateMessage,
	WM_COMMAND, WM_DESTROY, WM_HSCROLL, WM_TIMER, WNDCLASSEXW, WS_BORDER, WS_SYSMENU
};

use crate::error_handling::{show_last_error, validate_bool, validate_hwnd};
use crate::ui_element_creator::{create_input_box, create_label, create_slider};

mod error_handling;
mod ui_element_creator;

const INPUTBOX_ID: u16 = 12346;

const MARGIN: i32 = 10;
const STARTING_WIDTH: i32 = 500;
const UI_STARTING_WIDTH: i32 = STARTING_WIDTH - 2 * MARGIN;
const TIMER_THRESHOLD: isize = 100;
const TEXT_HEIGHT: i32 = 20;
const SLIDER_HEIGHT: i32 = 40;
const SHIFTINGSPEED_Y: i32 = 2 * MARGIN + 2 * TEXT_HEIGHT + SLIDER_HEIGHT;
const TRUNCATION_MAX: isize = 30;

static mut INPUTBOX: isize = 0;
static mut TRUNCATION: isize = 0;
static mut TRUNCATION_LABEL: isize = 0;
static mut SHIFTINGSPEED: isize = 0;
static mut SHIFTINGSPEED_LABEL: isize = 0;
static mut ROXL_TIMER_ID: usize = 123456;
static mut TIMER_INTERVAL: isize = 0;
static mut TRUNCATION_AMOUNT: isize = 0;
static mut TICKS: isize = 0;
static mut OFFSET: i32 = 0;

unsafe fn update_title(hwnd: HWND) {
	if INPUTBOX == 0 {
		return;
	}
	let mut buffer_len = GetWindowTextLengthW(INPUTBOX as HWND);
	let mut buffer = Vec::with_capacity(buffer_len as usize + 1);
	buffer_len = GetWindowTextW(INPUTBOX as HWND, buffer.as_mut_ptr(), buffer_len + 1);
	buffer.set_len(buffer_len as usize);
	let mut chars = String::from_utf16(&buffer).unwrap().chars().collect::<Vec<char>>();
	OFFSET %= chars.len().max(1) as i32;
	chars.rotate_left(OFFSET as usize);

	buffer = chars.iter().take(
		if TRUNCATION_AMOUNT == TRUNCATION_MAX {
			usize::MAX
		} else {
			TRUNCATION_AMOUNT as usize
		}.min(chars.len())
	).collect::<String>().convert();

	buffer.push(0);
	validate_bool(
		SetWindowTextW(hwnd, buffer.as_ptr()),
		"updating the window's title",
	);
}

unsafe extern "system" fn window_protocol(
	hwnd: HWND,
	msg: UINT,
	wparam: WPARAM,
	lparam: LPARAM,
) -> LRESULT {
	match msg {
		WM_DESTROY => PostQuitMessage(0),
		WM_TIMER if wparam == ROXL_TIMER_ID => {
			TICKS += TIMER_INTERVAL;
			if TICKS > TIMER_THRESHOLD {
				update_title(hwnd);
				OFFSET += 1;
				TICKS = 0;
			}
		}
		WM_COMMAND if wparam == ((EN_CHANGE as usize) << 16) + INPUTBOX_ID as usize => {
			OFFSET = 0;
			update_title(hwnd);
		}
		WM_HSCROLL if lparam == SHIFTINGSPEED => {
			TIMER_INTERVAL = SendMessageW(lparam as HWND, TBM_GETPOS, 0, 0);
			validate_bool(
				SetWindowTextW(
					SHIFTINGSPEED_LABEL as HWND,
					if TIMER_INTERVAL == 0 {
						OFFSET = 0;
						update_title(hwnd);
						"No character shifting.".to_string()
					} else {
						format!("Shifting speed: {}ms/character", (10.0 * TIMER_THRESHOLD as f64 / TIMER_INTERVAL as f64).round())
					}.convert().as_ptr(),
				), "updating the shifting-speed label",
			);
		}
		WM_HSCROLL if lparam == TRUNCATION => {
			TRUNCATION_AMOUNT = SendMessageW(lparam as HWND, TBM_GETPOS, 0, 0);
			update_title(hwnd);
			validate_bool(
				SetWindowTextW(
					TRUNCATION_LABEL as HWND,
					if TRUNCATION_AMOUNT == TRUNCATION_MAX {
						"The whole title is shown.".to_string()
					} else {
						format!("Maximum title length shown: {} characters", TRUNCATION_AMOUNT)
					}.convert().as_ptr(),
				), "updating the truncation label",
			);
		}
		_ => return DefWindowProcW(hwnd, msg, wparam, lparam)
	}
	0
}

fn main() {
	unsafe {
		let win = WNDCLASSEXW {
			cbSize: size_of::<WNDCLASSEXW>() as UINT,
			style: CS_HREDRAW | CS_VREDRAW,
			lpfnWndProc: Some(window_protocol),
			cbClsExtra: 0,
			cbWndExtra: 0,
			hInstance: GetModuleHandleW(null_mut()) as HINSTANCE,
			hIcon: null_mut(),
			hCursor: LoadCursorW(null_mut(), IDC_ARROW),
			hbrBackground: COLOR_WINDOW as HBRUSH,
			lpszMenuName: null_mut(),
			lpszClassName: "TestWindow2".convert().as_ptr(),
			hIconSm: null_mut(),
		};
		let atom = RegisterClassExW(&win);
		if atom == 0 {
			panic!("{}", Error::last_os_error());
		}

		let hwnd = validate_hwnd(
			CreateWindowExW(
				0,
				atom as *const u16,
				null_mut(), // auto_set by input box
				WS_SYSMENU | WS_BORDER,
				200, 200,
				STARTING_WIDTH + GetSystemMetrics(SM_CXMENUSIZE) - 2 * GetSystemMetrics(SM_CXBORDER),
				SHIFTINGSPEED_Y + TEXT_HEIGHT + SLIDER_HEIGHT + MARGIN
					+ GetSystemMetrics(SM_CYMENUSIZE) + GetSystemMetrics(SM_CYSIZE),
				null_mut(),
				null_mut(),
				win.hInstance,
				null_mut(),
			), "creating the window",
		);

		INPUTBOX = create_input_box(MARGIN, "Window Title", hwnd, win.hInstance, INPUTBOX_ID);

		TRUNCATION_LABEL = create_label(TEXT_HEIGHT + MARGIN, hwnd, win.hInstance);
		TRUNCATION = create_slider(
			2 * TEXT_HEIGHT + MARGIN,
			hwnd,
			win.hInstance,
			0,
			TRUNCATION_MAX,
			TRUNCATION_MAX,
		) as isize;

		SHIFTINGSPEED_LABEL = create_label(SHIFTINGSPEED_Y, hwnd, win.hInstance);
		SHIFTINGSPEED = create_slider(
			SHIFTINGSPEED_Y + TEXT_HEIGHT,
			hwnd,
			win.hInstance,
			0,
			50,
			TIMER_INTERVAL,
		) as isize;

		ROXL_TIMER_ID = SetTimer(hwnd, ROXL_TIMER_ID, 10, None);
		if ROXL_TIMER_ID == 0 {
			show_last_error("creating the shifting-update timer");
		}
		ShowWindow(hwnd, SW_SHOW);

		let mut msg: MSG = zeroed();
		while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
			TranslateMessage(&msg);
			DispatchMessageW(&msg);
		}
	}
}

pub trait WideString {
	fn convert(&self) -> Vec<u16>;
}

impl WideString for &str {
	fn convert(&self) -> Vec<u16> {
		self.encode_utf16().chain(std::iter::once(0)).collect()
	}
}

impl WideString for String {
	fn convert(&self) -> Vec<u16> {
		self.encode_utf16().chain(std::iter::once(0)).collect()
	}
}