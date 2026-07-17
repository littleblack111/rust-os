#![no_std]
#![no_main]
#![feature(used_with_arg)]

use core::panic::PanicInfo;
use font8x8::UnicodeFonts;
use limine::{BaseRevision, request::FramebufferRequest};

#[used(linker)]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used(linker)]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if !BASE_REVISION.is_supported() {
        panic!("Limine Base Revision not supported");
    }

    if let Some(response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(fb) = response
            .framebuffers()
            .first()
        {
            let pitch = fb.pitch as usize;
            let bpp = fb.bpp as usize;

            if bpp == 32 {
                let slice = unsafe { fb.as_slice_mut() };
                let msg = "Hello World!";
                let mut cursor_x = 10;
                let cursor_y = 10;

                for c in msg.chars() {
                    if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
                        for (y, row) in glyph
                            .iter()
                            .enumerate()
                        {
                            for x in 0..8 {
                                if (*row & (1 << x)) != 0 {
                                    let pixel_offset = (cursor_y + y) * pitch + (cursor_x + x) * 4;
                                    if pixel_offset + 3 < slice.len() {
                                        slice[pixel_offset] = 0xFF; // B
                                        slice[pixel_offset + 1] = 0xFF; // G
                                        slice[pixel_offset + 2] = 0xFF; // R
                                    }
                                }
                            }
                        }
                    }
                    cursor_x += 8;
                }
            }
        }
    }

    hcf();
}

fn hcf() -> ! {
    unsafe {
        core::arch::asm!("cli");
        loop {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf()
}
