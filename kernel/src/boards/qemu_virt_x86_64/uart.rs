// Copyright (c) 2025 vivo Mobile Communication Co., Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! UART driver for x86_64 (16550 compatible UART)

use super::config::*;
use core::fmt;

pub struct Uart {
    base: u64,
}

impl Uart {
    pub fn new(base: u64) -> Self {
        Self { base }
    }

    fn write_byte(&self, byte: u8) {
        unsafe {
            // Wait for transmit holding register empty
            loop {
                let mut status: u8;
                core::arch::asm!(
                    "in al, dx",
                    out("al") status,
                    in("dx") (self.base + 5) as u16, // Line Status Register
                    options(nostack, preserves_flags)
                );
                if (status & 0x20) != 0 {
                    break;
                }
            }
            
            // Write to the UART data register
            core::arch::asm!(
                "out dx, al",
                in("dx") self.base as u16,
                in("al") byte,
                options(nostack, preserves_flags)
            );
        }
    }

    fn read_byte(&self) -> u8 {
        unsafe {
            let mut byte: u8;
            core::arch::asm!(
                "in al, dx",
                out("al") byte,
                in("dx") self.base as u16,
                options(nostack, preserves_flags)
            );
            byte
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

static mut EARLY_UART: Option<Uart> = None;

pub fn get_early_uart() -> &'static mut dyn fmt::Write {
    unsafe {
        if EARLY_UART.is_none() {
            EARLY_UART = Some(Uart::new(UART0_BASE));
        }
        EARLY_UART.as_mut().unwrap()
    }
}