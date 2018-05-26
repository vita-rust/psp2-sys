use core::fmt::Result;
use core::fmt::Write;
use core::mem::size_of;
use core::slice::from_raw_parts_mut;

use psp2::display::sceDisplaySetFrameBuf;
use psp2::display::SceDisplayFrameBuf;
use psp2::display::SceDisplaySetBufSync::SCE_DISPLAY_SETBUF_NEXTFRAME;
use psp2::kernel::sysmem::sceKernelAllocMemBlock;
use psp2::kernel::sysmem::sceKernelGetMemBlockBase;
use psp2::kernel::sysmem::SceKernelMemBlockType::SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW;
use psp2::kernel::threadmgr::sceKernelCreateMutex;
use psp2::kernel::threadmgr::sceKernelLockMutex;
use psp2::kernel::threadmgr::sceKernelUnlockMutex;
use psp2::types::SceUID;
use psp2::void;

use super::font::DEBUG_FONT;

const SCREEN_WIDTH: usize = 960;
const SCREEN_HEIGHT: usize = 544;
const SCREEN_FB_WIDTH: usize = 960;
const SCREEN_FB_SIZE: usize = 2 * 1024 * 1024;
const SCREEN_TAB_SIZE: usize = 8;
const SCREEN_TAB_W: usize = DEBUG_FONT.size_w * SCREEN_TAB_SIZE;

const DEFAULT_FG: u32 = 0xFFFFFFFF;
const DEFAULT_BG: u32 = 0xFF000000;

// #[inline(always)]
// const fn FROM_GREY(c: u32) -> u32 {
//     ((((c)*9)    <<16)  |  (((c)*9)       <<8)  | ((c)*9))
// }
// #define FROM_GREY(c     ) ((((c)*9)    <<16)  |  (((c)*9)       <<8)  | ((c)*9))
// #define FROM_3BIT(c,dark) (((!!((c)&4))<<23)  | ((!!((c)&2))<<15)     | ((!!((c)&1))<<7) | (dark ? 0 : 0x7F7F7F))
// #define FROM_6BIT(c     ) ((((c)%6)*(51<text<16)) | ((((c)/6)%6)*(51<<8)) | ((((c)/36)%6)*51))
// #define FROM_FULL(r,g,b ) ((r<<16) | (g<<8) | (b))
// #define CLEARSCRN(

// macro_rules! FROM_GREY {
//     ($c: expr) => {
//         (((($c)*9)    <<16)  |  ((($c)*9)       <<8)  | (($c)*9))
//     }
// }
//
// macro_rules! FROM_3BIT {
//     ($c: expr, $dark: expr) => {
//         ((!!(($c)&4))<<23) | ((!!(($c)&2))<<15) | ((!!(($c)&1))<<7) | if $dark { 0 } else { 0x7F7F7F }
//     }
// }
//
// macro_rules! FROM_6BIT {
//     ($c: expr) => {
//         (((($c)%6)*(51<<16)) | (((($c)/6)%6)*(51<<8)) | (((($c)/36)%6)*51))
//     }
// }
//
// macro_rules! FROM_FULL {
//     ($r: expr, $g: expr, $b: expr) => {
//         (($r<<16) | ($g<<8) | ($b))
//     }
// }

pub struct DebugScreen<'a> {
    base: &'a mut [u32],
    mutex: SceUID,
    coord_x: usize,
    coord_y: usize,
    saved_x: usize,
    saved_y: usize,
    color_fg: u32,
    color_bg: u32,
}

impl<'a> Write for DebugScreen<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        self.puts(s.as_bytes());
        Ok(())
    }
}

impl<'a> DebugScreen<'a> {
    pub fn new() -> Self {
        unsafe {
            let mut base: *mut void = ::core::ptr::null_mut();

            // Create the global screen mutex
            let mutex: SceUID =
                sceKernelCreateMutex(b"_debug_mutex\0".as_ptr(), 0, 0, ::core::ptr::null_mut());

            // Allocate memory to use as display buffer
            let displayblock: SceUID = sceKernelAllocMemBlock(
                b"display\0".as_ptr(),
                SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW,
                SCREEN_FB_SIZE as i32,
                ::core::ptr::null_mut(),
            );
            sceKernelGetMemBlockBase(displayblock, &mut base);

            // Set the display frame using the allowated memory
            let frame = SceDisplayFrameBuf {
                size: size_of::<SceDisplayFrameBuf>() as u32,
                base,
                pitch: SCREEN_FB_WIDTH as u32,
                pixelformat: 0,
                width: SCREEN_WIDTH as u32,
                height: SCREEN_HEIGHT as u32,
            };
            sceDisplaySetFrameBuf(&frame, SCE_DISPLAY_SETBUF_NEXTFRAME);

            Self {
                base: from_raw_parts_mut(base as *mut u32, SCREEN_FB_SIZE as usize / 4),
                mutex,
                coord_x: 0,
                coord_y: 0,
                saved_x: 0,
                saved_y: 0,
                color_fg: DEFAULT_FG,
                color_bg: DEFAULT_BG,
            }
        }
    }

    // unsafe fn escape(&mut self, text: &[u8]) -> usize {
    //
    //     let mut argc = 0;
    //     let mut arg = [0u32; 32];
    //
    //     for i in 0..arg.len() {
    //         match text[i] {
    //             0 => {
    //                 break;
    //             }
    //             b'0'...b'9' => {
    //                 arg[argc] = arg[argc]*10 + (text[i] - b'0') as u32;
    //             }
    //             b';' => {
    //                 argc += 1;
    //             }
    //             b's' => {
    //                 self.saved_x = self.coord_x;
    //                 self.saved_y = self.coord_y;
    //                 return i;
    //             }
    //             b'u' => {
    //                 self.coord_x = self.saved_x;
    //                 self.coord_y = self.saved_y;
    //                 return i;
    //             }
    //             b'A' => {
    //                 self.coord_y -= arg[0] as i32 * DEBUG_FONT.size_h as i32;
    //                 return i;
    //             }
    //             b'B' => {
    //                 self.coord_y += arg[0] as i32 * DEBUG_FONT.size_h as i32;
    //                 return i;
    //             }
    //             b'C' => {
    //                 self.coord_x += arg[0] as i32 * DEBUG_FONT.size_w as i32;
    //                 return i;
    //             }
    //             b'D' => {
    //                 self.coord_x -= arg[0] as i32 * DEBUG_FONT.size_w as i32;
    //                 return i;
    //             }
    //             b'E' => {
    //                 self.coord_y += arg[0] as i32 * DEBUG_FONT.size_h as i32;
    //                 self.coord_x = 0;
    //                 return i;
    //             }
    //             b'F' => {
    //                 self.coord_y -= arg[0] as i32 * DEBUG_FONT.size_h as i32;
    //                 self.coord_x = 0;
    //                 return i;
    //             }
    //             b'G' => {
    //                 self.coord_x = (arg[0] - 1) as i32 * DEBUG_FONT.size_w as i32;
    //                 return i;
    //             }
    //             b'H' | b'f' => {
    //                 self.coord_y = (arg[0] - 1) as i32 * DEBUG_FONT.size_h as i32;
    //                 self.coord_y = (arg[1] - 1) as i32 * DEBUG_FONT.size_w as i32;
    //                 return i;
    //             }
    //             b'J' | b'K' => {
    //                 match arg[0] {
    //                     0 => self.clear(self.coord_y as usize, (self.coord_y + DEBUG_FONT.size_h) as usize, self.coord_x as usize, SCREEN_WIDTH as usize),
    //                     1 => self.clear(self.coord_y as usize, (self.coord_y + DEBUG_FONT.size_h) as usize, 0, self.coord_x as usize),
    //                     2 => self.clear(self.coord_y as usize, (self.coord_y + DEBUG_FONT.size_h) as usize, 0, SCREEN_WIDTH as usize),
    //                     _ => (),
    //                 };
    //                 if text[i] == b'J' {
    //                     match arg[0] {
    //                         0 => self.clear(self.coord_y as usize, SCREEN_HEIGHT as usize, 0, SCREEN_WIDTH as usize),
    //                         1 => self.clear(0, self.coord_y as usize, 0, SCREEN_WIDTH as usize),
    //                         2 => self.clear(0, SCREEN_HEIGHT as usize, 0, SCREEN_WIDTH as usize),
    //                         _ => (),
    //                     }
    //                 }
    //         		return i;
    //             }
    //             b'm' => {
    //
    //                 if arg[0] == 0 {
    //                     arg[0] = 39;
    //                     arg[1] = 49;
    //                     argc = 1;
    //                 }
    //
    //                 let mut c = 0;
    //                 let mut unit: u32;
    //                 let mut mode: u32;
    //                 let mut color: *mut u32;
    //
    //                 while c <= argc + 1 {
    //
    //                     unit = arg[c] % 10;
    //                     mode = arg[c] / 10;
    //
    //                     if (mode & 1) == 1 {
    //                         color = (&mut self.color_fg) as *mut u32
    //                     } else {
    //                         color = (&mut self.color_bg) as *mut u32
    //                     }
    //
    //                     if arg[c] == 1 { self.color_fg |= 0x808080 };
    //                     if arg[c] == 2 { self.color_fg &= 0x7F7F7F };
    //
    //                     if mode != 3 && mode != 4 && mode != 9 && mode != 10 {
    //                         continue;
    //                     }
    //
    //                     if unit == 9 { // reset FG or BG
    //                         *color = if (mode & 1) == 1 { DEFAULT_FG } else { DEFAULT_BG };
    //                     } else if unit == 8 && arg[c+1] == 5 { // 8bit : [0-15][16-231][232-256] color map
    //                         c += 2;
    //                         if arg[c]<=15 { *color = FROM_3BIT!(arg[c],mode<9) }
    //                         else if arg[c]>=232 { *color = FROM_GREY!(arg[c]-232) }
    //                         else { *color = FROM_6BIT!(arg[c]-16) }
    //                     } else if unit == 8 && arg[c+1] == 2 { // 24b color space
    //                         c += 4;
    //                         *color = FROM_FULL!(arg[c+4], arg[c+3], arg[c+2]);
    //                     } else {  // standard 8+8 colors
    //                         *color = FROM_3BIT!(unit, mode < 9);
    //                     }
    //
    //     			}
    //
    //     			return i;
    //             }
    //             _ => (),
    //         }
    //     }
    //
    //     0
    // }

    unsafe fn clear(&mut self, from_h: usize, to_h: usize, from_w: usize, to_w: usize) {
        for h in from_h..to_h {
            for w in from_w..to_w {
                self.base[h * SCREEN_FB_WIDTH + w] = self.color_bg;
            }
        }
    }

    fn lock(&mut self) {
        unsafe {
            sceKernelLockMutex(self.mutex, 1, ::core::ptr::null_mut());
        }
    }

    fn unlock(&mut self) {
        unsafe {
            sceKernelUnlockMutex(self.mutex, 1);
        }
    }

    fn puts(&mut self, text: &[u8]) {
        let bytes_per_glyph = DEBUG_FONT.width * DEBUG_FONT.height / 8;
        self.lock();

        for &chr in text.iter() {
            if chr == b'\t' {
                self.coord_x += SCREEN_TAB_W - (self.coord_x % SCREEN_TAB_W);
                continue;
            }

            if (self.coord_x + DEBUG_FONT.width > SCREEN_WIDTH) {
                self.coord_y += DEBUG_FONT.size_h;
                self.coord_x = 0;
            }

            if (self.coord_y + DEBUG_FONT.height > SCREEN_HEIGHT) {
                self.coord_x = 0;
                self.coord_y = 0;
            }

            if chr == b'\n' {
                self.coord_x = 0;
                self.coord_y += DEBUG_FONT.size_h;
                continue;
            } else if chr == b'\r' {
                self.coord_x = 0;
                continue;
            } // else if chr == b'\x1b' && text[c] == b'[' {
              //     c += self.escape(&text[c+1..]) + 2;
              //     if self.coord_x < 0 {
              //         self.coord_x = 0
              //     }
              //     if self.coord_y < 0 {
              //         self.coord_y = 0;
              //     }
              //     continue;
              // }

            let mut vram = &mut self.base[self.coord_x + self.coord_y * SCREEN_FB_WIDTH..];
            let mut font =
                &DEBUG_FONT.glyphs[(chr - DEBUG_FONT.first) as usize * bytes_per_glyph..];
            let mut mask = 1 << 7;

            for row in 0..DEBUG_FONT.height {
                for col in 0..DEBUG_FONT.width {
                    if mask == 0 {
                        font = &font[1..];
                        mask = 1 << 7;
                    }

                    vram[row * SCREEN_FB_WIDTH + col] = if (font[0] & mask == 0) {
                        self.color_bg
                    } else {
                        self.color_fg
                    };

                    mask >>= 1;
                }

                for col in DEBUG_FONT.width..DEBUG_FONT.size_w {
                    vram[row * SCREEN_FB_WIDTH + col] = self.color_bg
                }
            }

            for row in DEBUG_FONT.height..DEBUG_FONT.size_h {
                for col in 0..DEBUG_FONT.size_w {
                    vram[row * SCREEN_FB_WIDTH + col] = self.color_bg
                }
            }

            self.coord_x += DEBUG_FONT.size_w;
        }

        self.unlock();
    }
}
