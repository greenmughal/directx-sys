extern crate com_rs;
extern crate directx_sys;
extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

use com_rs::ComPtr;
use directx_sys::d2d::{self, D2D1Factory, D2D1RenderTarget};
use directx_sys::dwrite::{self, DWriteFactory, DWriteTextFormat};
use kernel32::*;
use user32::*;
use winapi::*;

fn utf16_str(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide()
                 .chain(Some(0).into_iter())
                 .collect::<Vec<_>>()
}

unsafe extern "system" fn wndproc(window: HWND, msg: UINT, wparam: WPARAM,
                                   lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        },
        _ => DefWindowProcW(window, msg, wparam, lparam)
    }
}

fn main() {
    let class_name = utf16_str("WindowClass");
    let window_title = utf16_str("DirectWrite Demo");

    let hwnd = unsafe {
        let wcex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(ptr::null()),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: ptr::null_mut()
        };
        RegisterClassExW(&wcex);

        let hwnd = CreateWindowExW(
            WS_EX_APPWINDOW,
            class_name.as_ptr(),
            window_title.as_ptr() as LPCWSTR,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1280,
            720,
            ptr::null_mut(),
            ptr::null_mut(),
            GetModuleHandleW(ptr::null()),
            ptr::null_mut());

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        hwnd
    };

    let mut d2d_factory: ComPtr<d2d::ID2D1Factory> = ComPtr::new();
    let result = unsafe {
        d2d::D2D1CreateFactory(
            d2d::FactoryType::SingleThreaded,
            &d2d_factory.iid(),
            &d2d::FactoryOptions::default(),
            d2d_factory.as_mut())
    };
    assert_eq!(result, 0);
    assert!(!d2d_factory.is_null());

    let mut dwrite_factory: ComPtr<dwrite::IDWriteFactory> = ComPtr::new();
    let result = unsafe {
        dwrite::DWriteCreateFactory(
            dwrite::FactoryType::Shared,
            &dwrite_factory.iid(),
            dwrite_factory.as_mut())
    };
    assert_eq!(result, 0);
    assert!(!dwrite_factory.is_null());

    let text = utf16_str("Hello Rust!");
    let text_length = text.len();

    let mut text_format: ComPtr<dwrite::IDWriteTextFormat> = ComPtr::new();
    let result = unsafe { dwrite_factory.create_text_format(
        utf16_str("Times New Roman").as_ptr(),
        ptr::null(),
        dwrite::FontWeight::Normal,
        dwrite::FontStyle::Normal,
        dwrite::FontStretch::Normal,
        72.0,
        utf16_str("en-us").as_ptr(),
        text_format.as_mut())
    };
    assert_eq!(result, 0);
    assert!(!text_format.is_null());

    unsafe {
        text_format.set_text_alignment(dwrite::TextAlignment::Center);
        text_format.set_paragraph_alignment(dwrite::ParagraphAlignment::Center);
    }

    let mut rect: winapi::RECT = unsafe { ::std::mem::zeroed() };
    unsafe { GetClientRect(hwnd, &mut rect) };

    let size = d2d::SizeU {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32
    };

    let mut render_target: ComPtr<d2d::ID2D1HwndRenderTarget> = ComPtr::new();
    let result = unsafe {
        d2d_factory.create_hwnd_render_target(
            &d2d::RenderTargetProperties::default(),
            &d2d::HwndRenderTargetProperties {
                hwnd: hwnd,
                pixel_size: size,
                present_options: d2d::PresentOptions::empty()
            },
            render_target.as_mut())
    };
    assert_eq!(result, 0);
    assert!(!render_target.is_null());

    let mut black_brush: ComPtr<d2d::ID2D1SolidColorBrush> = ComPtr::new();
    let result = unsafe {
        render_target.create_solid_color_brush(
            &d2d::ColorF { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
            ptr::null(),
            black_brush.as_mut())
    };
    assert_eq!(result, 0);
    assert!(!black_brush.is_null());

    let mut msg: MSG = unsafe { mem::zeroed() };
    loop {
        unsafe {
            while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        if msg.message == WM_QUIT { break };

        let area = d2d::RectF {
            left: rect.left as f32,
            top: rect.top as f32,
            right: (rect.right - rect.left) as f32,
            bottom: (rect.bottom - rect.top) as f32
        };

        let identity = d2d::Matrix3x2F {
            m: [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]
        };

        let bg_color = d2d::ColorF { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };

        unsafe {
            render_target.begin_draw();
            render_target.clear(&bg_color);
            render_target.set_transform(&identity);
            render_target.draw_text(
                text.as_ptr(),
                text_length as u32,
                text_format.as_ptr(),
                &area,
                black_brush.as_ptr(),
                d2d::DrawTextOptions::default(),
                dwrite::MeasuringMode::Natural);
            render_target.end_draw(
                ptr::null_mut(),
                ptr::null_mut());
        }
    }
}