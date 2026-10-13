//! Thin, safe-ish wrappers over Win32. Everything OS-specific lives here.
#![allow(clippy::missing_safety_doc)]
use std::collections::HashMap;
use std::ffi::c_void;
use std::os::windows::ffi::OsStrExt;

use base64::Engine;
use windows::core::{Interface, BOOL, PCWSTR, PWSTR};
use windows::Win32::Foundation::{CloseHandle, COLORREF, HWND, LPARAM, POINT, SIZE, WPARAM};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateRectRgn, DeleteDC, DeleteObject, GetDIBits, GetObjectW, SetWindowRgn,
    BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HGDIOBJ,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, IPersistFile, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED, STGM_READ,
};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
    PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};
use windows::Win32::UI::Shell::{
    IShellItemImageFactory, IShellLinkW, SHCreateItemFromParsingName, ShellExecuteExW, ShellLink,
    SEE_MASK_FLAG_NO_UI, SEE_MASK_NOASYNC, SHELLEXECUTEINFOW, SIIGBF_BIGGERSIZEOK, SIIGBF_ICONONLY,
};
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn wide(s: &str) -> Vec<u16> {
    std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

pub fn com_init() {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
}

pub fn to_hwnd(raw: isize) -> HWND {
    HWND(raw as *mut c_void)
}

/// One visible, user-facing top-level window (what the taskbar would show).
#[derive(Clone, Debug)]
pub struct AppWindow {
    pub hwnd: isize,
    pub pid: u32,
    pub hung: bool,
    pub minimized: bool,
}

unsafe extern "system" fn enum_cb(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let out = &mut *(lparam.0 as *mut Vec<AppWindow>);
    unsafe {
        if !IsWindowVisible(hwnd).as_bool() {
            return true.into();
        }
        if GetWindow(hwnd, GW_OWNER).is_ok() {
            return true.into();
        }
        let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        if ex & WS_EX_TOOLWINDOW.0 != 0 && ex & WS_EX_APPWINDOW.0 == 0 {
            return true.into();
        }
        if GetWindowTextLengthW(hwnd) == 0 {
            return true.into();
        }
        let mut cloaked: u32 = 0;
        let _ = DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut cloaked as *mut u32 as *mut c_void,
            4,
        );
        if cloaked != 0 {
            return true.into();
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        out.push(AppWindow {
            hwnd: hwnd.0 as isize,
            pid,
            hung: IsHungAppWindow(hwnd).as_bool(),
            minimized: IsIconic(hwnd).as_bool(),
        });
    }
    true.into()
}

/// Top-level windows in Z-order (topmost first).
pub fn enum_windows() -> Vec<AppWindow> {
    let mut v: Vec<AppWindow> = Vec::with_capacity(64);
    unsafe {
        let _ = EnumWindows(Some(enum_cb), LPARAM(&mut v as *mut _ as isize));
    }
    v
}

pub fn process_path(pid: u32) -> Option<String> {
    unsafe {
        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = [0u16; 1024];
        let mut len = buf.len() as u32;
        let r = QueryFullProcessImageNameW(h, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len);
        let _ = CloseHandle(h);
        r.ok()?;
        Some(String::from_utf16_lossy(&buf[..len as usize]))
    }
}

/// pid -> exe path cache (process paths never change for a pid's lifetime).
#[derive(Default)]
pub struct PathCache(HashMap<u32, Option<String>>);
impl PathCache {
    pub fn get(&mut self, pid: u32) -> Option<String> {
        self.0.entry(pid).or_insert_with(|| process_path(pid).map(|s| s.to_lowercase())).clone()
    }
    pub fn retain(&mut self, alive: &std::collections::HashSet<u32>) {
        self.0.retain(|k, _| alive.contains(k));
    }
}

pub struct LinkInfo {
    pub target: String,
    pub args: String,
    pub workdir: String,
}

pub fn resolve_lnk(path: &str) -> Option<LinkInfo> {
    unsafe {
        let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).ok()?;
        let pf: IPersistFile = link.cast().ok()?;
        let w = wide(path);
        pf.Load(PCWSTR(w.as_ptr()), STGM_READ).ok()?;
        let mut t = [0u16; 1024];
        link.GetPath(&mut t, std::ptr::null_mut(), 0).ok()?;
        let mut a = [0u16; 2048];
        let _ = link.GetArguments(&mut a);
        let mut d = [0u16; 1024];
        let _ = link.GetWorkingDirectory(&mut d);
        let s = |b: &[u16]| {
            let n = b.iter().position(|&c| c == 0).unwrap_or(b.len());
            String::from_utf16_lossy(&b[..n])
        };
        Some(LinkInfo { target: s(&t), args: s(&a), workdir: s(&d) })
    }
}

/// Large shell icon for any path, as PNG data URL.
pub fn icon_data_url(path: &str, size: i32) -> Option<String> {
    unsafe {
        let w = wide(path);
        let f: IShellItemImageFactory = SHCreateItemFromParsingName(PCWSTR(w.as_ptr()), None).ok()?;
        let hbm = f
            .GetImage(SIZE { cx: size, cy: size }, SIIGBF_ICONONLY | SIIGBF_BIGGERSIZEOK)
            .ok()?;
        let mut bm = BITMAP::default();
        GetObjectW(
            HGDIOBJ(hbm.0),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bm as *mut _ as *mut c_void),
        );
        let (wpx, hpx) = (bm.bmWidth, bm.bmHeight);
        if wpx <= 0 || hpx <= 0 {
            let _ = DeleteObject(HGDIOBJ(hbm.0));
            return None;
        }
        let mut bi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: wpx,
                biHeight: -hpx, // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut px = vec![0u8; (wpx * hpx * 4) as usize];
        let dc = CreateCompatibleDC(None);
        GetDIBits(dc, hbm, 0, hpx as u32, Some(px.as_mut_ptr() as *mut c_void), &mut bi, DIB_RGB_COLORS);
        let _ = DeleteDC(dc);
        let _ = DeleteObject(HGDIOBJ(hbm.0));
        // BGRA (premultiplied) -> RGBA straight
        let any_alpha = px.chunks(4).any(|c| c[3] != 0);
        for c in px.chunks_mut(4) {
            let (b, g, r, a) = (c[0], c[1], c[2], if any_alpha { c[3] } else { 255 });
            let un = |v: u8| if a == 0 || a == 255 { v } else { ((v as u32 * 255) / a as u32).min(255) as u8 };
            c[0] = un(r);
            c[1] = un(g);
            c[2] = un(b);
            c[3] = a;
        }
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, wpx as u32, hpx as u32);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().ok()?;
            wr.write_image_data(&px).ok()?;
        }
        Some(format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(out)
        ))
    }
}

/// ShellExecuteEx — works for .exe/.lnk/files/folders, honours UAC manifests.
pub fn shell_open(file: &str, args: &str, dir: &str) -> Result<(), String> {
    let f = wide(file);
    let a = wide(args);
    let d = wide(dir);
    let verb = wide("open");
    let mut info = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOASYNC | SEE_MASK_FLAG_NO_UI,
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(f.as_ptr()),
        lpParameters: if args.is_empty() { PCWSTR::null() } else { PCWSTR(a.as_ptr()) },
        lpDirectory: if dir.is_empty() { PCWSTR::null() } else { PCWSTR(d.as_ptr()) },
        nShow: SW_SHOWNORMAL.0,
        ..Default::default()
    };
    unsafe { ShellExecuteExW(&mut info).map_err(|e| e.message().to_string()) }
}

pub fn reveal_in_explorer(path: &str) {
    let _ = shell_open("explorer.exe", &format!("/select,\"{}\"", path), "");
}

pub fn foreground() -> isize {
    unsafe { GetForegroundWindow().0 as isize }
}

pub fn activate(hwnd: isize) {
    let h = to_hwnd(hwnd);
    unsafe {
        if IsIconic(h).as_bool() {
            let _ = ShowWindow(h, SW_RESTORE);
        }
        let _ = SetForegroundWindow(h);
    }
}

pub fn minimize(hwnd: isize) {
    unsafe {
        let _ = ShowWindow(to_hwnd(hwnd), SW_MINIMIZE);
    }
}

pub fn close(hwnd: isize) {
    unsafe {
        let _ = PostMessageW(Some(to_hwnd(hwnd)), WM_CLOSE, WPARAM(0), LPARAM(0));
    }
}

/// Show a window without stealing focus from the user's app.
#[allow(dead_code)]
pub fn show_no_activate(hwnd: isize) {
    unsafe {
        let h = to_hwnd(hwnd);
        let _ = ShowWindow(h, SW_SHOWNOACTIVATE);
        let _ = SetWindowPos(h, Some(HWND_TOPMOST), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
    }
}

pub fn cursor() -> (i32, i32) {
    let mut p = POINT::default();
    unsafe {
        let _ = GetCursorPos(&mut p);
    }
    (p.x, p.y)
}

pub fn lbutton_down() -> bool {
    unsafe { (GetAsyncKeyState(VK_LBUTTON.0 as i32) as u16 & 0x8000) != 0 }
}

/// Click-through on/off. WS_EX_LAYERED is always kept (Tauri transparent windows
/// need it for alpha); only WS_EX_TRANSPARENT is toggled.
pub fn set_click_through(hwnd: isize, on: bool) {
    let h = to_hwnd(hwnd);
    unsafe {
        let ex = GetWindowLongPtrW(h, GWL_EXSTYLE) as u32;
        let new = if on {
            ex | WS_EX_TRANSPARENT.0 | WS_EX_LAYERED.0
        } else {
            (ex | WS_EX_LAYERED.0) & !WS_EX_TRANSPARENT.0
        };
        if new != ex {
            SetWindowLongPtrW(h, GWL_EXSTYLE, new as isize);
            let _ = SetLayeredWindowAttributes(h, COLORREF(0), 255, LWA_ALPHA);
        }
    }
}

// ---- selective click-through via window region ----
//
// Restrict the window to the capsule rect (client device px). Outside the
// region Windows treats the window as absent: clicks and OLE drag-and-drop
// fall through to whatever is underneath. Inside, everything (including the
// cross-process WebView2 child) stays fully interactive — no WS_EX_TRANSPARENT,
// which would kill OLE drops.

pub fn set_click_region(hwnd: isize, x: f64, y: f64, w: f64, h: f64) {
    unsafe {
        let x1 = x.round() as i32;
        let y1 = y.round() as i32;
        let x2 = (x + w).round() as i32;
        let y2 = (y + h).round() as i32;
        let rgn = CreateRectRgn(x1, y1, x2.max(x1 + 1), y2.max(y1 + 1));
        if !rgn.0.is_null() {
            // `redraw: false` — repainting on every update made the DWM drop the
            // layered surface mid-frame (visible flicker) and, worse, re-entered
            // the OLE drag loop's hit-test, which cancelled Explorer drops.
            let _ = SetWindowRgn(to_hwnd(hwnd), Some(rgn), false);
        }
    }
}

/// Drop the region: the whole window is interactive again.
///
/// Required while *anything* is being dragged:
///   * internal tile drag — `whileDrag` lifts the tile outside the capsule and a
///     window region **clips rendering**, so the tile would vanish and the
///     pointer would leave the window, aborting the gesture;
///   * external Explorer drag — the OLE drop target is only hit inside the
///     region, so files dropped near (but not exactly on) the capsule were lost.
pub fn clear_click_region(hwnd: isize) {
    unsafe {
        let _ = SetWindowRgn(to_hwnd(hwnd), None, false);
    }
}
