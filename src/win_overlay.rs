use std::sync::{Mutex, atomic::{AtomicPtr, Ordering}};
use std::{mem, ptr};

use winapi::um::{uxtheme::*, dwmapi::*, winuser::*, wingdi::{GetStockObject, BLACK_BRUSH}};
use winapi::shared::windef::*;
use winapi::shared::d3d9::*;
use winapi::shared::d3d9types::*;
use winapi::shared::d3d9caps::*;

use crate::patcher;
use crate::win32;

static OVERLAY: AtomicPtr<Overlay> = AtomicPtr::new(0 as _);

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    color: u32,
    u: f32,
    v: f32,
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w: usize, l: isize) -> isize {
    match msg {
        WM_CREATE => 0,
        WM_SIZE => 0,
        WM_PAINT => {
            let overlay = OVERLAY.load(Ordering::Relaxed);

            if overlay as usize == 0 {
                return 0
            }
            
            let overlay = &mut *overlay;
            let device = overlay.device.lock().unwrap();
            let device = device.as_ref().unwrap();

            device.BeginScene();
        
            device.Clear(1, &D3DRECT {
                x1: 0,
                y1: 0,
                x2: overlay.system_metrics.0,
                y2: overlay.system_metrics.1
            }, D3DCLEAR_TARGET, 0x00000000, 0.0, 0);

            if win32::is_ingame() {
                let hwnd = GetForegroundWindow();

                if GetWindowLongW(hwnd, GWL_STYLE) & WS_OVERLAPPEDWINDOW as i32 == 0 && GetWindowLongW(hwnd, GWL_EXSTYLE) & WS_EX_TOPMOST as i32 != 0 {
                    let scan = MapVirtualKeyW(0x7a, 0);

                    win32::keybd_event_vk_scan(&[
                        (0, 0x7a, scan as u16 & 0x00FF),
                        (KEYEVENTF_KEYUP, 0x7a, scan as u16 & 0x00FF),
                    ]);

                    patcher::IS_MC_FULLSCREEN.store(false, Ordering::Relaxed);
                }

                if patcher::chat_opened!() {
                    let hwnd = GetForegroundWindow();
    
                    let mut rect = mem::zeroed::<RECT>();
                    GetWindowRect(hwnd, &mut rect);

                    let h = rect.bottom - rect.top;
                    let w = rect.right - rect.left;

                    let mut mi = MONITORINFO {
                        cbSize: mem::size_of::<MONITORINFO>() as u32,
                        rcMonitor: mem::zeroed(),
                        rcWork: mem::zeroed(),
                        dwFlags: 0,
                    };
        
                    if GetMonitorInfoW(MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY), &mut mi) != 0 {
                        let mw = (mi.rcMonitor.right - mi.rcMonitor.left) as f32;
                        let mh = (mi.rcMonitor.bottom - mi.rcMonitor.top) as f32;

                        let (x, y, size) = if w == mw as i32 && h == mh as i32 {
                            (rect.left as f32 + mw * 0.0015, rect.bottom as f32 - 53.0, 23.0)
                        } else if h > 518 && w > 655 {
                            (rect.left as f32 + 12.0, rect.bottom as f32 - 62.0, 23.0)
                        } else {
                            (rect.left as f32 + 10.0, rect.bottom as f32 - 35.0, 12.0)
                        };
    
                        device.SetTexture(0, match patcher::korean_mode!() {
                            true => overlay.kor_texture,
                            false => overlay.eng_texture
                        }.unwrap() as _);
    
                        let vertices = &[Vertex {
                            x, y, z: 0.0, w: 1.0,
                            color: 0xFFFFFFFF,
                            u: 0.0, v: 0.0,
                        }, Vertex {
                            x: x + size, y, z: 0.0, w: 1.0,
                            color: 0xFFFFFFFF,
                            u: 1.0, v: 0.0,
                        }, Vertex {
                            x, y: y + size, z: 0.0, w: 1.0,
                            color: 0xFFFFFFFF,
                            u: 0.0, v: 1.0,
                        }, Vertex {
                            x: x + size, y: y + size, z: 0.0, w: 1.0,
                            color: 0xFFFFFFFF,
                            u: 1.0, v: 1.0,
                        }];
    
                        device.SetPixelShader(0 as _);
                        device.SetFVF(D3DFVF_XYZRHW | D3DFVF_DIFFUSE | D3DFVF_TEX1);
                        device.DrawPrimitiveUP(D3DPT_TRIANGLESTRIP, 2, vertices as *const Vertex as _, mem::size_of::<Vertex>() as u32);
                    }
                }
            }

            device.EndScene();
            device.Present(0 as _, 0 as _, 0 as _, 0 as _);

            0
        },
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w, l)
    }
}

pub struct Overlay {
    hwnd: Option<HWND>,
    d3d: Option<&'static mut IDirect3D9>,
    device: Mutex<Option<&'static mut IDirect3DDevice9>>,
    system_metrics: (i32, i32),

    eng_texture: Option<*mut IDirect3DTexture9>,
    kor_texture: Option<*mut IDirect3DTexture9>,
}

impl Overlay {
    pub fn new() -> Self {
        Overlay {
            device: Mutex::new(None),
            hwnd: None,
            d3d: None,
            system_metrics: (0, 0),

            eng_texture: None,
            kor_texture: None
        }
    }

    pub fn run(&mut self) -> bool {
        unsafe {
            let w = GetSystemMetrics(SM_CXSCREEN);
            let h = GetSystemMetrics(SM_CYSCREEN);
            self.system_metrics = (w, h);

            let title = b"Korean Patcher Overlay Window".iter().map(|&x| x as u16).collect::<Vec<u16>>();
            let title = title.as_slice();
        
            let wnd_class = WNDCLASSEXW {
                cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: 0 as _,
                hIcon: 0 as _,
                hCursor: LoadCursorW(0 as _, IDC_ARROW),
                hbrBackground: GetStockObject(BLACK_BRUSH as _) as _,
                lpszMenuName: 0 as _,
                lpszClassName: title.as_ptr(),
                hIconSm: 0 as _,
            };
            
            RegisterClassExW(&wnd_class);
        
            let wnd = CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_NOACTIVATE, 
                title.as_ptr(), 0 as _, WS_POPUP | WS_VISIBLE, 0, 0, w, h,
                0 as _, 0 as _, 0 as _, 0 as _
            );

            self.hwnd = Some(wnd);
        
            DwmExtendFrameIntoClientArea(wnd, &MARGINS {
                cxLeftWidth: 0,
                cxRightWidth: 0,
                cyTopHeight: w,
                cyBottomHeight: h,
            });
            SetLayeredWindowAttributes(wnd, 0, 255, LWA_ALPHA);
            SetWindowDisplayAffinity(wnd, 0x00000011);
            ShowWindow(wnd, SW_SHOW);

            let raw_d3d = Direct3DCreate9(D3D_SDK_VERSION);
            let d3d = &mut *raw_d3d;
        
            let mut params = D3DPRESENT_PARAMETERS {
                PresentationInterval: D3DPRESENT_INTERVAL_IMMEDIATE,
                SwapEffect: D3DSWAPEFFECT_DISCARD,
                BackBufferFormat: D3DFMT_A8R8G8B8,
                BackBufferHeight: h as _,
                BackBufferWidth: w as _,
                hDeviceWindow: wnd,
                Windowed: 1,
                
                // zeroed
                FullScreen_RefreshRateInHz: 0,
                AutoDepthStencilFormat: 0,
                EnableAutoDepthStencil: 0,
                MultiSampleQuality: 0,
                BackBufferCount: 0,
                MultiSampleType: 0,
                Flags: 0,
            };
            
            let mut raw_device = ptr::null_mut::<IDirect3DDevice9>();
            let err = d3d.CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL, wnd, D3DCREATE_HARDWARE_VERTEXPROCESSING, &mut params, &mut raw_device);

            if err != 0 {
                // When device is lost, it has possibility to minecraft is fullscreen mode. so check about it or just rethrow error
                if err as u32 == 0x88760868 {
                    if win32::is_ingame() {
                        use std::time::Duration;

                        // simulate F11 keypress
                        let scan = MapVirtualKeyW(0x7a, 0);
                        
                        win32::keybd_event_vk_scan(&[
                            (0, 0x7a, scan as u16 & 0x00FF),
                            (KEYEVENTF_KEYUP, 0x7a, scan as u16 & 0x00FF),
                        ]);

                        // Give little bit delay for minecraft leave fullscreen mode
                        std::thread::sleep(Duration::from_millis(300));

                        let err = d3d.CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL, wnd, D3DCREATE_HARDWARE_VERTEXPROCESSING, &mut params, &mut raw_device);
                        
                        win32::toggle_fullscreen_custom();

                        patcher::IS_MC_FULLSCREEN.store(true, Ordering::Relaxed);

                        if err != 0 {
                            return true
                        }
                    } else {
                        return true 
                    }
                } else {
                    return true
                }
            } else {
                patcher::IS_MC_FULLSCREEN.store(false, Ordering::Relaxed);
            }

            self.load_textures(&mut *raw_device);

            let mut msg = mem::zeroed();
    
            self.d3d = Some(&mut *d3d);
            *self.device.lock().unwrap() = Some(&mut *raw_device);

            OVERLAY.store(self, Ordering::Relaxed);

            while GetMessageW(&mut msg, wnd, 0, 0) > 0 {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }

            return false
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            if self.hwnd.is_some() {
                DestroyWindow(self.hwnd.take().unwrap());
                self.d3d.take().unwrap().Release();
                self.device.lock().unwrap().take().unwrap().Release();
    
                (*self.eng_texture.take().unwrap()).Release();
                (*self.kor_texture.take().unwrap()).Release();
            }
        }
    }

    unsafe fn load_textures(&mut self, device: &mut IDirect3DDevice9) {
        self.eng_texture = Some(Overlay::load_texture(include_bytes!("..\\assets\\EngOverlay.png"), device));
        self.kor_texture = Some(Overlay::load_texture(include_bytes!("..\\assets\\KorOverlay.png"), device));
    }

    unsafe fn load_texture(buf: &[u8], device: &mut IDirect3DDevice9) -> *mut IDirect3DTexture9 {
        use png::Decoder;

        let decoder = Decoder::new(buf);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let img = &buf[..info.buffer_size()];
        let dim = (info.width, info.height);

        let mut raw_texture = ptr::null_mut::<IDirect3DTexture9>();
        device.CreateTexture(dim.0, dim.1, 1, 0, D3DFMT_A8R8G8B8, D3DPOOL_MANAGED, &mut raw_texture, 0 as _);
        
        let mut rect = std::mem::zeroed::<D3DLOCKED_RECT>();
        let texture = &mut *raw_texture;
        
        texture.LockRect(0, &mut rect, 0 as _, D3DLOCK_DISCARD);

        ptr::copy(img.as_ptr(), rect.pBits as *mut u8, (dim.0 * dim.1 * 4) as usize);

        texture.UnlockRect(0);

        raw_texture
    }
}