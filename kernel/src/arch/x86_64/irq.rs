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

//! x86_64 interrupt handling

use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IrqNumber(u32);

impl IrqNumber {
    pub const fn new(num: u32) -> Self {
        Self(num)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for IrqNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// x86_64 specific interrupt handlers
pub fn init_interrupts() {
    // TODO: Initialize IDT (Interrupt Descriptor Table)
}

pub fn handle_interrupt(_irq: IrqNumber) {
    // TODO: Implement interrupt handling
}