use std::ffi::{CString, NulError, c_void};

use windows::Win32::Foundation::{HMODULE, MAX_PATH};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow;
use windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW};
use windows::core::{GUID, HRESULT, IUnknown, PCSTR, PCWSTR};

#[allow(dead_code)]
#[repr(i8)]
#[derive(Clone, Copy)]
enum TetherResult {
    Undefined = 0,
    GetSystemDirectoryFail = -1,
    BufferError = -2,
    LoadLibraryError = -3,
    GetAddressError = -4,
}

impl std::fmt::Display for TetherResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

fn string_to_pcwstr(s: &str) -> Result<(Box<[u16]>, PCWSTR), NulError> {
    let mut utf16: Vec<u16> = s.encode_utf16().collect();
    utf16.push(0);
    let boxed = utf16.into_boxed_slice();
    let ptr = PCWSTR(boxed.as_ptr());
    Ok((boxed, ptr))
}
fn string_to_pcstr(str: &str) -> Result<(CString, PCSTR), NulError> {
    let c_string = CString::new(str)?;
    let pcstr = PCSTR(c_string.as_ptr() as *const u8);
    Ok((c_string, pcstr))
}

fn error_box(result: (TetherResult, Option<String>)) {
    let buffer = match result.1 {
        Some(m) => format!("{} ({})", m, result.0),
        None => format!("{}", result.0),
    };

    let (_box1, text) = string_to_pcwstr(&buffer).expect("Failed to create MessageBoxW text.");
    let (_box2, caption) = string_to_pcwstr(&buffer).expect("Failed to create MessageBoxW caption.");

    unsafe {
        MessageBoxW(
            Some(GetActiveWindow()),
            text,
            caption,
            MB_OK,
        )
    };
}

fn create_tether(
    module_name: &str,
    proc_name: &str,
) -> Result<unsafe extern "system" fn() -> isize, (TetherResult, Option<String>)> {
    let mut buffer: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];

    let len = unsafe { GetSystemDirectoryW(Some(&mut buffer)) };
    if len == 0 || len >= MAX_PATH {
        return Err((
            TetherResult::GetSystemDirectoryFail,
            Some("Couldn't get system directory.".to_string()),
        ));
    }

    if len + 1 >= MAX_PATH {
        return Err((
            TetherResult::BufferError,
            Some("Buffer too small for system directory.".to_string()),
        ));
    }
    buffer[len as usize] = '\\' as u16;

    let total_len = len + 1 + module_name.len() as u32;
    if total_len >= MAX_PATH {
        return Err((
            TetherResult::BufferError,
            Some("Buffer too small for module name.".to_string()),
        ));
    }
    let start_index = len as usize + 1;
    for (i, val) in module_name.chars().enumerate() {
        buffer[start_index + i] = val as u16;
    }

    let Ok(lib) = (unsafe { LoadLibraryW(PCWSTR::from_raw(buffer.as_ptr())) }) else {
        return Err((
            TetherResult::LoadLibraryError,
            Some("Failed to load library when creating tether.".to_string()),
        ));
    };

    let Ok((_cstr, proc_pcstr)) = string_to_pcstr(proc_name) else {
        return Err((
            TetherResult::BufferError,
            Some("Failed to create procedure name string.".to_string()),
        ));
    };

    let Some(addr) = (unsafe { GetProcAddress(lib, proc_pcstr) }) else {
        return Err((
            TetherResult::GetAddressError,
            Some(format!(
                "Failed to get address of {} in {}",
                proc_name, module_name
            )),
        ));
    };

    Ok(addr)
}

// dxgi
type CreateDXGIFactoryFn =
    extern "stdcall" fn(riid: *const GUID, ppfactory: *mut *mut c_void) -> HRESULT;

#[unsafe(no_mangle)]
unsafe extern "stdcall" fn tether_CreateDXGIFactory(
    riid: *const GUID,
    ppfactory: *mut *mut c_void,
) -> HRESULT {
    match create_tether("dxgi.dll", "CreateDXGIFactory") {
        Ok(addr) => (unsafe {
            std::mem::transmute::<unsafe extern "system" fn() -> isize, CreateDXGIFactoryFn>(addr)
        })(riid, ppfactory),
        Err(err) => {
            error_box(err);
            HRESULT(0x80004005u32 as i32)
        }
    }
}
#[cfg(target_env = "msvc")]
#[unsafe(link_section = ".drectve")]
#[used]
static LINK_DIRECTIVE_1: [u8; 51] = *b"/EXPORT:CreateDXGIFactory=tether_CreateDXGIFactory\0";

type CreateDXGIFactory1Fn =
    extern "stdcall" fn(riid: *const GUID, ppfactory: *mut *mut c_void) -> HRESULT;

#[unsafe(no_mangle)]
unsafe extern "stdcall" fn tether_CreateDXGIFactory1(
    riid: *const GUID,
    ppfactory: *mut *mut c_void,
) -> HRESULT {
    match create_tether("dxgi.dll", "CreateDXGIFactory1") {
        Ok(addr) => (unsafe {
            std::mem::transmute::<unsafe extern "system" fn() -> isize, CreateDXGIFactory1Fn>(addr)
        })(riid, ppfactory),
        Err(err) => {
            error_box(err);
            HRESULT(0x80004005u32 as i32)
        }
    }
}
#[cfg(target_env = "msvc")]
#[unsafe(link_section = ".drectve")]
#[used]
static LINK_DIRECTIVE_2: [u8; 53] = *b"/EXPORT:CreateDXGIFactory1=tether_CreateDXGIFactory1\0";

type CreateDXGIFactory2Fn =
    extern "stdcall" fn(flags: u32, riid: *const GUID, ppfactory: *mut *mut c_void) -> HRESULT;

#[unsafe(no_mangle)]
unsafe extern "stdcall" fn tether_CreateDXGIFactory2(
    flags: u32,
    riid: *const GUID,
    ppfactory: *mut *mut c_void,
) -> HRESULT {
    match create_tether("dxgi.dll", "CreateDXGIFactory2") {
        Ok(addr) => (unsafe {
            std::mem::transmute::<unsafe extern "system" fn() -> isize, CreateDXGIFactory2Fn>(addr)
        })(flags, riid, ppfactory),
        Err(err) => {
            error_box(err);
            HRESULT(0x80004005u32 as i32)
        }
    }
}
#[cfg(target_env = "msvc")]
#[unsafe(link_section = ".drectve")]
#[used]
static LINK_DIRECTIVE_3: [u8; 53] = *b"/EXPORT:CreateDXGIFactory2=tether_CreateDXGIFactory2\0";

// dinput8
type DirectInput8CreateFn = extern "stdcall" fn(
    hinst: HMODULE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: IUnknown,
) -> HRESULT;

#[unsafe(no_mangle)]
unsafe extern "stdcall" fn tether_DirectInput8Create(
    hinst: HMODULE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: IUnknown,
) -> HRESULT {
    match create_tether("dinput8.dll", "DirectInput8Create") {
        Ok(addr) => (unsafe {
            std::mem::transmute::<unsafe extern "system" fn() -> isize, DirectInput8CreateFn>(addr)
        })(hinst, dwversion, riidltf, ppvout, punkouter),
        Err(err) => {
            error_box(err);
            HRESULT(0x80004005u32 as i32)
        }
    }
}
#[cfg(target_env = "msvc")]
#[unsafe(link_section = ".drectve")]
#[used]
static LINK_DIRECTIVE_4: [u8; 53] = *b"/EXPORT:DirectInput8Create=tether_DirectInput8Create\0";

// d3d11
type D3D11CreateDeviceFn = extern "stdcall" fn(
    adapter: *mut c_void,
    driver_type: u32,
    software: *mut c_void,
    flags: u32,
    feature_levels: *const u32,
    feature_levels_count: u32,
    sdk_version: u32,
    device: *mut *mut c_void,
    feature_level: *mut u32,
    immediate_context: *mut *mut c_void,
) -> HRESULT;

#[unsafe(no_mangle)]
unsafe extern "stdcall" fn tether_D3D11CreateDevice(
    adapter: *mut c_void,
    driver_type: u32,
    software: *mut c_void,
    flags: u32,
    feature_levels: *const u32,
    feature_levels_count: u32,
    sdk_version: u32,
    device: *mut *mut c_void,
    feature_level: *mut u32,
    immediate_context: *mut *mut c_void,
) -> HRESULT {
    match create_tether("d3d11.dll", "D3D11CreateDevice") {
        Ok(addr) => (unsafe {
            std::mem::transmute::<unsafe extern "system" fn() -> isize, D3D11CreateDeviceFn>(addr)
        })(
            adapter,
            driver_type,
            software,
            flags,
            feature_levels,
            feature_levels_count,
            sdk_version,
            device,
            feature_level,
            immediate_context,
        ),
        Err(err) => {
            error_box(err);
            HRESULT(0x80004005u32 as i32)
        }
    }
}
#[cfg(target_env = "msvc")]
#[unsafe(link_section = ".drectve")]
#[used]
static LINK_DIRECTIVE_5: [u8; 51] = *b"/EXPORT:D3D11CreateDevice=tether_D3D11CreateDevice\0";

type D3D11CreateDeviceAndSwapChainFn = extern "stdcall" fn(
    adapter: *mut c_void,
    driver_type: u32,
    software: *mut c_void,
    flags: u32,
    feature_levels: *const u32,
    feature_levels_count: u32,
    sdk_version: u32,
    swap_chain_desc: *const c_void,
    swap_chain: *mut *mut c_void,
    device: *mut *mut c_void,
    feature_level: *mut u32,
    immediate_context: *mut *mut c_void,
) -> HRESULT;

#[unsafe(no_mangle)]
unsafe extern "stdcall" fn tether_D3D11CreateDeviceAndSwapChain(
    adapter: *mut c_void,
    driver_type: u32,
    software: *mut c_void,
    flags: u32,
    feature_levels: *const u32,
    feature_levels_count: u32,
    sdk_version: u32,
    swap_chain_desc: *const c_void,
    swap_chain: *mut *mut c_void,
    device: *mut *mut c_void,
    feature_level: *mut u32,
    immediate_context: *mut *mut c_void,
) -> HRESULT {
    match create_tether("d3d11.dll", "D3D11CreateDeviceAndSwapChain") {
        Ok(addr) => (unsafe {
            std::mem::transmute::<
                unsafe extern "system" fn() -> isize,
                D3D11CreateDeviceAndSwapChainFn,
            >(addr)
        })(
            adapter,
            driver_type,
            software,
            flags,
            feature_levels,
            feature_levels_count,
            sdk_version,
            swap_chain_desc,
            swap_chain,
            device,
            feature_level,
            immediate_context,
        ),
        Err(err) => {
            error_box(err);
            HRESULT(0x80004005u32 as i32)
        }
    }
}
#[cfg(target_env = "msvc")]
#[unsafe(link_section = ".drectve")]
#[used]
static LINK_DIRECTIVE_6: [u8; 75] =
    *b"/EXPORT:D3D11CreateDeviceAndSwapChain=tether_D3D11CreateDeviceAndSwapChain\0";
