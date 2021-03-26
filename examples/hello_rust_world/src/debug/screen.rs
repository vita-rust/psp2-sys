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
            }

            let vram = &mut self.base[self.coord_x + self.coord_y * SCREEN_FB_WIDTH..];
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
